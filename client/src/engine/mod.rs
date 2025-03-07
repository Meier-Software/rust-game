use assets::Asset;
use net::NetworkingIn;
use net::{NetClient, NetworkingOut};

use event::{Event, EventType};
use ggez::Context;
use protocol::{Facing, Position};
use render::RenderFrame;
use specs::{Builder, RunNow, World, WorldExt};

pub mod event;
mod net;
mod render;
mod assets;

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
        world.register::<Asset>();

        protocol::world_register(&mut world);

        let pos = Position::new(0.0, 0.0);
        let facing = Facing::North;
        

        world.create_entity()
        .with(pos)
        .with(facing)
        .build();

        let nc = NetClient::new();
        world.insert(nc);

        Self {
            state: State::PreAuth,
            world,
        }
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

        let mut render_frame = RenderFrame;
        render_frame.run_now(&self.world);


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
    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        // TODO: Get input from ctx.
        self.fixed_update();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        self.fps_update(ctx);
        Ok(())
    }
}
