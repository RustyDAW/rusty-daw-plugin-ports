// All parameters have an "unscaled" value and a "scaled" value.
// The scaled value is just the unscaled value with a bit of extra math.
// Typically, all parameters are set and stored unscaled, and fetched scaled.

// TODO: refactor.
//       For ease of porting, I'm going to follow the structure that the original CloudSeed has
//       these parameters in. I'm not a huge fan of that structure though, so I'll refactor later.

use crate::{multitap_diffuser, allpass_diffuser};
use crate::value_tables::{Table, ValueTables};

pub const PARAMETER_COUNT: usize = 46;

#[derive(Copy, Clone)]
pub enum ParameterName {
    // Input
    InputMix = 0,
    PreDelay = 1,
    HighPass = 2,
    LowPass = 3,
    // Early
    TapCount = 4,
    TapLength = 5,
    TapGain = 6,
    TapDecay = 7,
    DiffusionEnabled = 8,
    DiffusionStages = 9,
    DiffusionDelay = 10,
    DiffusionFeedback = 11,
    // Late
    LineCount = 12,
    LineDelay = 13,
    LineDecay = 14,
    LateDiffusionEnabled = 15,
    LateDiffusionStages = 16,
    LateDiffusionDelay = 17,
    LateDiffusionFeedback = 18,
    // Frequency response
    PostLowShelfGain = 19,
    PostLowShelfFrequency = 20,
    PostHighShelfGain = 21,
    PostHighShelfFrequency = 22,
    PostCutoffFrequency = 23,
    // Modulation
    EarlyDiffusionModAmount = 24,
    EarlyDiffusionModRate = 25,
    LineModAmount = 26,
    LineModRate = 27,
    LateDiffusionModAmount = 28,
    LateDiffusionModRate = 29,
    // Seeds
    TapSeed = 30,
    DiffusionSeed = 31,
    DelaySeed = 32,
    PostDiffusionSeed = 33,
    // Seed mix
    CrossSeed = 34,
    DryOut = 35,
    PredelayOut = 36,
    EarlyOut = 37,
    MainOut = 38,
    // Switches
    HighPassEnabled = 39,
    LowPassEnabled = 40,
    LowShelfEnabled = 41,
    HighShelfEnabled = 42,
    CutoffEnabled = 43,
    LateStageTap = 44,
    // Effects
    Interpolation = 45,
}

pub struct Parameters {
    parameters: [f64; PARAMETER_COUNT],

    // TODO: we probably want to just share this between the whole crate since it's never changed
    //       once created, but I'm not sure the best way to do that yet because I haven't seen the
    //       whole architecture of CloudSeed yet.
    value_tables: ValueTables,
}

impl Parameters {
    pub fn new() -> Self {
        let mut parameters = [0.0; 46];

        // "Small room" preset
        // TODO: This is just something to start with. Later I'll refactor this into actual presets.
        parameters[ParameterName::InputMix as usize] = 0.0;
        parameters[ParameterName::PreDelay as usize] = 0.0;
        parameters[ParameterName::HighPass as usize] = 0.0;
        parameters[ParameterName::LowPass as usize] = 0.755000114440918;
        parameters[ParameterName::TapCount as usize] = 0.41499990224838257;
        parameters[ParameterName::TapLength as usize] = 0.43999996781349182;
        parameters[ParameterName::TapGain as usize] = 0.87999999523162842;
        parameters[ParameterName::TapDecay as usize] = 1.0;
        parameters[ParameterName::DiffusionEnabled as usize] = 1.0;
        parameters[ParameterName::DiffusionStages as usize] = 0.71428573131561279;
        parameters[ParameterName::DiffusionDelay as usize] = 0.335000216960907;
        parameters[ParameterName::DiffusionFeedback as usize] = 0.660000205039978;
        parameters[ParameterName::LineCount as usize] = 0.18181818723678589;
        parameters[ParameterName::LineDelay as usize] = 0.51000016927719116;
        parameters[ParameterName::LineDecay as usize] = 0.29999998211860657;
        parameters[ParameterName::LateDiffusionEnabled as usize] = 1.0;
        parameters[ParameterName::LateDiffusionStages as usize] = 0.4285714328289032;
        parameters[ParameterName::LateDiffusionDelay as usize] = 0.22999951243400574;
        parameters[ParameterName::LateDiffusionFeedback as usize] = 0.59499990940093994;
        parameters[ParameterName::PostLowShelfGain as usize] = 0.87999987602233887;
        parameters[ParameterName::PostLowShelfFrequency as usize] = 0.19499993324279785;
        parameters[ParameterName::PostHighShelfGain as usize] = 0.875;
        parameters[ParameterName::PostHighShelfFrequency as usize] = 0.59000009298324585;
        parameters[ParameterName::PostCutoffFrequency as usize] = 0.79999983310699463;
        parameters[ParameterName::EarlyDiffusionModAmount as usize] = 0.13499999046325684;
        parameters[ParameterName::EarlyDiffusionModRate as usize] = 0.29000008106231689;
        parameters[ParameterName::LineModAmount as usize] = 0.18999995291233063;
        parameters[ParameterName::LineModRate as usize] = 0.22999987006187439;
        parameters[ParameterName::LateDiffusionModAmount as usize] = 0.1249999925494194;
        parameters[ParameterName::LateDiffusionModRate as usize] = 0.28500008583068848;
        parameters[ParameterName::TapSeed as usize] = 0.00048499999684281647;
        parameters[ParameterName::DiffusionSeed as usize] = 0.00020799999765586108;
        parameters[ParameterName::DelaySeed as usize] = 0.00033499998971819878;
        parameters[ParameterName::PostDiffusionSeed as usize] = 0.00037200000951997936;
        parameters[ParameterName::CrossSeed as usize] = 0.42500001192092896;
        parameters[ParameterName::DryOut as usize] = 1.0;
        parameters[ParameterName::PredelayOut as usize] = 0.0;
        parameters[ParameterName::EarlyOut as usize] = 0.8599998950958252;
        parameters[ParameterName::MainOut as usize] = 0.90500003099441528;
        parameters[ParameterName::HighPassEnabled as usize] = 0.0;
        parameters[ParameterName::LowPassEnabled as usize] = 1.0;
        parameters[ParameterName::LowShelfEnabled as usize] = 0.0;
        parameters[ParameterName::HighShelfEnabled as usize] = 0.0;
        parameters[ParameterName::CutoffEnabled as usize] = 0.0;
        parameters[ParameterName::LateStageTap as usize] = 1.0;
        parameters[ParameterName::Interpolation as usize] = 1.0;

        Self {
            parameters,
            value_tables: ValueTables::new(),
        }
    }

    pub fn set(&mut self, parameter_name: ParameterName, value: f64) {
        self.parameters[parameter_name as usize] = value;
    }

    // TODO: not sure if needed
    #[allow(dead_code)]
    pub fn get(&self, parameter_name: ParameterName) -> f64 {
        self.parameters[parameter_name as usize]
    }

    pub fn get_scaled(&self, parameter_name: ParameterName) -> f64 {
        let unscaled = self.parameters[parameter_name as usize];
        match parameter_name {
            ParameterName::InputMix                => unscaled,
            ParameterName::PreDelay                => (unscaled * 1000.0).trunc(),
            ParameterName::HighPass                => self.value_tables.get(Table::Response4_Oct, unscaled) * 980.0 + 20.0,
            ParameterName::LowPass                 => self.value_tables.get(Table::Response4_Oct, unscaled) * 19600.0 + 400.0,
            ParameterName::TapCount                => 1.0 + (unscaled * (multitap_diffuser::MAX_TAPS - 1) as f64).trunc(),
            ParameterName::TapLength               => (unscaled * 500.0).trunc(),
            ParameterName::TapGain                 => self.value_tables.get(Table::Response2_Dec, unscaled),
            ParameterName::TapDecay                => unscaled,
            ParameterName::DiffusionEnabled        => if unscaled < 0.5 { 0.0 } else { 1.0 },
            ParameterName::DiffusionStages         => 1.0 + (unscaled * ((allpass_diffuser::MAX_STAGE_COUNT as f64) - 0.001)).trunc(),
            ParameterName::DiffusionDelay          => (unscaled * 90.0 + 10.0).trunc(),
            ParameterName::DiffusionFeedback       => unscaled,
            ParameterName::LineCount               => (unscaled * 11.999).trunc() + 1.0,
            ParameterName::LineDelay               => (self.value_tables.get(Table::Response2_Dec, unscaled) * 980.0 + 20.0).trunc(),
            ParameterName::LineDecay               => self.value_tables.get(Table::Response3_Dec, unscaled) * 59.95 + 0.05,
            ParameterName::LateDiffusionEnabled    => if unscaled < 0.5 { 0.0 } else { 1.0 },
            ParameterName::LateDiffusionStages     => 1.0 + (unscaled * ((allpass_diffuser::MAX_STAGE_COUNT as f64) - 0.001)).trunc(),
            ParameterName::LateDiffusionDelay      => (unscaled * 90.0 + 10.0).trunc(),
            ParameterName::LateDiffusionFeedback   => unscaled,
            ParameterName::PostLowShelfGain        => self.value_tables.get(Table::Response2_Dec, unscaled),
            ParameterName::PostLowShelfFrequency   => self.value_tables.get(Table::Response4_Oct, unscaled) * 980.0 + 20.0,
            ParameterName::PostHighShelfGain       => self.value_tables.get(Table::Response2_Dec, unscaled),
            ParameterName::PostHighShelfFrequency  => self.value_tables.get(Table::Response4_Oct, unscaled) * 19600.0 + 400.0,
            ParameterName::PostCutoffFrequency     => self.value_tables.get(Table::Response4_Oct, unscaled) * 19600.0 + 400.0,
            ParameterName::EarlyDiffusionModAmount => unscaled * 2.5,
            ParameterName::EarlyDiffusionModRate   => self.value_tables.get(Table::Response2_Dec, unscaled) * 5.0,
            ParameterName::LineModAmount           => unscaled * 2.5,
            ParameterName::LineModRate             => self.value_tables.get(Table::Response2_Dec, unscaled) * 5.0,
            ParameterName::LateDiffusionModAmount  => unscaled * 2.5,
            ParameterName::LateDiffusionModRate    => self.value_tables.get(Table::Response2_Dec, unscaled) * 5.0,
            ParameterName::TapSeed                 => (unscaled * 1000000.0 + 0.001).floor(),
            ParameterName::DiffusionSeed           => (unscaled * 1000000.0 + 0.001).floor(),
            ParameterName::DelaySeed               => (unscaled * 1000000.0 + 0.001).floor(),
            ParameterName::PostDiffusionSeed       => (unscaled * 1000000.0 + 0.001).floor(),
            ParameterName::CrossSeed               => unscaled,
            ParameterName::DryOut                  => self.value_tables.get(Table::Response2_Dec, unscaled),
            ParameterName::PredelayOut             => self.value_tables.get(Table::Response2_Dec, unscaled),
            ParameterName::EarlyOut                => self.value_tables.get(Table::Response2_Dec, unscaled),
            ParameterName::MainOut                 => self.value_tables.get(Table::Response2_Dec, unscaled),
            ParameterName::HighPassEnabled         => if unscaled < 0.5 { 0.0 } else { 1.0 },
            ParameterName::LowPassEnabled          => if unscaled < 0.5 { 0.0 } else { 1.0 },
            ParameterName::LowShelfEnabled         => if unscaled < 0.5 { 0.0 } else { 1.0 },
            ParameterName::HighShelfEnabled        => if unscaled < 0.5 { 0.0 } else { 1.0 },
            ParameterName::CutoffEnabled           => if unscaled < 0.5 { 0.0 } else { 1.0 },
            ParameterName::LateStageTap            => if unscaled < 0.5 { 0.0 } else { 1.0 },
            ParameterName::Interpolation           => if unscaled < 0.5 { 0.0 } else { 1.0 },
        }
    }


}