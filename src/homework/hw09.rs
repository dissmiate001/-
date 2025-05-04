// Файл: homeworks/hw09.rs

pub fn rotate(s: String, n: isize) -> String {
    let len = s.len();
    if len == 0 {
        return s;
    }

    // Нормалізуємо зміщення в межах [0, len)
    let shift = ((n % len as isize + len as isize) % len as isize) as usize;

    // Розділяємо рядок на дві частини та об'єднуємо їх у новому порядку
    format!("{}{}", &s[shift..], &s[..shift])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "abcdefgh";
        let shifts = [
            (0, "abcdefgh"),
            (8, "abcdefgh"),
            (1, "bcdefgha"),
            (2, "cdefghab"),
            (10, "cdefghab"),
            (-1, "habcdefg"),
            (-2, "ghabcdef"),
            (-10, "ghabcdef"),
        ];

        shifts
            .iter()
            .for_each(|(n, exp)| {
                assert_eq!(
                    rotate(s.to_string(), *n),
                    exp.to_string()
                )
            });
    }
}
