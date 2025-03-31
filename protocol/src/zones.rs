use crate::Position;
use crate::ProtocolError;
use std::str::FromStr;
use std::fmt;

// This is a teleportation link to be used by doors. hub/room1@x20y30
#[derive(Debug)]
pub struct ZoneLink {
    // A slash separated list of zones
    pub zones: Vec<String>,
    pub pos: Position,
}

impl fmt::Display for ZoneLink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@x{}y{}", self.zones.join("/"), self.pos.x, self.pos.y)
    }
}

impl FromStr for ZoneLink {
    type Err = ProtocolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (zones_str, pos_str) = s.split_once("@").ok_or(ProtocolError::ServerLineUnparsable)?;
        
        // Parse position string in format "x20y30"
        let (x_str, y_str) = pos_str.split_once("y").ok_or(ProtocolError::ServerLineUnparsable)?;
        
        let x = x_str.trim_start_matches("x")
            .parse::<i32>()
            .map_err(|_| ProtocolError::ServerLineUnparsable)?;
        let y = y_str.parse::<i32>()
            .map_err(|_| ProtocolError::ServerLineUnparsable)?;

        Ok(Self {
            zones: zones_str.split('/').map(String::from).collect(),
            pos: Position::new(x, y),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_zone_link() {
        let link = ZoneLink::from_str("hub@x20y30").expect("Invalid zone link format");
        assert_eq!(link.zones, vec!["hub"]);
        assert_eq!(link.pos.x, 20);
        assert_eq!(link.pos.y, 30);
    }

    #[test]
    fn test_zone_link_with_multiple_zones() {
        let link = ZoneLink::from_str("hub/room1@x20y30").expect("Invalid zone link format");
        assert_eq!(link.zones, vec!["hub", "room1"]);
        assert_eq!(link.pos.x, 20);
        assert_eq!(link.pos.y, 30);
    }

    #[test]
    fn test_zone_link_with_negative_coordinates() {
        let link = ZoneLink::from_str("dungeon/level1@x-10y-5").expect("Invalid zone link format");
        assert_eq!(link.zones, vec!["dungeon", "level1"]);
        assert_eq!(link.pos.x, -10);
        assert_eq!(link.pos.y, -5);
    }

    #[test]
    fn test_zone_link_with_large_coordinates() {
        let link = ZoneLink::from_str("world/area1/room2@x1000y2000").expect("Invalid zone link format");
        assert_eq!(link.zones, vec!["world", "area1", "room2"]);
        assert_eq!(link.pos.x, 1000);
        assert_eq!(link.pos.y, 2000);
    }

    #[test]
    #[should_panic(expected = "Invalid zone link format: ServerLineUnparsable")]
    fn test_invalid_format_no_at() {
        ZoneLink::from_str("invalid").expect("Invalid zone link format");
    }

    #[test]
    #[should_panic(expected = "Invalid zone link format: ServerLineUnparsable")]
    fn test_invalid_position_format() {
        ZoneLink::from_str("zone@invalid").expect("Invalid zone link format");
    }

    #[test]
    #[should_panic(expected = "Invalid zone link format: ServerLineUnparsable")]
    fn test_invalid_x_coordinate() {
        ZoneLink::from_str("zone@abc20y30").expect("Invalid zone link format");
    }

    #[test]
    #[should_panic(expected = "Invalid zone link format: ServerLineUnparsable")]
    fn test_invalid_y_coordinate() {
        ZoneLink::from_str("zone@x20yabc").expect("Invalid zone link format");
    }
}
