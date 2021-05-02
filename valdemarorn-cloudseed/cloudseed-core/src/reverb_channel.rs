use crate::BUFFER_SIZE;
use crate::parameters::{PARAMETER_COUNT, ParameterName};
use crate::modulated_delay::ModulatedDelay;
use crate::multitap_diffuser::MultitapDiffuser;
use crate::allpass_diffuser::AllpassDiffuser;
use crate::delay_line::DelayLine;
use crate::sha256_random;
use crate::high_pass_1::HighPass1;
use crate::low_pass_1::LowPass1;
use crate::utils;

const TOTAL_LINE_COUNT: usize = 12;

#[derive(PartialEq)]
pub enum ChannelLr {
    Left,
    Right,
}

pub struct ReverbChannel {
    sample_rate: u32,
    channel_lr: ChannelLr,
    pre_delay: ModulatedDelay,
    multitap_diffuser: MultitapDiffuser,
    allpass_diffuser: AllpassDiffuser,
    delay_lines: [DelayLine; TOTAL_LINE_COUNT],
    // random: ShaRandom, // NOTE: doesn't need to be a class, functions are fine
    high_pass_filter: HighPass1,
    low_pass_filter: LowPass1,
    temp_buffer: [f64; BUFFER_SIZE],
    line_out_buffer: [f64; BUFFER_SIZE],
    out_buffer: [f64; BUFFER_SIZE],
    delay_line_seed: i32,
    post_diffusion_seed: i32,
    line_count: usize,
    high_pass_enabled: bool,
    low_pass_enabled: bool,
    diffuser_enabled: bool,
    dry_out: f64,
    predelay_out: f64,
    early_out: f64,
    line_out: f64,
    cross_seed: f64,
    internal_parameters: [f64; PARAMETER_COUNT], // TODO: this is dumb, remove it.
}

impl ReverbChannel {
    pub fn new(sample_rate: u32, channel_lr: ChannelLr) -> Self {
        // delay_buffer_length_ms = 150 to allow for 100ms + modulation time
        let mut allpass_diffuser = AllpassDiffuser::new(sample_rate, 150);
        allpass_diffuser.set_interpolation_enabled(true);

        let mut high_pass_filter = HighPass1::new(sample_rate);
        high_pass_filter.set_cutoff_hz(20.0);

        let mut low_pass_filter = LowPass1::new(sample_rate);
        low_pass_filter.set_cutoff_hz(20_000.0);

        // arr_macro::arr doesn't like consts (grumble grumble grumble)
        // could probably just use a Vec here.
        let delay_lines = [
            DelayLine::new(sample_rate), DelayLine::new(sample_rate), DelayLine::new(sample_rate),
            DelayLine::new(sample_rate), DelayLine::new(sample_rate), DelayLine::new(sample_rate),
            DelayLine::new(sample_rate), DelayLine::new(sample_rate), DelayLine::new(sample_rate),
            DelayLine::new(sample_rate), DelayLine::new(sample_rate), DelayLine::new(sample_rate),
        ];

        Self {
            sample_rate,
            channel_lr,
            pre_delay: ModulatedDelay::new(sample_rate, 100), // 1s delay buffer
            multitap_diffuser: MultitapDiffuser::new(sample_rate), // 1s delay buffer
            allpass_diffuser,
            delay_lines,
            high_pass_filter,
            low_pass_filter,
            temp_buffer: [0.0; BUFFER_SIZE],
            line_out_buffer: [0.0; BUFFER_SIZE],
            out_buffer: [0.0; BUFFER_SIZE],
            delay_line_seed: 0, // NOTE: unspecified
            post_diffusion_seed: 0, // NOTE: unspecified
            line_count: 8,
            high_pass_enabled: false, // NOTE: unspecified
            low_pass_enabled: false, // NOTE: unspecified
            diffuser_enabled: false, // NOTE: unspecified
            dry_out: 0.0, // NOTE: unspecified
            predelay_out: 0.0, // NOTE: unspecified
            early_out: 0.0, // NOTE: unspecified
            line_out: 0.0, // NOTE: unspecified
            cross_seed: 0.0,
            internal_parameters: [0.0; PARAMETER_COUNT],
        }
    }

    #[allow(dead_code)]
    pub fn get_sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn set_sample_rate(&mut self, sample_rate: u32) {
        self.sample_rate = sample_rate;
        self.high_pass_filter.set_sample_rate(sample_rate);
        self.low_pass_filter.set_sample_rate(sample_rate);

        for i in 0..TOTAL_LINE_COUNT {
            self.delay_lines[i].set_sample_rate(sample_rate);
        }

        // TODO: This is super dumb, refactor it.
        let mut update = |parameter_name: ParameterName| {
            self.set_parameter(parameter_name, self.internal_parameters[parameter_name as usize])
        };
        update(ParameterName::PreDelay);
        update(ParameterName::TapLength);
        update(ParameterName::DiffusionDelay);
        update(ParameterName::LineDelay);
        update(ParameterName::LateDiffusionDelay);
        update(ParameterName::EarlyDiffusionModRate);
        update(ParameterName::LineModRate);
        update(ParameterName::LateDiffusionModRate);
        update(ParameterName::LineModAmount);

        self.update_lines();
    }

    pub fn get_output(&self, sample: usize) -> f64 {
        self.out_buffer[sample]
    }

    // NOTE: not sure if needed
    #[allow(dead_code)]
    pub fn get_line_output(&self, sample: usize) -> f64 {
        self.line_out_buffer[sample]
    }

    pub fn set_parameter(&mut self, parameter_name: ParameterName, value: f64) {
        self.internal_parameters[parameter_name as usize] = value;

        match parameter_name {
            ParameterName::InputMix => {}, // nothing to do here
            ParameterName::PreDelay => {
                self.pre_delay.set_sample_delay(
                    milliseconds_to_samples(value, self.sample_rate) as u32
                );
            },
            ParameterName::HighPass => {
                self.high_pass_filter.set_cutoff_hz(value);
            },
            ParameterName::LowPass => {
                self.low_pass_filter.set_cutoff_hz(value);
            },
            ParameterName::TapCount => {
                self.multitap_diffuser.set_tap_count(value as i32);
            },
            ParameterName::TapLength => {
                self.multitap_diffuser.set_tap_length(
                    milliseconds_to_samples(value, self.sample_rate) as i32
                );
            },
            ParameterName::TapGain => {
                self.multitap_diffuser.set_tap_gain(value);
            },
            ParameterName::TapDecay => {
                self.multitap_diffuser.set_tap_decay(value);
            },
            ParameterName::DiffusionEnabled => {
                let new_diffuser_enabled = value >= 0.5;
                if new_diffuser_enabled != self.diffuser_enabled {
                    self.allpass_diffuser.clear_buffers();
                }
                self.diffuser_enabled = new_diffuser_enabled;
            },
            ParameterName::DiffusionStages => {
                self.allpass_diffuser.set_stages(value as i32);
            },
            ParameterName::DiffusionDelay => {
                self.allpass_diffuser.set_delay(
                    milliseconds_to_samples(value, self.sample_rate) as usize
                );
            },
            ParameterName::DiffusionFeedback => {
                self.allpass_diffuser.set_feedback(value);
            },
            ParameterName::LineCount => {
                self.line_count = value as usize;
            },
            ParameterName::LineDelay => {
                self.update_lines();
            },
            ParameterName::LineDecay => {
                self.update_lines();
            },
            ParameterName::LateDiffusionEnabled => {
                for line in &mut self.delay_lines {
                    let new_val = value >= 0.5;
                    if new_val != line.get_diffuser_enabled() {
                        line.clear_diffuser_buffer();
                    }
                    line.set_diffuser_enabled(new_val);
                }
            },
            ParameterName::LateDiffusionStages => {
                for line in &mut self.delay_lines {
                    line.set_diffuser_stages(value as i32);
                }
            },
            ParameterName::LateDiffusionDelay => {
                for line in &mut self.delay_lines {
                    line.set_diffuser_delay(
                        milliseconds_to_samples(value, self.sample_rate) as i32
                    );
                }
            },
            ParameterName::LateDiffusionFeedback => {
                for line in &mut self.delay_lines {
                    line.set_diffuser_feedback(value);
                }
            },
            ParameterName::PostLowShelfGain => {
                for line in &mut self.delay_lines {
                    line.set_low_shelf_gain(value);
                }
            },
            ParameterName::PostLowShelfFrequency => {
                for line in &mut self.delay_lines {
                    line.set_low_shelf_frequency(value);
                }
            },
            ParameterName::PostHighShelfGain => {
                for line in &mut self.delay_lines {
                    line.set_high_shelf_gain(value);
                }
            },
            ParameterName::PostHighShelfFrequency => {
                for line in &mut self.delay_lines {
                    line.set_high_shelf_frequency(value);
                }
            },
            ParameterName::PostCutoffFrequency => {
                for line in &mut self.delay_lines {
                    line.set_cutoff_frequency(value);
                }
            },
            ParameterName::EarlyDiffusionModAmount => {
                self.allpass_diffuser.set_modulation_enabled(value > 0.0);
                self.allpass_diffuser.set_modulation_amount(
                    milliseconds_to_samples(value, self.sample_rate)
                );
            },
            ParameterName::EarlyDiffusionModRate => {
                self.allpass_diffuser.set_modulation_rate(value);
            },
            ParameterName::LineModAmount => {
                self.update_lines();
            },
            ParameterName::LineModRate => {
                self.update_lines();
            },
            ParameterName::LateDiffusionModAmount => {
                self.update_lines();
            },
            ParameterName::LateDiffusionModRate => {
                self.update_lines();
            },
            ParameterName::TapSeed => {
                self.multitap_diffuser.set_seed(value as i32);
            },
            ParameterName::DiffusionSeed => {
                self.allpass_diffuser.set_seed(value as i32);
            },
            ParameterName::DelaySeed => {
                self.delay_line_seed = value as i32;
                self.update_lines();
            },
            ParameterName::PostDiffusionSeed => {
                self.post_diffusion_seed = value as i32;
                self.update_post_diffusion();
            },
            ParameterName::CrossSeed => {
                self.cross_seed = if self.channel_lr == ChannelLr::Right {
                    value
                } else {
                    0.0
                };

                self.multitap_diffuser.set_cross_seed(value);
                self.allpass_diffuser.set_cross_seed(value);
                self.update_lines();
                self.update_post_diffusion();
            },
            ParameterName::DryOut => {
                self.dry_out = value;
            },
            ParameterName::PredelayOut => {
                self.predelay_out = value;
            },
            ParameterName::EarlyOut => {
                self.early_out = value;
            },
            ParameterName::MainOut => {
                self.line_out = value;
            },
            ParameterName::HighPassEnabled => {
                self.high_pass_enabled = value >= 0.5;
            },
            ParameterName::LowPassEnabled => {
                self.low_pass_enabled = value >= 0.5;
            },
            ParameterName::LowShelfEnabled => {
                for line in &mut self.delay_lines {
                    line.set_low_shelf_enabled(value >= 0.5);
                }
            },
            ParameterName::HighShelfEnabled => {
                for line in &mut self.delay_lines {
                    line.set_high_shelf_enabled(value >= 0.5);
                }
            },
            ParameterName::CutoffEnabled => {
                for line in &mut self.delay_lines {
                    line.set_cutoff_enabled(value >= 0.5);
                }
            },
            ParameterName::LateStageTap => {
                for line in &mut self.delay_lines {
                    line.set_late_stage_tap(value >= 0.5);
                }
            },
            ParameterName::Interpolation => {
                for line in &mut self.delay_lines {
                    line.set_interpolation_enabled(value >= 0.5);
                }
            },
        }
    }

    // TODO: Lots of allocations and buffer copying here.
    //       Definitely want to look into working with slices/references instead.
    //       Yes, this Rust code is bad. I'll fix it once I've ported everything else and tested
    //       for parity.
    pub fn process(&mut self, input: &[f64], sample_count: usize) {
        // TODO: allocation!
        let mut pre_delay_output = vec![0.0f64; sample_count];
        for s in 0..sample_count {
            pre_delay_output[s] = self.pre_delay.get_output(s);
        }
        // TODO: I'd implement this a bit differently for clarity.
        //       input signal --> HPF --> LPF --> (rest) --> output signal

        // TODO: allocation!
        let mut low_pass_input = vec![0.0; sample_count];
        if self.high_pass_enabled {
            for s in 0..sample_count {
                low_pass_input[s] = self.temp_buffer[s];
            }
        } else {
            for s in 0..sample_count {
                low_pass_input[s] = input[s];
            }
        }

        if self.high_pass_enabled {
            self.high_pass_filter.process_buffer(input, &mut self.temp_buffer[..], sample_count);
        }
        if self.low_pass_enabled {
            self.low_pass_filter.process_buffer(&low_pass_input, &mut self.temp_buffer[..], sample_count);
        }
        if (!self.low_pass_enabled) && (!self.high_pass_enabled) {
            // TODO: probably a proper memcpy function around here somewhere
            for s in 0..sample_count {
                self.temp_buffer[s] = input[s]
            }
        }

        // Completely zero out small values
        // NOTE: this was done in the original CloudSeed due to some weird CPU spikes, it may not
        //       be needed here. Porting over for parity.
        for s in 0..sample_count {
            let n = self.temp_buffer[s];
            if n*n < 0.000000001 {
                self.temp_buffer[s] = 0.0;
            }
        }

        self.pre_delay.process(&self.temp_buffer, sample_count);

        // TODO: allocation!
        let mut multitap_input = vec![0.0; sample_count];
        for s in 0..sample_count {
            multitap_input[s] = self.pre_delay.get_output(s);
        }
        self.multitap_diffuser.process(&multitap_input, sample_count);

        // TODO: allocation!
        let mut early_out_stage = vec![0.0; sample_count];
        if self.diffuser_enabled {
            for s in 0..sample_count {
                early_out_stage[s] = self.allpass_diffuser.get_output(s);
            }
        } else {
            for s in 0..sample_count {
                early_out_stage[s] = self.multitap_diffuser.get_output(s);
            }
        }

        if self.diffuser_enabled {
            // TODO: allocation!
            let mut allpass_diffuser_input = vec![0.0; sample_count];
            for s in 0..sample_count {
                allpass_diffuser_input[s] = self.multitap_diffuser.get_output(s);
            }
            self.allpass_diffuser.process(&allpass_diffuser_input, sample_count);

            for s in 0..sample_count {
                self.temp_buffer[s] = self.allpass_diffuser.get_output(s);
            }
        } else {
            for s in 0..sample_count {
                self.temp_buffer[s] = self.multitap_diffuser.get_output(s);
            }
        }

        for line_idx in 0..self.line_count {
            self.delay_lines[line_idx].process(&self.temp_buffer, sample_count);
        }

        for line_idx in 0..self.line_count {
            // TODO: allocation!
            let mut buf = vec![0.0; sample_count];
            for s in 0..sample_count {
                buf[s] = self.delay_lines[line_idx].get_output(s);
            }

            if line_idx == 0 {
                for j in 0..sample_count {
                    self.temp_buffer[j] = buf[j];
                }
            } else {
                for j in 0..sample_count {
                    self.temp_buffer[j] += buf[j];
                }
            }
        }

        let per_line_gain = self.get_per_line_gain();
        utils::gain(&mut self.temp_buffer, per_line_gain, sample_count);

        for s in 0..sample_count {
            self.line_out_buffer[s] = self.temp_buffer[s]
        }

        for s in 0..sample_count {
            self.out_buffer[s] =
                self.dry_out * input[s] +
                self.predelay_out * pre_delay_output[s] +
                self.early_out * early_out_stage[s] +
                self.line_out * self.temp_buffer[s];
        }
    }

    pub fn clear_buffers(&mut self) {
        for s in 0..BUFFER_SIZE {
            self.temp_buffer[s] = 0.0;
            self.line_out_buffer[s] = 0.0;
            self.out_buffer[s] = 0.0;
        }

        self.low_pass_filter.set_output(0.0); // ?
        self.high_pass_filter.set_output(0.0); // ?

        self.pre_delay.clear_buffers();
        self.multitap_diffuser.clear_buffers();
        self.allpass_diffuser.clear_buffers();

        for line in &mut self.delay_lines {
            line.clear_buffers();
        }
    }

    fn get_per_line_gain(&self) -> f64 {
        1.0 / (self.line_count as f64).sqrt()
    }

    fn update_lines(&mut self) {
        let line_delay_samples = milliseconds_to_samples(
            self.internal_parameters[ParameterName::LineDelay as usize],
            self.sample_rate
        ) as i32;
        let line_decay_milliseconds = self.internal_parameters[ParameterName::LineDecay as usize] * 1000.0;
        let line_decay_samples = milliseconds_to_samples(
            line_decay_milliseconds,
            self.sample_rate
        );
        let line_modulation_amount = milliseconds_to_samples(
            self.internal_parameters[ParameterName::LineModAmount as usize],
            self.sample_rate
        );
        let line_modulation_rate = self.internal_parameters[ParameterName::LineModRate as usize];
        let late_diffusion_modulation_amount = milliseconds_to_samples(
            self.internal_parameters[ParameterName::LateDiffusionModAmount as usize],
            self.sample_rate
        );
        let late_diffusion_modulation_rate = self.internal_parameters[ParameterName::LateDiffusionModRate as usize];
        let delay_line_seeds = sha256_random::generate_with_cross_seed(
            self.delay_line_seed as u64,
            (self.delay_lines.len() * 3) as i32,
            self.cross_seed,
        );
        let count = self.delay_lines.len();

        for i in 0..count {
            let mod_amount = line_modulation_amount * (0.7 + 0.3 * delay_line_seeds[i + count]);
            let mod_rate = line_modulation_rate * (0.7 + 0.3 * delay_line_seeds[i + 2*count]) / self.sample_rate as f64;

            let mut delay_samples = (0.5 + 1.0 * delay_line_seeds[i]) * line_delay_samples as f64;
            if delay_samples < mod_amount + 2.0 {
                // When the delay is set really short, and the modulation is very high, the
                // modulation could actually take the delay time negative. Prevent that!
                // Provide 2 extra samples as margin of safety.
                delay_samples = mod_amount + 2.0;
            }

            // line_decay is the time it takes to reach T60
            let db_after_1_iteration = delay_samples / line_decay_samples * (-60.0);
            let gain_after_1_iteration = utils::db_to_gain(db_after_1_iteration);

            self.delay_lines[i].set_delay(delay_samples as i32);
            self.delay_lines[i].set_feedback(gain_after_1_iteration);
            self.delay_lines[i].set_line_modulation_amount(mod_amount);
            self.delay_lines[i].set_line_modulation_rate(mod_rate);
            self.delay_lines[i].set_diffuser_modulation_amount(late_diffusion_modulation_amount);
            self.delay_lines[i].set_diffuser_modulation_rate(late_diffusion_modulation_rate);
        }
    }

    fn update_post_diffusion(&mut self) {
        for i in 0..self.delay_lines.len() {
            self.delay_lines[i].set_diffuser_seed(
                ((self.post_diffusion_seed as u64) * (i as u64 + 1)) as i32,
                self.cross_seed
            );
        }
    }
}

fn milliseconds_to_samples(milliseconds: f64, sample_rate: u32) -> f64 {
    milliseconds / 1000.0 * (sample_rate as f64)
}