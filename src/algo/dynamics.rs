#[inline]
pub fn formula_d(d: f64, t: f64, da: f64, ta: f64) -> f64 {
    d + da * f64::exp((ta - t) / 200.0)
}

#[inline]
pub fn formula_p(s: f64, u: f64, t: f64, d: f64) -> f64 {
    let k_m: f64 = 1.0 / (1.0 - f64::exp(-0.005));
    let m = s - (s - u) * f64::powf(1.0 - f64::exp(-t / 10000.0), 10.0);
    if d < 20.0 {
        m + (0.5 - m) * (d / k_m)
    } else {
        m + (1.0 - m) * (f64::powf(4.0, d / k_m) - 1.0) / 3.0
    }
}
