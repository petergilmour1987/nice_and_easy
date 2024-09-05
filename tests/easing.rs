use approx::assert_relative_eq;
use nice_and_easy::*;

#[test]
fn test_linear() {
    assert_relative_eq!(linear::<f32>(1.0, 2.0, 3.0, 4.0), 2.7500);
}

#[test]
fn test_back_in() {
    assert_relative_eq!(back_in::<f32>(1.0, 2.0, 3.0, 4.0), 1.8075902);
}

#[test]
fn test_back_out() {
    assert_relative_eq!(back_out::<f32>(1.0, 2.0, 3.0, 4.0), 4.452229);
}

#[test]
fn test_back_in_out() {
    assert_relative_eq!(back_in_out::<f32>(1.0, 2.0, 3.0, 4.0), 1.7009544);
    assert_relative_eq!(back_in_out::<f32>(51.0, 1.0, 100.0, 100.0), 56.432546);
}

#[test]
fn test_bounce_in() {
    assert_relative_eq!(bounce_in::<f32>(1.0, 2.0, 3.0, 4.0), 2.082031);
    assert_relative_eq!(bounce_in_out::<f32>(1.0, 2.0, 3.0, 4.0), 2.3515625);
    assert_relative_eq!(bounce_in_out::<f32>(51.0, 1.0, 100.0, 100.0), 51.151250);
}

#[test]
fn test_bounce_out() {
    assert_relative_eq!(bounce_out::<f32>(1.0, 2.0, 3.0, 4.0), 3.4179688);
    assert_relative_eq!(bounce_out::<f32>(1.0, 2.0, 3.0, 2.0), 4.296875);
    assert_relative_eq!(bounce_out::<f32>(100.0, 1.0, 100.0, 100.0), 101.000000);
}

#[test]
fn test_bounce_in_out() {
    assert_relative_eq!(bounce_in_out::<f32>(1.0, 2.0, 3.0, 4.0), 2.3515625);
    assert_relative_eq!(bounce_in_out::<f32>(51.0, 1.0, 100.0, 100.0), 51.151250);
}

#[test]
fn test_circ_in() {
    assert_relative_eq!(circ_in::<f32>(1.0, 2.0, 3.0, 4.0), 2.0952625);
}

#[test]
fn test_circ_out() {
    assert_relative_eq!(circ_out::<f32>(1.0, 2.0, 3.0, 4.0), 3.9843135);
}

#[test]
fn test_circ_in_out() {
    assert_relative_eq!(circ_in_out::<f32>(1.0, 2.0, 3.0, 4.0), 2.200962);
    assert_relative_eq!(circ_in_out::<f32>(51.0, 1.0, 100.0, 100.0), 60.949871);
}

#[test]
fn test_cubic_in() {
    assert_relative_eq!(cubic_in::<f32>(1.0, 2.0, 3.0, 4.0), 2.046875);
}

#[test]
fn test_cubic_out() {
    assert_relative_eq!(cubic_out::<f32>(1.0, 2.0, 3.0, 4.0), 3.734375);
}

#[test]
fn test_cubic_in_out() {
    assert_relative_eq!(cubic_in_out::<f32>(1.0, 2.0, 3.0, 4.0), 2.187500);
    assert_relative_eq!(cubic_in_out::<f32>(51.0, 1.0, 100.0, 100.0), 53.940397);
}

#[test]
fn test_elastic_in() {
    assert_relative_eq!(elastic_in::<f32>(1.0, 2.0, 3.0, 4.0), 1.983427);
    assert_relative_eq!(elastic_in::<f32>(0.0, 1.0, 100.0, 100.0), 1.0000);
    assert_relative_eq!(elastic_in::<f32>(100.0, 1.0, 100.0, 100.0), 101.000);
}

#[test]
fn test_elastic_out() {
    assert_relative_eq!(elastic_out::<f32>(1.0, 2.0, 3.0, 4.0), 4.734835);
    assert_relative_eq!(elastic_out::<f32>(0.0, 1.0, 100.0, 100.0), 1.0000);
    assert_relative_eq!(elastic_out::<f32>(100.0, 1.0, 100.0, 100.0), 101.000);
}

#[test]
fn test_elastic_in_out() {
    assert_relative_eq!(elastic_in_out::<f32>(1.0, 2.0, 3.0, 4.0), 2.035908);
    assert_relative_eq!(elastic_in_out::<f32>(0.0, 1.0, 100.0, 100.0), 1.0000);
    assert_relative_eq!(elastic_in_out::<f32>(100.0, 1.0, 100.0, 100.0), 101.0000);
    assert_relative_eq!(elastic_in_out::<f32>(51.0, 1.0, 100.0, 100.0), 59.158646);
}

#[test]
fn test_expo_in() {
    assert_relative_eq!(expo_in::<f32>(1.0, 2.0, 3.0, 4.0), 2.016573);
    assert_relative_eq!(expo_in::<f32>(0.0, 1.0, 100.0, 100.0), 1.000000);
}

#[test]
fn test_expo_out() {
    assert_relative_eq!(expo_out::<f32>(1.0, 2.0, 3.0, 4.0), 4.469670);
    assert_relative_eq!(expo_out::<f32>(100.0, 1.0, 100.0, 100.0), 101.0000);
}

#[test]
fn test_expo_in_out() {
    assert_relative_eq!(expo_in_out::<f32>(1.0, 2.0, 3.0, 4.0), 2.046875);
    assert_relative_eq!(expo_in_out::<f32>(0.0, 1.0, 100.0, 100.0), 1.0000);
    assert_relative_eq!(expo_in_out::<f32>(100.0, 1.0, 100.0, 100.0), 101.000);
    assert_relative_eq!(expo_in_out::<f32>(51.0, 1.0, 100.0, 100.0), 57.472466);
}

#[test]
fn test_quad_in() {
    assert_relative_eq!(quad_in::<f32>(1.0, 2.0, 3.0, 4.0), 2.187500);
}

#[test]
fn test_quad_out() {
    assert_relative_eq!(quad_out::<f32>(1.0, 2.0, 3.0, 4.0), 3.312500);
}

#[test]
fn test_quad_in_out() {
    assert_relative_eq!(quad_in_out::<f32>(1.0, 2.0, 3.0, 4.0), 2.37500);
    assert_relative_eq!(quad_in_out::<f32>(51.0, 1.0, 100.0, 100.0), 52.98);
}

#[test]
fn test_quart_in() {
    assert_relative_eq!(quart_in::<f32>(1.0, 2.0, 3.0, 4.0), 2.011719);
}

#[test]
fn test_quart_out() {
    assert_relative_eq!(quart_out::<f32>(1.0, 2.0, 3.0, 4.0), 4.050781);
}

#[test]
fn test_quart_in_out() {
    assert_relative_eq!(quart_in_out::<f32>(1.0, 2.0, 3.0, 4.0), 2.093750);
    assert_relative_eq!(quart_in_out::<f32>(51.0, 1.0, 100.0, 100.0), 54.881588);
}

#[test]
fn test_quint_in() {
    assert_relative_eq!(quint_in::<f32>(1.0, 2.0, 3.0, 4.0), 2.002930);
}

#[test]
fn test_quint_out() {
    assert_relative_eq!(quint_out::<f32>(1.0, 2.0, 3.0, 4.0), 4.288086);
}

#[test]
fn test_quint_in_out() {
    assert_relative_eq!(quint_in_out::<f32>(1.0, 2.0, 3.0, 4.0), 2.046875);
    assert_relative_eq!(quint_in_out::<f32>(51.0, 1.0, 100.0, 100.0), 55.803956);
}

#[test]
fn test_sine_in() {
    assert_relative_eq!(sine_in::<f32>(1.0, 2.0, 3.0, 4.0), 2.2283616);
}

#[test]
fn test_sine_out() {
    assert_relative_eq!(sine_out::<f32>(1.0, 2.0, 3.0, 4.0), 3.148050);
}

#[test]
fn test_sine_in_out() {
    assert_relative_eq!(sine_in_out::<f32>(1.0, 2.0, 3.0, 4.0), 2.439340);
}
