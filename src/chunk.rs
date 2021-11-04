use std::convert::TryFrom;
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Chunk {
    pub x: i64,
    pub z: i64,
}

impl Chunk {
    pub fn new(x: i64, z: i64) -> Chunk {
        Chunk { x, z }
    }
    /// Panics if the chunk coordinates do not fit in i32
    pub fn from_point(Point { x, z }: Point) -> Chunk {
        // x / 16 is not correct because it rounds x=-1 to 0 instead of -1
        // (x - 15) / 16 is not correct because it rounds x=16 to 0 instead of 1
        // Euclidian division by 16 would be correct, but we just use a simple bit shift
        Chunk {
            x: i64::try_from(x >> 4).unwrap(),
            z: i64::try_from(z >> 4).unwrap(),
        }
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.x, self.z)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub x: i64,
    pub z: i64,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point2 {
    pub x: i64,
    pub z: i64,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point4 {
    pub x: i64,
    pub z: i64,
}

impl Point {
    pub fn into_quarter_scale(self) -> Option<Point4> {
        fn divide_by_4(x: i64) -> i64 {
            (x - 2) / 4
        }

        if self.x as u8 % 4 == 2 && self.z as u8 % 4 == 2 {
            Some(Point4 { x: divide_by_4(self.x), z: divide_by_4(self.z)})
        } else {
            None
        }
    }
}

impl Point4 {
    pub fn into_full_resolution(self) -> Point {
        fn multiply_by_4(x: i64) -> i64 {
            (x * 4) + 2
        }

        Point { x: multiply_by_4(self.x), z: multiply_by_4(self.z) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_from_coordinates_x() {
        for i in 0-32..=15-32 {
            assert_eq!(Chunk::from_point(Point { x: i, z: 0 }), Chunk { x: -2, z: 0 });
        }
        for i in 0-16..=15-16 {
            assert_eq!(Chunk::from_point(Point { x: i, z: 0 }), Chunk { x: -1, z: 0 });
        }
        for i in 0..=15 {
            assert_eq!(Chunk::from_point(Point { x: i, z: 0 }), Chunk { x: 0, z: 0 });
        }
        for i in 0+16..=15+16 {
            assert_eq!(Chunk::from_point(Point { x: i, z: 0 }), Chunk { x: 1, z: 0 });
        }
        for i in 0+32..=15+32 {
            assert_eq!(Chunk::from_point(Point { x: i, z: 0 }), Chunk { x: 2, z: 0 });
        }
    }
}