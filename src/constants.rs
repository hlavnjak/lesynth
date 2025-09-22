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

use std::f32::consts::PI;

// Audio Constants
pub const NUM_HARMONICS: usize = 64;
pub const NUM_KEYS: usize = 88;

// Parameter Defaults and Ranges
pub static NUM_OF_BUCKETS_DEFAULT: usize = 70;
pub static NUM_OF_BUCKETS_MIN: i32 = 30;
pub static NUM_OF_BUCKETS_MAX: i32 = 2000;

// Amplitude Parameter Ranges
pub static MIN_OFFSET_AMP: f64 = 0.0;
pub static MAX_OFFSET_AMP: f64 = 1.0;
pub static MIN_AMP_SINE_AMP: f64 = 0.0;
pub static MAX_AMP_SINE_AMP: f64 = 1.0;

// Phase Parameter Ranges
pub static MIN_OFFSET_PHASE: f64 = 0.0;
pub static MAX_OFFSET_PHASE: f64 = 6.28;
pub static MIN_PHASE_SINE_AMP: f64 = 0.0;
pub static MAX_PHASE_SINE_AMP: f64 = 6.28;

// Sine Curve Frequency Ranges
pub static MIN_SINE_FREQ: f64 = 0.0;
pub static MAX_SINE_FREQ: f64 = 0.35;

// GUI Constants
pub static LABEL_FONT_SIZE: f32 = 12.0;

// Audio Processing Constants
pub const TWO_PI: f32 = 2.0 * PI;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_constants() {
        assert_eq!(NUM_HARMONICS, 32);
        assert_eq!(NUM_KEYS, 88);
    }

    #[test]
    fn test_parameter_defaults_and_ranges() {
        assert_eq!(NUM_OF_BUCKETS_DEFAULT, 70);
        assert_eq!(NUM_OF_BUCKETS_MIN, 30);
        assert_eq!(NUM_OF_BUCKETS_MAX, 2000);
        assert!(NUM_OF_BUCKETS_MIN < NUM_OF_BUCKETS_DEFAULT as i32);
        assert!(NUM_OF_BUCKETS_DEFAULT < NUM_OF_BUCKETS_MAX as usize);
    }

    #[test]
    fn test_amplitude_ranges() {
        assert_eq!(MIN_OFFSET_AMP, 0.0);
        assert_eq!(MAX_OFFSET_AMP, 1.0);
        assert!(MIN_OFFSET_AMP < MAX_OFFSET_AMP);
        
        assert_eq!(MIN_AMP_SINE_AMP, 0.0);
        assert_eq!(MAX_AMP_SINE_AMP, 1.0);
        assert!(MIN_AMP_SINE_AMP < MAX_AMP_SINE_AMP);
    }

    #[test]
    fn test_phase_ranges() {
        assert_eq!(MIN_OFFSET_PHASE, 0.0);
        assert_eq!(MAX_OFFSET_PHASE, 6.28);
        assert!(MIN_OFFSET_PHASE < MAX_OFFSET_PHASE);
        
        assert_eq!(MIN_PHASE_SINE_AMP, 0.0);
        assert_eq!(MAX_PHASE_SINE_AMP, 6.28);
        assert!(MIN_PHASE_SINE_AMP < MAX_PHASE_SINE_AMP);
        
        // Should be approximately 2*PI
        assert!((MAX_OFFSET_PHASE - 2.0 * PI as f64).abs() < 0.01);
        assert!((MAX_PHASE_SINE_AMP - 2.0 * PI as f64).abs() < 0.01);
    }

    #[test]
    fn test_sine_frequency_ranges() {
        assert_eq!(MIN_SINE_FREQ, 0.0);
        assert_eq!(MAX_SINE_FREQ, 0.35);
        assert!(MIN_SINE_FREQ < MAX_SINE_FREQ);
    }

    #[test]
    fn test_two_pi_constant() {
        assert_eq!(TWO_PI, 2.0 * PI);
        assert!((TWO_PI - 6.28318530717959).abs() < 1e-6);
    }

    #[test]
    fn test_gui_constants() {
        assert_eq!(LABEL_FONT_SIZE, 12.0);
        assert!(LABEL_FONT_SIZE > 0.0);
    }
}
