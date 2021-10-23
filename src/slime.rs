use slime_seed_finder::chunk::Chunk;
use slime_seed_finder::java_rng::JavaRng;
use std::num::Wrapping;

mod slime_const {
    use std::num::Wrapping;
    pub const A: Wrapping<i32> = Wrapping(0x4c1906);
    pub const B: Wrapping<i32> = Wrapping(0x5ac0db);
    pub const C: Wrapping<i64> = Wrapping(0x4307a7);
    pub const D: Wrapping<i32> = Wrapping(0x5f24f);
    pub const E: u64 = 0x3ad8025f;
}

fn calculate_slime_data(c: &Chunk) -> u64 {
    let x = Wrapping(c.x as i32);
    let z = Wrapping(c.z as i32);
    let a = Wrapping((x * x * slime_const::A).0 as i64);
    let b = Wrapping((x * slime_const::B).0 as i64);
    let c = Wrapping((z * z).0 as i64) * slime_const::C;
    let d = Wrapping((z * slime_const::D).0 as i64);

    (a + b + c + d).0 as u64
}

pub fn is_slime_chunk(seed: u64, c: &Chunk) -> bool {
    let x = calculate_slime_data(c);
    is_slime_data(seed, x)
}

fn is_slime_data(seed: u64, x: u64) -> bool {
    let mut r = rng_with_slime_data(seed, x);
    r.next_int_n(10) == 0
}

fn rng_with_slime_data(seed: u64, x: u64) -> JavaRng {
    let s = seed.wrapping_add(x) ^ slime_const::E;
    JavaRng::with_seed(s)
}

