fn main() {
    // Take the mod of some large number to a large power
    let value = mod_of_power(324322, 87234632, 3724);
    println!("{} {}", "324322^(87234632) mod 645:", value);
}

/// Get the modulus of an integer raised to a power.
/// **Calculates the modulus using modular exponentiation.**
fn mod_of_power(value: i32, power: i32, modulus: i32) -> i32 {
    let mut x: i32 = 1;
    for i in 0..power {
        x = (x * value) % modulus;
    }
    x
}
