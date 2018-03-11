pub fn manhattan_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    let dx = i32::abs(x2 - x1);
    let dy = i32::abs(y2 - y1);
    dx + dy
}

pub fn distance(x1: i32, y1: i32, x2: i32, y2: i32) -> f32 {
    let dx = (x2 - x1) as f32;
    let dy = (y2 - y1) as f32;
    f32::sqrt(dx * dx + dy * dy)
}

pub fn overlaps_horizontal(x1: i32, w1: i32, x2: i32, w2: i32) -> bool {
    let left1 = x1;
    let left2 = x2;
    let right1 = x1 + w1;
    let right2 = x2 + w2;

    if right1 < left2 {
        return false;
    }
    if left1 > right2 {
        return false;
    }
    true
}

pub fn overlaps_vertical(y1: i32, h1: i32, y2: i32, h2: i32) -> bool {
    let top1 = y1;
    let top2 = y2;
    let bottom1 = y1 + h1;
    let bottom2 = y2 + h2;

    if bottom1 < top2 {
        return false;
    }
    if top1 > bottom2 {
        return false;
    }
    true
}

pub fn overlaps(x1: i32, y1: i32, w1: i32, h1: i32,
    x2: i32, y2: i32, w2: i32, h2: i32) -> bool {
    overlaps_horizontal(x1, w1, x2, w2) || overlaps_vertical(y1, h1, y2, h2)
}

#[cfg(test)]
mod tests {
    use coordinate_utils::*;
    #[test]
    fn manhattan_distance_is_sane() {
        assert_eq!(manhattan_distance(2, 2, 0, 0), 4);
        assert_eq!(manhattan_distance(2, -2, 0, 0), 4);
        assert_eq!(manhattan_distance(-2, 2, 0, 0), 4);
        assert_eq!(manhattan_distance(-2, -2, 0, 0), 4);
        assert_eq!(manhattan_distance(0, 0, 0, 0), 0);
    }
    #[test]
    fn pythagorean_distance_is_sane() {
        assert_eq!(distance(-2, -3, -4, 4), 7.28011);
    }
}
