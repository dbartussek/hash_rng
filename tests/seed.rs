use hash_rng::Hash64Gen;
use rand::*;

#[test]
fn seed_is_equal() {
    let seed = rand::thread_rng().next_u64();
    let jump = rand::thread_rng().next_u64();
    dbg!(seed);
    dbg!(jump);

    let mut a = Hash64Gen::seed_from_u64(seed);
    let mut b = Hash64Gen::seed_from_u64(seed);
    a.jump_forward(jump);
    b.jump_forward(jump);

    for i in 0..1000 {
        assert_eq!(a.next_u64(), b.next_u64(), "Divergence at {}", i);
    }
}
