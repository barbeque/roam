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

pub struct Rect {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

pub fn overlaps(r1: &Rect, r2: &Rect) -> bool {
    overlaps_horizontal(r1.x, r1.width, r2.x, r2.width)
        || overlaps_vertical(r1.y, r1.height, r2.y, r2.height)
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
    fn basic_overlap() {
        let r = Rect {
            x: 0,
            y: 0,
            width: 50,
            height: 50,
        };
        let collides = Rect {
            x: 25,
            y: 25,
            width: 4,
            height: 4,
        };
        let no_overlap = Rect {
            x: 110,
            y: 110,
            width: 50,
            height: 50,
        };

        assert!(overlaps(&r, &collides));
        assert!(!overlaps(&r, &no_overlap));
    }
    #[test]
    fn basic_horizontal_overlap() {
        let r = Rect {
            x: 0,
            y: 0,
            width: 50,
            height: 50,
        };
        let no_overlap_horizontal = Rect {
            x: 110,
            y: 0,
            width: 50,
            height: 50,
        };

        assert!(!overlaps_horizontal(
            r.x,
            r.width,
            no_overlap_horizontal.x,
            no_overlap_horizontal.width
        ));
    }
    #[test]
    fn basic_vertical_overlap() {
        let r = Rect {
            x: 0,
            y: 0,
            width: 50,
            height: 50,
        };
        let no_overlap_vertical = Rect {
            x: 0,
            y: 110,
            width: 50,
            height: 50,
        };

        assert!(!overlaps_vertical(
            r.y,
            r.height,
            no_overlap_vertical.y,
            no_overlap_vertical.height
        ));
    }
    #[test]
    fn pythagorean_distance_is_sane() {
        assert_eq!(distance(-2, -3, -4, 4), 7.28011);
    }
}
