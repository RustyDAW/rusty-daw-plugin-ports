pub const MAX_STAGE_COUNT: i64 = 8;

pub struct AllpassDiffuser {
    stages: i32,
}

impl AllpassDiffuser {
    #[allow(unused_variables)]
    pub fn new(sample_rate: u32, delay_buffer_length_ms: u32) -> Self {
        todo!()
    }

    #[allow(dead_code)]
    pub fn get_sample_rate(&self) -> u32 {
        todo!()
    }

    #[allow(dead_code)]
    #[allow(unused_variables)]
    pub fn set_sample_rate(&mut self, sample_rate: u32) {
        todo!();
    }

    #[allow(unused_variables)]
    pub fn set_seed(&mut self, seed: i32) {
        todo!();
    }

    #[allow(unused_variables)]
    pub fn set_cross_seed(&mut self, cross_seed: f64) {
        todo!();
    }

    #[allow(dead_code)]
    pub fn get_modulation_enabled(&self) -> bool {
        todo!()
    }

    #[allow(unused_variables)]
    pub fn set_modulation_enabled(&mut self, enabled: bool) {
        todo!();
    }

    #[allow(unused_variables)]
    pub fn set_interpolation_enabled(&mut self, enabled: bool) {
        todo!();
    }

    #[allow(unused_variables)]
    pub fn get_output(&self, sample: usize) -> f64 {
        todo!()
    }

    #[allow(unused_variables)]
    pub fn set_delay(&mut self, delay_samples: usize) {
        todo!();
    }

    #[allow(unused_variables)]
    pub fn set_feedback(&mut self, feedback: f64) {
        todo!();
    }

    #[allow(unused_variables)]
    pub fn set_modulation_amount(&mut self, modulation_amount: f64) {
        todo!();
    }

    #[allow(unused_variables)]
    pub fn set_modulation_rate(&mut self, modulation_rate: f64) {
        todo!();
    }

    #[allow(unused_variables)]
    pub fn process(&mut self, input: &[f64], sample_count: usize) {
        todo!();
    }

    pub fn clear_buffers(&mut self) {
        todo!();
    }

    #[allow(dead_code)]
    pub fn get_stages(&self) -> i32 {
        self.stages
    }

    pub fn set_stages(&mut self, stages: i32) {
        self.stages = stages;
    }

    #[allow(dead_code)]
    fn update(&mut self) {
        todo!();
    }

    #[allow(dead_code)]
    fn update_seeds(&mut self) {
        todo!();
    }
}