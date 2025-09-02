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

use nih_plug::prelude::*;
use super::CurveType;

/// A single harmonic's complete parameter set.
/// - amp:     amplitude multiplier
/// - phase:   phase offset multiplier
/// - curve:   which envelope/curve to use
/// - a, b:    extra curve parameters
#[derive(Params)]
pub struct HarmonicParam {
    #[id = "curve_offset_amp"]
    pub curve_offset_amp: FloatParam,
    #[id = "curve_offset_phase"]
    pub curve_offset_phase: FloatParam,
    #[id = "curve_type_amp"]
    pub curve_type_amp: EnumParam<CurveType>,
    #[id = "curve_type_phase"]
    pub curve_type_phase: EnumParam<CurveType>,
    #[id = "sine_curve_amp_amp"]
    pub sine_curve_amp_amp: FloatParam,
    #[id = "sine_curve_freq_amp"]
    pub sine_curve_freq_amp: FloatParam,
    #[id = "sine_curve_amp_phase"]
    pub sine_curve_amp_phase: FloatParam,
    #[id = "sine_curve_freq_phase"]
    pub sine_curve_freq_phase: FloatParam,
}