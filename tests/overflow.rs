use hash_rng::Hash64Gen;
use rand::*;

#[test]
fn overflow() {
    let mut overflow_rng = Hash64Gen {
        state: u64::MAX,
        seed: 0,
    };

    overflow_rng.next_u64();
    overflow_rng.next_u64();
    overflow_rng.next_u64();

    dbg!(overflow_rng);

    let mut jump_rng = Hash64Gen {
        state: u64::MAX,
        seed: 0,
    };

    jump_rng.jump_forward(10);
    jump_rng.next_u64();

    dbg!(jump_rng);
}

#[test]
fn underflow() {
    let mut underflow_rng = Hash64Gen { state: 0, seed: 0 };

    underflow_rng.jump_backwards(10);
    underflow_rng.next_u64();

    dbg!(underflow_rng);
}
