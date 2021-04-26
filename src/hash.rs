//! Taken from https://nullprogram.com/blog/2018/07/31/

pub fn dual_hash_32<const S1: u32, const M1: u32, const S2: u32, const M2: u32, const S3: u32>(
    mut v: u32,
) -> u32 {
    v ^= v >> S1;
    v = v.wrapping_mul(M1);
    v ^= v >> S2;
    v = v.wrapping_mul(M2);
    v ^= v >> S3;
    v
}
pub fn triple_hash_32<
    const S1: u32,
    const M1: u32,
    const S2: u32,
    const M2: u32,
    const S3: u32,
    const M3: u32,
    const S4: u32,
>(
    mut v: u32,
) -> u32 {
    v ^= v >> S1;
    v = v.wrapping_mul(M1);
    v ^= v >> S2;
    v = v.wrapping_mul(M2);
    v ^= v >> S3;
    v = v.wrapping_mul(M3);
    v ^= v >> S4;
    v
}

pub fn dual_hash_64<const S1: u64, const M1: u64, const S2: u64, const M2: u64, const S3: u64>(
    mut v: u64,
) -> u64 {
    v ^= v >> S1;
    v = v.wrapping_mul(M1);
    v ^= v >> S2;
    v = v.wrapping_mul(M2);
    v ^= v >> S3;
    v
}

pub fn hash_32(v: u32) -> u32 {
    dual_hash_32::<16, 0x7feb352d, 15, 0x846ca68b, 16>(v)
}
pub fn hash_32_ext(v: u32) -> u32 {
    triple_hash_32::<17, 0xed5ad4bb, 11, 0xac4c1b51, 15, 0x31848bab, 14>(v)
}

pub fn hash_64(v: u64) -> u64 {
    dual_hash_64::<32, 0xd6e8feb86659fd93, 32, 0xd6e8feb86659fd93, 32>(v)
}

pub fn seed_hash_32(v: u32, seed: u32) -> u32 {
    hash_32(seed ^ hash_32(v))
}

pub fn seed_hash_64(v: u64, seed: u64) -> u64 {
    hash_64(seed ^ hash_64(v))
}
