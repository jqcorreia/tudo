pub fn find_previous_power_of_two(x: u32) -> u32 {
    let s = (x as f32).log2();

    2_u32.pow(s.trunc() as u32)
}

pub fn find_next_power_of_two(x: u32) -> u32 {
    let s = (x as f32).log2();

    2_u32.pow((s.trunc() + 1.0) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn math_power_or_two() {
        let result = find_previous_power_of_two(37);
        assert_eq!(result, 32);
    }

    #[test]
    fn math_power_or_two_exact() {
        let result = find_previous_power_of_two(32);
        assert_eq!(result, 32);
    }

    #[test]
    fn math_next_power_or_two() {
        let result = find_next_power_of_two(67);
        assert_eq!(result, 128);
    }
}
