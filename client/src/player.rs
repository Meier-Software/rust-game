use protocol::Position;

pub struct Player {
 name: String,
 pos: Position
}

pub struct Players {
    self_player: Player,
    players: Vec<Player>
}

