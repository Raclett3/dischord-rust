pub type Wave = fn(f64, f64) -> f64;

pub fn pulse50(frequency: f64, position: f64) -> f64 {
    if (position * frequency) % 1.0 >= 0.5 {1.0} else {-1.0}
}