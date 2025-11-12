use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::Read;
use std::ops::{Add, Div, Mul};
use std::path::Path;

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.x, self.y)
    }
}
impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Div for Point {
    type Output = Point;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl Mul for Point {
    type Output = Point;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x - self.y * rhs.y,
            y: self.x * rhs.y + self.y * rhs.x,
        }
    }
}
#[derive(Copy, Clone, Default, PartialEq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}
impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

pub fn read_file(file_name: &str) -> String {
    let mut contents_string = String::new();
    {
        let mut file = File::open(Path::new(file_name)).expect(&format!("Input file {} does not exist", file_name));
        file.read_to_string(&mut contents_string).expect("could not read file");
    }
    contents_string
}
