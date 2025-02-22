fn main() {
    println!("Hello, world!");
}
use rand::Rng;

pub fn generate_data(size: usize) -> Vec<(f32, f32)> {
    let mut rng = rand::rng();
    (0..size)
        .map(|_| {
            let x = rng.random_range(0.0..10.0);
            let noise = rng.random_range(-0.5..0.5);
            let y = 2.0 * x + 1.0 + noise;
            (x, y)
        })
        .collect()
}
