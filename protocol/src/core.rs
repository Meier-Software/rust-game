use std::str::FromStr;

use crate::ProtocolError;

#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Facing {
    North,
    East,
    South,
    West,
}

impl std::str::FromStr for Facing {
    type Err = ProtocolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "North" => Ok(Self::North),
            "East" => Ok(Self::East),
            "South" => Ok(Self::South),
            "West" => Ok(Self::West),
            _ => Err(ProtocolError::InvalidFacingDirection),
        }
    }
}

impl std::fmt::Display for Facing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Facing::*;
        let a = match self {
            North => "North".to_string(),
            East => "East".to_string(),
            South => "South".to_string(),
            West => "West".to_string(),
        };
        write!(f, "{}", a)
    }
}

#[test]
fn worth_sorth_sanity_check() {
    use std::str::FromStr;

    let no = Facing::from_str("Worth");
    match no {
        Ok(ok) => panic!("Should have failed."),
        Err(err) => {
            assert_eq!(err, ProtocolError::InvalidFacingDirection)
        }
    }
}
