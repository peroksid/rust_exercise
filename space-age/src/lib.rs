// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

macro_rules! planet_system {
    (reference year $e:literal;$($rest:tt)*) => {
        const REFERENCE_YEAR: f64 = $e;
        planet_system!($($rest)*);
    };
    (planet $name:ident $factor:literal;$($rest:tt)*) => {
        pub struct $name;

        impl Planet for $name {
            fn years_during(d: &Duration) -> f64 {
                d.0 as f64 / REFERENCE_YEAR  / $factor
            }
        }
        planet_system!($($rest)*);
    };
    (planet $name:ident;$($rest:tt)*) => {
        pub struct $name;

        impl Planet for $name {
            fn years_during(d: &Duration) -> f64 {
                d.0 as f64 / REFERENCE_YEAR
            }
        }
        planet_system!($($rest)*);
    };
    () => {};
}
planet_system! {
    reference year 31_557_600_f64;
    planet Mercury 0.2408467_f64;
    planet Venus 0.61519726_f64;
    planet Earth;
    planet Mars 1.8808158_f64;
    planet Jupiter 11.862615f64;
    planet Saturn 29.447498_f64;
    planet Uranus 84.016846_f64;
    planet Neptune 164.79132_f64;
}
