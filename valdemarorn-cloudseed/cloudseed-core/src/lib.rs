mod parameters;
mod allpass_diffuser;
mod multitap_diffuser;
mod value_tables;
mod reverb_channel;

use parameters::{Parameters, ParameterName};
use reverb_channel::{ReverbChannel, ChannelLr};

// TODO: does this need to be a member variable instead?
pub const BUFFER_SIZE: usize = 4096; // "just make it huge by default..."

pub struct Cloudseed {
    sample_rate: u32,
    channel_left: ReverbChannel,
    channel_right: ReverbChannel,
    left_channel_in: [f64; BUFFER_SIZE],
    right_channel_in: [f64; BUFFER_SIZE],
    #[allow(dead_code)] left_line_buffer: [f64; BUFFER_SIZE], // TODO: used anywhere??
    #[allow(dead_code)] right_line_buffer: [f64; BUFFER_SIZE], // TODO: used anywhere??
    parameters: Parameters,
}

impl Cloudseed {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            sample_rate,
            channel_left: ReverbChannel::new(sample_rate, ChannelLr::Left),
            channel_right: ReverbChannel::new(sample_rate, ChannelLr::Right),
            left_channel_in: [0.0; BUFFER_SIZE],
            right_channel_in: [0.0; BUFFER_SIZE],
            left_line_buffer: [0.0; BUFFER_SIZE],
            right_line_buffer: [0.0; BUFFER_SIZE],
            parameters: Parameters::new(),
        }
    }

    pub fn get_sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn set_sample_rate(&mut self, sample_rate: u32) {
        self.sample_rate = sample_rate;
        self.channel_left.set_sample_rate(sample_rate);
        self.channel_right.set_sample_rate(sample_rate);
    }

    pub fn get_parameter_count(&self) -> usize {
        parameters::PARAMETER_COUNT
    }

    // NOTE: not sure if needed
    #[allow(dead_code)]
    pub fn get_all_parameters(&self) {
        todo!();
    }

    pub fn get_scaled_parameter(&self, parameter_name: ParameterName) -> f64 {
        self.parameters.get_scaled(parameter_name)
    }

    pub fn set_parameter(&mut self, parameter_name: ParameterName, value: f64) {
        self.parameters.set(parameter_name, value);

        // also make the channels handle this parameter change
        let scaled_value = self.get_scaled_parameter(parameter_name);
        self.channel_left.handle_parameter_change(parameter_name, scaled_value);
        self.channel_right.handle_parameter_change(parameter_name, scaled_value);
    }

    pub fn clear_buffers(&mut self) {
        self.channel_left.clear_buffer();
        self.channel_right.clear_buffer();
    }

    pub fn process(&mut self, input: &[&[f64]], output: &mut [&mut [f64]], buffer_size: usize) {
        // TODO: better variable names
        let cm = self.parameters.get_scaled(ParameterName::InputMix) * 0.5;
        let cmi = 1.0 - cm;

        for i in 0..buffer_size {
            self.left_channel_in[i] = input[0][i] * cmi + input[1][i] * cm;
            self.right_channel_in[i] = input[1][i] * cmi + input[0][i] * cm;
        }

        self.channel_left.process(&self.left_channel_in, buffer_size);
        self.channel_right.process(&self.right_channel_in, buffer_size);

        for i in 0..buffer_size {
            output[0][i] = self.channel_left.get_output(i);
            output[1][i] = self.channel_right.get_output(i);
        }
    }
}