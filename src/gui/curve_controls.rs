// Copyright 2025 Jakub Hlavnicka
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Arc;
use nih_plug::prelude::ParamSetter;
use crate::constants::*;
use crate::engine::{ChartType, SynthComputeEngine};
use crate::params::{CurveType, HarmonicParam};

pub fn draw_curve_controls(
    ui: &mut nih_plug_egui::egui::Ui,
    idx: usize,
    chart_type: ChartType,
    harmonic: &HarmonicParam,
    synth_compute_engine: Arc<SynthComputeEngine>,
    setter: &ParamSetter,
    params_changed_action: &dyn Fn(),
    offset_min: f64,
    offset_max: f64,
    sine_amp_min: f64,
    sine_amp_max: f64,
) {
    ui.label(format!("{:?}:", chart_type));
    ui.columns(5, |cols| {
        let (offset, a, b, curve) = match chart_type {
            ChartType::Amp => (
                &harmonic.curve_offset_amp,
                &harmonic.sine_curve_amp_amp,
                &harmonic.sine_curve_freq_amp,
                &harmonic.curve_type_amp,
            ),
            ChartType::Phase => (
                &harmonic.curve_offset_phase,
                &harmonic.sine_curve_amp_phase,
                &harmonic.sine_curve_freq_phase,
                &harmonic.curve_type_phase,
            ),
        };

        // Column 0: Offset
        {
            let param = offset;
            let engine = synth_compute_engine.clone();
            let chart_type_clone = chart_type.clone();
            let slider = nih_plug_egui::egui::Slider::from_get_set(offset_min..=offset_max, move |new_val| {
                if let Some(v) = new_val {
                    setter.begin_set_parameter(param);
                    setter.set_parameter(param, v as f32);
                    setter.end_set_parameter(param);
                    v
                } else {
                    param.value() as f64
                }
            })
            .suffix(" Offset");

            let response = cols[0].add(slider);
            if response.drag_stopped() {
                match curve.value() {
                    CurveType::Sine => engine.fill_sin_curve(idx, chart_type_clone.clone()),
                    CurveType::Constant => engine.fill_constant_curve(idx, offset.value(), chart_type_clone.clone()),
                }
                params_changed_action();
            }
        }

        // Column 1: Sine Amp
        {
            let param = a;
            let engine = synth_compute_engine.clone();
            let chart_type_clone = chart_type.clone();
            let slider = nih_plug_egui::egui::Slider::from_get_set(sine_amp_min..=sine_amp_max, move |new_val| {
                if let Some(v) = new_val {
                    setter.begin_set_parameter(param);
                    setter.set_parameter(param, v as f32);
                    setter.end_set_parameter(param);
                    v
                } else {
                    param.value() as f64
                }
            })
            .suffix(" Sine Amp.");

            let response = cols[1].add(slider);
            if response.drag_stopped() {
                if curve.value() == CurveType::Sine {
                    engine.fill_sin_curve(idx, chart_type_clone.clone());
                }
                params_changed_action();
            }
        }

        // Column 2: Sine Freq
        {
            let param = b;
            let engine = synth_compute_engine.clone();
            let chart_type_clone = chart_type.clone();
            let slider = nih_plug_egui::egui::Slider::from_get_set(MIN_SINE_FREQ..=MAX_SINE_FREQ, move |new_val| {
                if let Some(vf) = new_val {
                    setter.begin_set_parameter(param);
                    setter.set_parameter(param, vf as f32);
                    setter.end_set_parameter(param);
                    vf as f64
                } else {
                    param.value() as f64
                }
            })
            .suffix(" Sine Freq.");

            let response = cols[2].add(slider);
            if response.drag_stopped() {
                if curve.value() == CurveType::Sine {
                    engine.fill_sin_curve(idx, chart_type_clone.clone());
                }
                params_changed_action();
            }
        }

        // Column 3: Curve Type Combo
        {
            let combo_id = format!("{:?}_curve_type_combo_{}", chart_type, idx);
            nih_plug_egui::egui::ComboBox::from_id_salt(combo_id)
                .selected_text(format!("{:?}", curve.value()))
                .show_ui(&mut cols[3], |ui| {
                    for &variant in CurveType::VARIANTS.iter() {
                        if ui
                            .selectable_label(
                                curve.value() == variant,
                                format!("{:?}", variant),
                            )
                            .clicked()
                        {
                            setter.begin_set_parameter(curve);
                            setter.set_parameter(curve, variant);
                            setter.end_set_parameter(curve);
                            match variant {
                                CurveType::Sine => {
                                    synth_compute_engine.fill_sin_curve(idx, chart_type.clone());
                                }
                                CurveType::Constant => {
                                    let offset_value = match chart_type {
                                        ChartType::Amp => offset.value(),
                                        ChartType::Phase => offset.value(),
                                    };
                                    synth_compute_engine.fill_constant_curve(idx, offset_value, chart_type.clone());
                                }
                            }
                            params_changed_action();
                        }
                    }
                });
        }
        // Column 4: Enable Checkbox
        {
            // We need to let the lock out of the scope in order to avoid deadlock when calling params_changed_action
            let new_enabled = {
                let mut enabled = match chart_type {
                    ChartType::Amp => synth_compute_engine
                        .shared_params
                        .harmonic_ampl_enabled
                        .lock()
                        .unwrap(),
                    ChartType::Phase => synth_compute_engine
                        .shared_params
                        .harmonic_phase_enabled
                        .lock()
                        .unwrap(),
                };
                let checkbox = cols[4].checkbox(&mut enabled[idx], "Enabled");
                if checkbox.changed() {
                    let new_val = enabled[idx];
                    Some(new_val)
                } else {
                    None
                }
            };

            if let Some(_val) = new_enabled {
                // Mark all buffers as dirty since enabled state affects audio generation
                synth_compute_engine.shared_params.mark_all_buffers_dirty();
                // Update assembled chart immediately to reflect the enable state change
                synth_compute_engine.update_assembled_chart_with_key24();
                params_changed_action();
            }
        }
    });
}