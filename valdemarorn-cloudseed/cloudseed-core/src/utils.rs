pub fn gain(buffer: &mut [f64], gain: f64, sample_count: usize) {
    for s in 0..sample_count {
        buffer[s] *= gain;
    }
}

pub fn db_to_gain(input: f64) -> f64 {
    (10.0_f64).powf(input / 20.0)
}

#[allow(dead_code)]
pub fn gain_to_db(input: f64) -> f64 {
    if input < 0.0000001 {
        return -100000.0;
    }
    20.0 * input.log10()
}