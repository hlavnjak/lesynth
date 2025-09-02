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

#[derive(Debug, Clone, Copy, PartialEq, Enum)]
pub enum CurveType {
    Constant,
    Sine,
}

impl CurveType {
    // so we can write `for variant in CurveType::VARIANTS`
    pub const VARIANTS: [CurveType; 2] = [
        CurveType::Constant,
        CurveType::Sine,
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curve_type_variants() {
        assert_eq!(CurveType::VARIANTS.len(), 2);
        assert_eq!(CurveType::VARIANTS[0], CurveType::Constant);
        assert_eq!(CurveType::VARIANTS[1], CurveType::Sine);
    }

    #[test]
    fn test_curve_type_debug() {
        assert_eq!(format!("{:?}", CurveType::Constant), "Constant");
        assert_eq!(format!("{:?}", CurveType::Sine), "Sine");
    }

    #[test]
    fn test_curve_type_clone() {
        let original = CurveType::Sine;
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_curve_type_equality() {
        assert_eq!(CurveType::Constant, CurveType::Constant);
        assert_ne!(CurveType::Constant, CurveType::Sine);
        assert_eq!(CurveType::Sine, CurveType::Sine);
    }
}