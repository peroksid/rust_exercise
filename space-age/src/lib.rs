#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

const REFERENCE_YEAR: f64 = 31_557_600_f64;
pub trait Planet {
    const FACTOR: f64;
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / REFERENCE_YEAR / Self::FACTOR
    }
}
pub struct Mercury;
impl Planet for Mercury {
    const FACTOR: f64 = 0.2408467_f64;
}
pub struct Venus;
impl Planet for Venus {
    const FACTOR: f64 = 0.61519726_f64;
}
pub struct Earth;
impl Planet for Earth {
    const FACTOR: f64 = 1.0;
}
pub struct Mars;
impl Planet for Mars {
    const FACTOR: f64 = 1.8808158_f64;
}
pub struct Jupiter;
impl Planet for Jupiter {
    const FACTOR: f64 = 11.862615f64;
}
pub struct Saturn;
impl Planet for Saturn {
    const FACTOR: f64 = 29.447498_f64;
}
pub struct Uranus;
impl Planet for Uranus {
    const FACTOR: f64 = 84.016846_f64;
}
pub struct Neptune;
impl Planet for Neptune {
    const FACTOR: f64 = 164.79132_f64;
}
