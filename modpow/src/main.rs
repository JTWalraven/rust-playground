fn main() {
    // Loop 100 times
    for i in 1..10 {
        // Take the mod of number to large power
        let pow = i * 10000000;
        let value = mod_of_power(324322, pow, 3724);
        println!("{}{}{} {}", "324322^(", pow, ") mod 3724 =", value);
    }
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

#[cfg(test)]
mod tests {
    use super::mod_of_power;

    #[test]
    fn mod_of_power_zero() {
        let value = mod_of_power(3, 0, 5);
        assert_eq!(1, value);
    }

    #[test]
    fn mod_of_power_known_power() {
        let value = mod_of_power(5, 3, 7);
        assert_eq!(6, value);
    }
}
