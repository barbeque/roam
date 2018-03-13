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
    overlaps_1d(x1, w1, x2, w2)
}

pub fn overlaps_vertical(y1: i32, h1: i32, y2: i32, h2: i32) -> bool {
    // Convenience func, for readability
    overlaps_1d(y1, h1, y2, h2)
}

fn overlaps_1d(start1: i32, length1: i32, start2: i32, length2: i32) -> bool {
    let end1 = start1 + length1;
    let end2 = start2 + length2;

    if end1 < start2 {
        return false;
    }
    if start1 > end2 {
        return false;
    }
    true
}

pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl PartialEq for Rect {
    fn eq(self: &Rect, other: &Rect) -> bool {
        self.x == other.x && self.y == other.y && self.width == other.width && self.height == other.height
    }
}

use std::fmt;
impl fmt::Debug for Rect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rect ( {}, {}: {} x {})", self.x, self.y, self.width, self.height)
    }
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
    fn rect_equality() {
        let r = Rect { x: 13, y: 13, width: 46, height: 306 };
        let q = Rect { x: 14, y: 13, width: 46, height: 306 };
        assert_eq!(r, r);
        assert_ne!(r, q);
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
        assert!(overlaps_horizontal(r.x, r.width, collides.x, collides.width));
        assert!(overlaps_vertical(r.y, r.height, collides.y, collides.height));
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
