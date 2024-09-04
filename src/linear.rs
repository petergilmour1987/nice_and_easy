use super::ease::Easing;
use num_traits::Float;

#[derive(Debug)]
pub struct Linear;

impl<F: Float> Easing<F> for Linear {
    fn ease_in(t: F, b: F, c: F, d: F) -> F {
        c * t / d + b
    }

    fn ease_out(t: F, b: F, c: F, d: F) -> F {
        c * t / d + b
    }

    fn ease_in_out(t: F, b: F, c: F, d: F) -> F {
        c * t / d + b
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::{assert_relative_eq, assert_relative_ne};

    #[test]
    fn ease_in() {
        assert_relative_eq!(Linear::ease_in(1.0_f32, 2.0, 3.0, 4.0), 2.7500);
    }

    #[test]
    fn ease_out() {
        assert_relative_eq!(Linear::ease_out(1.0_f32, 2.0, 3.0, 4.0), 2.7500);
    }

    #[test]
    fn ease_in_out() {
        assert_relative_eq!(Linear::ease_in_out(1.0_f32, 2.0, 3.0, 4.0), 2.7500);
    }

    const RESULT: f64 = 2.948683298050514;

    #[test]
    fn f32_precision() {
        let ease32 = Linear::ease_in(10_f32.sqrt(), 2.0, 3.0, 10.0);
        assert_relative_ne!(ease32 as f64, RESULT);
        assert_relative_eq!(ease32, RESULT as f32);
    }

    #[test]
    fn f64_precision() {
        let ease64 = Linear::ease_in(10_f64.sqrt(), 2.0, 3.0, 10.0);
        assert_relative_eq!(ease64, RESULT);
    }
}
