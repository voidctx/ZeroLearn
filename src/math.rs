use crate::structs::Coeffs;
use std::f32::consts::{E,PI,TAU,FRAC_PI_2,FRAC_PI_4,SQRT_2,SQRT_3,LN_2,LOG2_E};

fn wave(x: f64)->f32 {
    ((x.sin() + 1.0) * 0.5) as f32
}

fn smooth(x: f32)->f32 {
    x * x * (3.0 - 2.0 * x)
}

pub fn coeffs(seed:u64)->Coeffs {
    let x = seed as f64;
    let a = wave(x*(1.0/SQRT_2 as f64) + PI as f64);
    let b = wave(x*(1.0/SQRT_3 as f64) + E as f64);
    let c = wave(x*(1.0/LOG2_E as f64) + FRAC_PI_4 as f64);
    let d = wave(x*(1.0/LN_2 as f64) + FRAC_PI_2 as f64);
    let e = wave(x*(1.0/PI as f64) + TAU as f64);
    let f = wave(x*(1.0/E as f64) + SQRT_2 as f64);
    let g = wave(x*(1.0/SQRT_3 as f64) + PI as f64 + E as f64);
    let h = wave(x*(1.0/SQRT_2 as f64) + TAU as f64 + FRAC_PI_4 as f64);

    Coeffs {
        a1: smooth(a),
        a2: smooth(b),
        a3: smooth(c),
        a4: smooth(d),
        a5: smooth(e),
        a6: smooth(f),
        a7: smooth(g),
        a8: smooth(h),
    }
}

pub fn f_t(a1:f32,a4:f32,a6:f32,data:f32,temp:f32)->f32 {
    let threshold = 0.15 + a1 * 0.70;
    let sensitivity = 0.5 + a4 * 3.5;
    let temperature = temp.max(0.001);
    let x = (data - threshold) * sensitivity / temperature;
    if x >= 0.0 {
        let e = (-x).exp();
        (1.0 + e * (1.0 + a6 * 0.05)).recip()
    } else {
        let e = x.exp();
        (e * (1.0 + a6 * 0.05)) / (1.0 + e * (1.0 + a6 * 0.05))
    }
}

pub fn f_d(s2:f32,s7:f32)->f32 {
    0.01 + 0.49 * (s2 * s2 * 0.65 + s7 * 0.35)
}

pub fn f_n(s3:f32,s5:f32)->f32 {
    0.0005 + 0.08 * (s3 * s3 * 0.7 + s5 * 0.3)
}

pub fn f_r(s2:f32,s8:f32,t3:f32,t7:f32)->f32 {
    let source = 0.35 + 1.65 * (s2 * 0.65 + s8 * 0.35);
    let target = 0.35 + 1.65 * (t3 * 0.55 + t7 * 0.45);
    source * target
}