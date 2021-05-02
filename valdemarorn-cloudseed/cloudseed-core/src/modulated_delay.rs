pub struct ModulatedDelay {
    sample_delay: u32,
}

impl ModulatedDelay {
    #[allow(unused_variables)]
    pub fn new(buffer_size_samples: u32, sample_delay: u32) -> Self {
        todo!()
    }

    #[allow(unused_variables)]
    pub fn get_output(&self, sample: usize) -> f64 {
        todo!()
    }

    #[allow(unused_variables)]
    pub fn process(&mut self, input: &[f64], sample_count: usize) {
        todo!();
    }

    pub fn clear_buffers(&mut self) {
        todo!();
    }

    #[allow(dead_code)]
    pub fn get_sample_delay(&self) -> u32 {
        self.sample_delay
    }

    pub fn set_sample_delay(&mut self, value: u32) {
        self.sample_delay = value;
    }

    #[allow(dead_code)]
    fn update(&mut self) {
        todo!();
    }
}