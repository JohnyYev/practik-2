#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug)]
struct Rectangle {
    a: Point, // top-left
    b: Point, // bottom-right
}

fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    // Collect and sort unique x-coordinates
    let mut xs_coords: Vec<i32> = xs.iter().flat_map(|r| vec![r.a.x, r.b.x]).collect();
    xs_coords.sort_unstable();
    xs_coords.dedup();

    let mut area = 0;
    for i in 0..xs_coords.len() - 1 {
        let x_left = xs_coords[i];
        let x_right = xs_coords[i + 1];
        let dx = x_right - x_left;
        if dx <= 0 {
            continue;
        }
        // Gather y-intervals of rectangles covering this vertical slab
        let mut ys: Vec<(i32, i32)> = xs
            .iter()
            .filter_map(|r| {
                if r.a.x < x_right && r.b.x > x_left {
                    // rectangle intersects slab
                    Some((r.b.y, r.a.y))
                } else {
                    None
                }
            })
            .collect();
        if ys.is_empty() {
            continue;
        }
        // Sort intervals by lower y
        ys.sort_unstable_by_key(|&(y1, _)| y1);
        // Merge overlapping intervals and sum their heights
        let mut merged_height = 0;
        let (mut cur_start, mut cur_end) = ys[0];
        for &(y1, y2) in ys.iter().skip(1) {
            if y1 > cur_end {
                merged_height += cur_end - cur_start;
                cur_start = y1;
                cur_end = y2;
            } else {
                cur_end = cur_end.max(y2);
            }
        }
        merged_height += cur_end - cur_start;
        area += dx * merged_height;
    }
    area
}

// Sample test data
fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle { a: Point { x: 2, y: 9 }, b: Point { x: 5, y: 3 } },
        Rectangle { a: Point { x: 1, y: 8 }, b: Point { x: 11, y: 6 } },
        Rectangle { a: Point { x: 9, y: 10 }, b: Point { x: 13, y: 2 } },
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
