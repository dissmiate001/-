// Файл: homeworks/hw11.rs

use rand::Rng;

/// Генерує вектор довжиною n зі значеннями від 10 до 99 включно
pub fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..=99)).collect()
}

/// Знаходить індекс першої пари елементів з мінімальною сумою
pub fn min_adjacent_sum(data: &[i32]) -> Option<(usize, i32)> {
    if data.len() < 2 {
        return None;
    }

    let mut min_sum = data[0] + data[1];
    let mut min_index = 0;

    for i in 1..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = i;
        }
    }

    Some((min_index, min_sum))
}

/// Виводить вектор, підкреслюючи мінімальну пару суміжних значень
pub fn print_with_min_pair(data: &[i32]) {
    if let Some((index, sum)) = min_adjacent_sum(data) {
        for (i, val) in data.iter().enumerate() {
            if i == index {
                print!("|{:>3}", val);
            } else if i == index + 1 {
                print!("{:>3}| ", val);
            } else {
                print!("{:>4}", val);
            }
        }
        println!("\nМінімальна сума: {} + {} = {}", data[index], data[index + 1], sum);
    } else {
        println!("Недостатньо елементів для пошуку пари.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_adjacent_sum() {
        let v = vec![30, 20, 10, 40, 50];
        assert_eq!(min_adjacent_sum(&v), Some((1, 30)));
    }

    #[test]
    fn test_random_vector_length() {
        let v = gen_random_vector(20);
        assert_eq!(v.len(), 20);
        for x in &v {
            assert!((10..=99).contains(x));
        }
    }
}
