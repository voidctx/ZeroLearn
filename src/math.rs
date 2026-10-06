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

pub fn f_t(s1:f32,s4:f32,s6:f32,data:f32,temp:f32)->f32 {
    let threshold = (s1 + s1 * s1) / (E as f32);
    let sensitivity = (s4 + SQRT_2 * s4 * s4) / (s6 + E.recip());
    let thermal = temp.abs() + s6 + E.recip();
    let x = (data - threshold) * sensitivity / thermal;
    if x >= 0.0 {
        let q = (-x).exp();
        (q + 1.0).recip()
    } else {
        let q = x.exp();
        q / (q + 1.0)
    }
}

pub fn f_d(s2:f32,s7:f32)->f32 {
    let x = s2 * s2 + s7 * s7;
    (-x / E).exp()
}

pub fn f_n(s3:f32,s5:f32)->f32 {
    let magnitude = (s3 * s3 + s5 * s5).sqrt();
    magnitude / SQRT_2
}

pub fn f_r(s2:f32,s8:f32,t3:f32,t7:f32)->f32 {
    let ds = s2 - t3;
    let dt = s8 - t7;
    let distance = (ds * ds + dt * dt).sqrt();
    (distance * E).exp()
}