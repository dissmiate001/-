// src/homework/hw12.rs

use rand::Rng;

pub fn count_permutation(shipments: &Vec<u32>) -> Option<usize> {
    let total: u32 = shipments.iter().sum();
    let len = shipments.len() as u32;
    if total % len != 0 {
        return None;
    }

    let average = total / len;
    let mut moves = 0;

    for &load in shipments {
        if load > average {
            moves += (load - average) as usize;
        }
    }

    Some(moves)
}

pub fn gen_shipments(n: usize) -> Vec<u32> {
    let avg = rand::thread_rng().gen_range(5..20);
    let mut shipments = vec![avg; n];

    for i in 0..(n / 2) {
        let delta = rand::thread_rng().gen_range(1..avg.min(10));
        shipments[i] += delta;
        shipments[n - 1 - i] -= delta;
    }

    shipments
}

// Приклад використання
fn main() {
    let example = vec![9, 3, 7, 2, 9];
    println!("Shipments: {:?}", example);

    match count_permutation(&example) {
        Some(moves) => println!("Minimum moves required: {}", moves),
        None => println!("Equal distribution is not possible."),
    }

    let generated = gen_shipments(6);
    println!("Generated valid shipment: {:?}", generated);
}
