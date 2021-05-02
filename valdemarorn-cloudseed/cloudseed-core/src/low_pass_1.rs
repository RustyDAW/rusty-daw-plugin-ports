pub struct LowPass1 {
    output: f64
}

impl LowPass1 {
    #[allow(unused_variables)]
    pub fn new(sample_rate: u32) -> Self {
        todo!()
    }

    #[allow(dead_code)]
    pub fn get_sample_rate(&self) -> u32 {
        todo!()
    }

    #[allow(unused_variables)]
    pub fn set_sample_rate(&mut self, sample_rate: u32) {
        todo!();
    }

    #[allow(dead_code)]
    pub fn get_cutoff_hz(&self) -> f64 {
        todo!()
    }

    #[allow(unused_variables)]
    pub fn set_cutoff_hz(&mut self, hz: f64) {
        todo!();
    }

    #[allow(dead_code)]
    pub fn update(&mut self) {
        todo!();
    }

    #[allow(dead_code)]
    #[allow(unused_variables)]
    pub fn process(&mut self, input: f64) -> f64 {
        todo!()
    }

    #[allow(unused_variables)]
    pub fn process_buffer(&mut self, input: &[f64], output: &mut [f64], len: usize) {
        todo!();
    }

    #[allow(dead_code)]
    pub fn get_output(&self) -> f64 {
        self.output
    }

    pub fn set_output(&mut self, value: f64) {
        self.output = value;
    }
}