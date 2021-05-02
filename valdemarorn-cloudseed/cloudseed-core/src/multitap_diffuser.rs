pub const MAX_TAPS: i64 = 50;

pub struct MultitapDiffuser {
}

impl MultitapDiffuser {
    #[allow(unused_variables)]
    pub fn new(delay_buffer_size: u32) -> Self {
        todo!()
    }

    #[allow(unused_variables)]
    pub fn set_seed(&mut self, seed: i32) {
        todo!();
    }

    #[allow(unused_variables)]
    pub fn set_cross_seed(&mut self, cross_seed: f64) {
        todo!();
    }

    #[allow(unused_variables)]
    pub fn get_output(&self, sample: usize) -> f64 {
        todo!()
    }

    #[allow(unused_variables)]
    pub fn set_tap_count(&mut self, tap_count: i32) {
        todo!();
    }

    #[allow(unused_variables)]
    pub fn set_tap_length(&mut self, tap_length: i32) {
        todo!();
    }

    #[allow(unused_variables)]
    pub fn set_tap_decay(&mut self, tap_decay: f64) {
        todo!();
    }

    #[allow(unused_variables)]
    pub fn set_tap_gain(&mut self, tap_gain: f64) {
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
    fn update(&mut self) {
        todo!();
    }

    #[allow(dead_code)]
    fn update_seeds(&mut self) {
        todo!();
    }
}