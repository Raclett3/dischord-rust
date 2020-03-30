use std::f64::consts::PI;
use std::i32;

pub type Wave = fn(f64, f64) -> f64;

pub fn pulse50(frequency: f64, position: f64) -> f64 {
    if (position * frequency) % 1.0 >= 0.5 {
        1.0
    } else {
        -1.0
    }
}

pub fn pulse25(frequency: f64, position: f64) -> f64 {
    if (position * frequency) % 1.0 >= 0.25 {
        1.0
    } else {
        -1.0
    }
}

pub fn pulse125(frequency: f64, position: f64) -> f64 {
    if (position * frequency) % 1.0 >= 0.125 {
        1.0
    } else {
        -1.0
    }
}

pub fn triangle(frequency: f64, position: f64) -> f64 {
    let value = (frequency * position + 0.5) % 1.0 * 4.0;
    if value <= 2.0 {
        value - 1.0
    } else {
        -value + 3.0
    }
}

pub fn saw(frequency: f64, position: f64) -> f64 {
    (frequency * position + 0.5) % 1.0 * 2.0 - 1.0
}

pub fn sine(frequency: f64, position: f64) -> f64 {
    (position * frequency * PI * 2.0).sin()
}

static mut RAND: i32 = 1;

fn next() -> i32 {
    let a: i32 = 48271;
    let m: i32 = 2_147_483_647;
    unsafe {
        let hi: i32 = RAND / (m / a);
        let lo: i32 = RAND % (m / a);
        let test = a * lo - (m % a) * hi;
        RAND = if test > 0 {
            test
        } else {
            test + m
        };
        RAND
    }
}

pub fn noise(_: f64, _: f64) -> f64 {
    next() as f64 / i32::MAX as f64
}

pub fn better_pulse(frequency: f64, position: f64) -> f64 {
    let mut sum: f64 = 0.0;
    let mut nth = 1f64;
    while nth * frequency < 20000f64 {
        sum += (position * frequency * nth * PI * 2.0).sin() / nth;
        nth += 2.0;
    }
    sum
}
