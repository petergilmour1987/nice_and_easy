pub use num_traits::Float;

#[macro_export]
macro_rules! cast_constants {
    ($t:ty; $( $name:ident=$val:expr ),+) => {
        $(
            let $name: $t = f($val as f64);
        )+
    }
}

#[inline]
pub fn f<F: Float>(x: f64) -> F {
    F::from(x).expect("Type must be f32 or f64")
}

#[cfg(test)]
mod test {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn cast_constants_macro() {
        type F = f64;
        cast_constants!(F; _1_234=1.234, _234=234);
        assert_relative_eq!(_1_234, 1.234);
        assert_relative_eq!(_234, 234.0);
    }

    #[test]
    fn float_casting() {
        type F1 = f64;
        type F2 = f32;

        let root2: f64 = 2.0.sqrt();
        let root2_f1: F1 = f(root2);
        assert_relative_eq!(root2_f1, root2);

        let root2_f2: F2 = f(root2);
        assert_relative_eq!(root2_f2, root2 as F2);
    }
}
