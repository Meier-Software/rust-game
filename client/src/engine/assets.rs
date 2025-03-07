

use ggez::graphics::Image;
use specs::Component;
use specs::prelude::*;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Asset {
 pub img: Image,
}