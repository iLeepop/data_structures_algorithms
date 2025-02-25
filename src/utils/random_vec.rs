use rand::Rng;
pub fn random_vec(len: usize, max: i32, min: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let v = (0..len)
        .map(|_| rng.gen_range(min..=max))
        .collect::<Vec<_>>();
    v
}