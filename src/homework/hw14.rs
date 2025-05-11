use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rectangle {
    a: Point,
    b: Point,
}

fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut occupied: HashSet<Point> = HashSet::new();

    for rect in xs {
        let x_start = rect.a.x.min(rect.b.x);
        let x_end = rect.a.x.max(rect.b.x);
        let y_start = rect.a.y.min(rect.b.y);
        let y_end = rect.a.y.max(rect.b.y);

        for x in x_start..x_end {
            for y in y_start..y_end {
                occupied.insert(Point { x, y });
            }
        }
    }

    occupied.len() as i32
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_occupied_test() {
        let data = test_data();
        let occupied = area_occupied(&data);
        assert_eq!(occupied, 60);
    }
}
