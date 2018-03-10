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

#[cfg(test)]
mod tests {
    use coordinate_utils::*;
    #[test]
    fn manhattan_distance_is_sane() {
        assert_eq!(manhattan_distance(2, 2, 0, 0), 4);
    }
    #[test]
    fn pythagorean_distance_is_sane() {
        assert_eq!(distance(-2, -3, -4, 4), 7.28011);
    }
}
