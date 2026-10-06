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
