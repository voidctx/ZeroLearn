use crate::structs::Coeffs;
use std::f32::consts::{E,PI,TAU,FRAC_PI_2,FRAC_PI_4,SQRT_2,SQRT_3,LN_2,LOG2_E};

fn wave(x: f64)->f32 {
    ((x.sin() + 1.0) * 0.5) as f32
}

fn smooth(x: f32)->f32 {
    x * x * (3.0 - 2.0 * x)
}

pub fn genCoeffs(seed:u64)->Coeffs {
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

fn f_t(s1:f32,s4:f32,s6:f32,data:f32,temp:f32)->f32 {
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

fn f_d(s2:f32,s7:f32)->f32 {
    let x = s2 * s2 + s7 * s7;
    (-x / E).exp()
}

fn f_n(s3:f32,s5:f32)->f32 {
    let magnitude = (s3 * s3 + s5 * s5).sqrt();
    magnitude / SQRT_2
}

fn f_r(s2:f32,s8:f32,t3:f32,t7:f32)->f32 {
    let ds = s2 - t3;
    let dt = s8 - t7;
    let distance = (ds * ds + dt * dt).sqrt();
    (distance * E).exp()
}

pub fn propagate(sn: &Neuron, tn: &Neuron, data: f32, temp: f32) -> f32 {
    let source = genCoeffs(sn.seed);
    let activation = f_t(source.a1,source.a4,source.a6,data,temp);
    if activation < 0.5 {
        return 0.0;
    }
    let target = genCoeffs(tn.seed);
    let resistance = f_r(source.a2,source.a8,target.a3,target.a7);
    let time = 0.25 + resistance * resistance;
    let decay_rate = f_d(source.a2,source.a7);
    let decay = (-decay_rate * time).exp();
    let mut result = data * decay;
    let noise_rate = f_n(source.a3,source.a5);
    let temperature_factor = 0.5 + temp.clamp(0.0, 4.0) * 0.5;
    let noise_state = (source.a3+source.a5+source.a7+source.a8) * 0.25 - 0.5;
    let mut noise = noise_state* noise_rate * time.sqrt() * temperature_factor;
    static RNG_STATE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0x6a09e667f3bcc909);
    let mut rng = RNG_STATE.fetch_add(0x9e3779b97f4a7c15,std::sync::atomic::Ordering::Relaxed);
    rng ^= sn.seed;
    rng ^= sn.seed.rotate_left(17);
    rng ^= rng << 13;
    rng ^= rng >> 7;
    rng ^= rng << 17;
    if rng & 0x0f < 3 {
        let magnitude = 0.01 + (((rng >> 8) & 0xff) as f32 / 255.0) * 0.02;
        let direction = if rng & 0x10000 != 0 { 1.0 } else { -1.0 };
        noise *= 1.0 + direction * magnitude;
    }
    result += noise;
    result
}