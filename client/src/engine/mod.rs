use net::NetworkingIn;
use net::{NetClient, NetworkingOut};

use event::{Event, EventType};
use ggez::Context;
use protocol::Position;
use specs::{Builder, RunNow, World, WorldExt};

pub mod event;
mod net;
mod render;

pub enum State {
    PreAuth,
    // Represents you as being signed in and in a menu.
    InMenu,
    // Represents you as being signed in and in a game world.
    InGame,
}

pub struct Engine {
    state: State,
    world: World,
}

impl Engine {
    pub fn new(ctx: &mut Context) -> Self {
        let mut world = World::new();
        world.register::<NetClient>();
        world.register::<Event>();
        world.register::<EventType>();

        protocol::world_register(&mut world);

        let pos = Position::new(0.0, 0.0);
        let facing = protocol::Facing::South; // Default facing direction

        // Create player entity with position and facing components
        world.create_entity()
            .with(pos)
            .with(facing)
            .build();

        let nc = NetClient::new();
        world.insert(nc);

        let mut engine = Self {
            state: State::PreAuth,
            world,
        };
        
        // Initialize rendering resources
        engine.init_rendering(ctx);
        
        engine
    }

    // X times per second. TickRate
    pub fn fixed_update(&mut self) {
        // Handle Net Events.
        let mut net_in_system = NetworkingIn {};
        net_in_system.run_now(&self.world);

        match self.state {
            State::PreAuth => {
                let username = "hjk";
                let password = "123";

                let line = format!("register {} {}\r\n", username, password);
                println!("{}", line);
                self.fire_event(EventType::NetSend, line);

                // self.state = State::InGame;

                // TODO: Add in a system to handle auth. If authed move to InGame.
            }
            State::InMenu => {
                todo!();
            }
            State::InGame => {
                // TODO: While InGame render player.
                todo!()
            }
        }

        // Handle Net Events.
        let mut net_system = NetworkingOut {};
        net_system.run_now(&self.world);

        // == ECS Maintain ==
        self.world.maintain();
    }

    pub fn close(&mut self) {}
}

impl Engine {
    pub fn fire_event(&mut self, event_type: EventType, event: String) {
        // println!("fired {:?} - {}", event_type, event);
        let event = Event::new(event);
        self.world
            .create_entity()
            .with(event_type.clone())
            .with(event)
            .build();
    }
}

impl ggez::event::EventHandler<ggez::GameError> for Engine {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        // Update animation timer
        {
            let delta_time = ctx.time.delta().as_secs_f32();
            let mut player_sprite = self.world.write_resource::<render::PlayerSprite>();
            
            // Update animation timer
            player_sprite.animation_timer += delta_time;
            if player_sprite.animation_timer >= render::ANIMATION_FRAME_TIME {
                player_sprite.animation_timer = 0.0;
                
                // Get the number of frames in the sprite sheet
                let sprite_width = player_sprite.sprite.width() as f32;
                let frames_per_row = (sprite_width / render::SPRITE_SHEET_WIDTH) as usize;
                
                // Update animation frame
                player_sprite.animation_frame = (player_sprite.animation_frame + 1) % frames_per_row;
            }
        }
        
        // TODO: Get input from ctx.
        self.fixed_update();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        self.fps_update(ctx);
        Ok(())
    }
}
