// Файл: homeworks/hw07.rs

pub fn swap_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_lowercase() {
                c.to_uppercase().to_string()
            } else if c.is_uppercase() {
                c.to_lowercase().to_string()
            } else {
                c.to_string()
            }
        })
        .collect()
}

// Тестова функція для перевірки
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_case() {
        assert_eq!(swap_case("Hello, World!"), "hELLO, wORLD!");
        assert_eq!(swap_case("RuSt"), "rUsT");
        assert_eq!(swap_case("123abcABC"), "123ABCabc");
    }
}



// Файл: homeworks/hw07.rs

pub fn switch_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_lowercase() {
                c.to_ascii_uppercase()
            } else if c.is_uppercase() {
                c.to_ascii_lowercase()
            } else {
                c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_switch_case() {
        assert_eq!(switch_case("HeLLo WoRLd!"), "hEllO wOrlD!");
        assert_eq!(switch_case("RUST"), "rust");
        assert_eq!(switch_case("rust"), "RUST");
        assert_eq!(switch_case("123!@#"), "123!@#");
    }
}
