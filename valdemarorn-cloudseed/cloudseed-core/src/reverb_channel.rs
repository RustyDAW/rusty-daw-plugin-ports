#[allow(unused_imports)] use crate::BUFFER_SIZE;
use crate::parameters::ParameterName;

pub enum ChannelLr {
    Left,
    Right,
}

#[allow(dead_code)]
pub struct ReverbChannel {
    sample_rate: u32,
    channel_lr: ChannelLr,
}

impl ReverbChannel {
    pub fn new(sample_rate: u32, channel_lr: ChannelLr) -> Self {
        Self {
            sample_rate,
            channel_lr,
        }
    }

    #[allow(unused_variables)]
    pub fn handle_parameter_change(&mut self, parameter_name: ParameterName, value: f64) {
        todo!();
    }

    pub fn clear_buffer(&mut self) {
        todo!();
    }

    #[allow(unused_variables)]
    pub fn set_sample_rate(&mut self, sample_rate: u32) {
        todo!();
    }

    #[allow(unused_variables)]
    pub fn process(&mut self, input: &[f64], sample_count: usize) {
        todo!();
    }

    #[allow(unused_variables)]
    pub fn get_output(&self, sample: usize) -> f64 {
        todo!()
    }
}