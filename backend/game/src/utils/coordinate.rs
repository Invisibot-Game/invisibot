#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub x: u32,
    pub y: u32,
}

#[macro_export]
macro_rules! coord {
    ( $x:expr, $y:expr ) => {{
        Coordinate { x: $x, y: $y }
    }};
}
