use crate::structs::Coeffs;

fn mix64(mut x: u64) -> u64 {
    x ^= x >> 30;
    x = x.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x ^= x >> 27;
    x = x.wrapping_mul(0x94d0_49bb_1331_11eb);
    x ^ (x >> 31)
}

fn unit(x: u64) -> f32 {
    ((x >> 40) as u32) as f32 / 16_777_215.0
}

fn signed(x: u64) -> f32 {
    unit(x) * 2.0 - 1.0
}

pub fn genCoeffs(seed: u64) -> Coeffs {
    let a = mix64(seed ^ 0x243f_6a88_85a3_08d3);
    let b = mix64(seed ^ 0x1319_8a2e_0370_7344);
    let c = mix64(seed ^ 0xa409_3822_299f_31d0);
    let d = mix64(seed ^ 0x082e_fa98_ec4e_6c89);
    let e = mix64(seed ^ 0x4528_21e6_38d0_1377);
    let f = mix64(seed ^ 0xbe54_66cf_34e9_0c6c);
    let g = mix64(seed ^ 0xc0ac_29b7_c97c_50dd);
    let h = mix64(seed ^ 0x3f84_d5b5_b547_0917);

    Coeffs {
        a1: unit(a),
        a2: unit(b),
        a3: unit(c),
        a4: unit(d),
        a5: unit(e),
        a6: unit(f),
        a7: unit(g),
        a8: unit(h),
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