use num_traits::Float;

pub trait Easing<F: Float> {
    fn ease_in(t: F, b: F, c: F, d: F) -> F;
    fn ease_out(t: F, b: F, c: F, d: F) -> F;
    fn ease_in_out(t: F, b: F, c: F, d: F) -> F;
}
