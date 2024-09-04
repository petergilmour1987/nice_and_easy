use num_traits::{Float, FloatConst};

macro_rules! cast_constants {
    ($t:ty; $( $name:ident=$val:expr ),+) => {
        $(
            let $name: $t = f($val as f64);
        )+
    }
}

#[inline]
fn f<T: Float>(x: f64) -> T {
    T::from(x).expect("Type must be f32 or f64")
}

pub fn linear<T: Float + FloatConst>(t: T, b: T, c: T, d: T) -> T {
    c * t / d + b
}

pub fn back_in<T: Float + FloatConst>(t: T, b: T, c: T, d: T) -> T {
    let s: T = f(1.70158);
    let t = t / d;
    c * t * t * ((s + f(1.0)) * t - s) + b
}

pub fn back_out<T: Float + FloatConst>(t: T, b: T, c: T, d: T) -> T {
    let s: T = f(1.70158);
    let t = (t / d) - f(1.0);
    c * (t * t * ((s + f(1.0)) * t + s) + f(1.0)) + b
}

pub fn back_in_out<T: Float + FloatConst>(t: T, b: T, c: T, d: T) -> T {
    let s: T = f(1.70158);
    let t = t / (d / f(2.0));
    if t < f(1.0) {
        let s = s * f(1.525);
        c / f(2.0) * (t * t * ((s + f(1.0)) * t - s)) + b
    } else {
        let t = t - f(2.0);
        let s = s * f(1.525);
        c / f(2.0) * (t * t * ((s + f(1.0)) * t + s) + f(2.0)) + b
    }
}

pub fn bounce_in<T: Float + FloatConst>(t: T, b: T, c: T, d: T) -> T {
    c - bounce_out(d - t, f(0.0), c, d) + b
}

pub fn bounce_out<T: Float + FloatConst>(t: T, b: T, c: T, d: T) -> T {
    cast_constants!(T; _1=1, _1_5=1.5, _2=2, _2_25=2.25, _2_5=2.5,
            _2_625=2.625, _7_5625=7.5625);

    let t = t / d;
    if t < _1 / f(2.75) {
        c * (_7_5625 * t * t) + b
    } else if t < _2 / f(2.75) {
        let t = t - _1_5 / f(2.75);
        c * (_7_5625 * t * t + f(0.75)) + b
    } else if t < _2_5 / f(2.75) {
        let t = t - _2_25 / f(2.75);
        c * (_7_5625 * t * t + f(0.9375)) + b
    } else {
        let t = t - _2_625 / f(2.75);
        c * (_7_5625 * t * t + f(0.984375)) + b
    }
}

pub fn bounce_in_out<T: Float + FloatConst>(t: T, b: T, c: T, d: T) -> T {
    if t < (d / f(2.0)) {
        bounce_in(t * f(2.0), f(0.0), c, d) * f(0.5) + b
    } else {
        bounce_out(t * f(2.0) - d, f(0.0), c, d) * f(0.5) + c * f(0.5) + b
    }
}

pub fn circ_in<T: Float + FloatConst>(t: T, b: T, c: T, d: T) -> T {
    let t = t / d;
    -c * ((T::one() - t * t).sqrt() - f(1.0)) + b
}

pub fn circ_out<T: Float + FloatConst>(t: T, b: T, c: T, d: T) -> T {
    let t = t / d - f(1.0);
    c * (T::one() - t * t).sqrt() + b
}

pub fn circ_in_out<T: Float + FloatConst>(t: T, b: T, c: T, d: T) -> T {
    let t = t / (d / f(2.0));
    if t < f(1.0) {
        -c / f(2.0) * ((T::one() - t * t).sqrt() - f(1.0)) + b
    } else {
        let t = t - f(2.0);
        c / f(2.0) * ((T::one() - t * t).sqrt() + f(1.0)) + b
    }
}

pub fn cubic_in<T: Float + FloatConst>(t: T, b: T, c: T, d: T) -> T {
    let t = t / d;
    c * (t * t * t) + b
}

pub fn cubic_out<T: Float + FloatConst>(t: T, b: T, c: T, d: T) -> T {
    let t = t / d - f(1.0);
    c * ((t * t * t) + f(1.0)) + b
}

pub fn cubic_in_out<T: Float + FloatConst>(t: T, b: T, c: T, d: T) -> T {
    let t = t / (d / f(2.0));
    if t < f(1.0) {
        c / f(2.0) * (t * t * t) + b
    } else {
        let t = t - f(2.0);
        c / f(2.0) * (t * t * t + f(2.0)) + b
    }
}

pub fn elastic_in<T: Float + FloatConst>(t: T, b: T, c: T, d: T) -> T {
    cast_constants!(T; _2=2, _10=10);

    if t == f(0.0) {
        return b;
    }

    let t = t / d;
    if t == f(1.0) {
        return b + c;
    }

    let p = d * f(0.3);
    let a = c;
    let s = p / f(4.0);
    let t = t - f(1.0);
    let post_fix = a * _2.powf(_10 * t);
    let temp = (t * d - s) * (_2 * T::PI()) / p;
    -(post_fix * temp.sin()) + b
}

pub fn elastic_out<T: Float + FloatConst>(t: T, b: T, c: T, d: T) -> T {
    cast_constants!(T; _2=2, _10=10);

    if t == f(0.0) {
        return b;
    }

    let t = t / d;
    if t == f(1.0) {
        return b + c;
    }

    let p = d * f(0.3);
    let a = c;
    let s = p / f(4.0);
    let temp = (t * d - s) * (_2 * T::PI()) / p;
    a * _2.powf(-_10 * t) * temp.sin() + c + b
}

pub fn elastic_in_out<T: Float + FloatConst>(t: T, b: T, c: T, d: T) -> T {
    cast_constants!(T; _2=2, _10=10, _0_5=0.5);

    if t == f(0.0) {
        return b;
    }

    let t = t / (d / _2);
    if t == _2 {
        return b + c;
    }

    let p = d * f(0.3) * f(1.5);
    let a = c;
    let s = p / f(4.0);

    if t < f(1.0) {
        let t = t - f(1.0);
        let post_fix = a * _2.powf(_10 * t);
        let temp = (t * d - s) * (_2 * T::PI()) / p;
        return -_0_5 * (post_fix * temp.sin()) + b;
    }

    let t = t - f(1.0);
    let post_fix = a * _2.powf(-_10 * t);
    let temp = (t * d - s) * (_2 * T::PI()) / p;
    post_fix * temp.sin() * f(0.5) + c + b
}

pub fn expo_in<T: Float + FloatConst>(t: T, b: T, c: T, d: T) -> T {
    cast_constants!(T; _2=2, _10=10);

    if t == f(0.0) {
        b
    } else {
        c * _2.powf(_10 * (t / d - f(1.0))) + b
    }
}

pub fn expo_out<T: Float + FloatConst>(t: T, b: T, c: T, d: T) -> T {
    cast_constants!(T; _2=2, _10=10);

    if t == d {
        b + c
    } else {
        c * (-_2.powf(-_10 * t / d) + f(1.0)) + b
    }
}

pub fn expo_in_out<T: Float + FloatConst>(t: T, b: T, c: T, d: T) -> T {
    cast_constants!(T; _2=2, _10=10);

    if t == f(0.0) {
        return b;
    }
    if t == d {
        return b + c;
    }
    let t = t / (d / _2);
    if t < f(1.0) {
        c / _2 * _2.powf(_10 * (t - f(1.0))) + b
    } else {
        let t = t - f(1.0);
        c / _2 * (-(_2.powf(-_10 * t)) + _2) + b
    }
}

pub fn quad_in<T: Float + FloatConst>(t: T, b: T, c: T, d: T) -> T {
    let t = t / d;
    c * t * t + b
}

pub fn quad_out<T: Float + FloatConst>(t: T, b: T, c: T, d: T) -> T {
    let t = t / d;
    -c * t * (t - f(2.0)) + b
}

pub fn quad_in_out<T: Float + FloatConst>(t: T, b: T, c: T, d: T) -> T {
    let t = t / (d / f(2.0));
    if t < f(1.0) {
        c / f(2.0) * t * t + b
    } else {
        let t = t - f(1.0);
        -c / f(2.0) * (t * (t - f(2.0)) - f(1.0)) + b
    }
}

pub fn quart_in<T: Float + FloatConst>(t: T, b: T, c: T, d: T) -> T {
    let t = t / d;
    c * (t * t * t * t) + b
}

pub fn quart_out<T: Float + FloatConst>(t: T, b: T, c: T, d: T) -> T {
    let t = t / d - f(1.0);
    -c * ((t * t * t * t) - f(1.0)) + b
}

pub fn quart_in_out<T: Float + FloatConst>(t: T, b: T, c: T, d: T) -> T {
    let t = t / (d / f(2.0));
    if t < f(1.0) {
        c / f(2.0) * (t * t * t * t) + b
    } else {
        let t = t - f(2.0);
        -c / f(2.0) * ((t * t * t * t) - f(2.0)) + b
    }
}

pub fn quint_in<T: Float + FloatConst>(t: T, b: T, c: T, d: T) -> T {
    let t = t / d;
    c * (t * t * t * t * t) + b
}

pub fn quint_out<T: Float + FloatConst>(t: T, b: T, c: T, d: T) -> T {
    let t = t / d - f(1.0);
    c * ((t * t * t * t * t) + f(1.0)) + b
}

pub fn quint_in_out<T: Float + FloatConst>(t: T, b: T, c: T, d: T) -> T {
    let t = t / (d / f(2.0));
    if t < f(1.0) {
        (c / f(2.0)) * (t * t * t * t * t) + b
    } else {
        let t = t - f(2.0);
        c / f(2.0) * ((t * t * t * t * t) + f(2.0)) + b
    }
}

pub fn sine_in<T: Float + FloatConst>(t: T, b: T, c: T, d: T) -> T {
    -c * (t / d * (T::PI() / f(2.0))).cos() + c + b
}

pub fn sine_out<T: Float + FloatConst>(t: T, b: T, c: T, d: T) -> T {
    c * (t / d * (T::PI() / f(2.0))).sin() + b
}

pub fn sine_in_out<T: Float + FloatConst>(t: T, b: T, c: T, d: T) -> T {
    -c / f(2.0) * ((T::PI() * t / d).cos() - f(1.0)) + b
}
