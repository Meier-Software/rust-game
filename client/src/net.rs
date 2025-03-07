use std::{
    io::{Read, Write},
    net::TcpStream,
};
use crate::engine::event::*;

use specs::prelude::*;
use specs::{Component, prelude::*};

#[derive(Component)]
#[storage(VecStorage)]
pub struct NetClient {
    tcp: TcpStream,
}

impl NetClient {
    pub fn new() -> Self {
        let addr = "game.ablecorp.us:45250";
        let stream = TcpStream::connect(addr).unwrap();

        Self { tcp: stream }
    }
    pub fn send(&mut self, string: String) {
        println!("Net event sent.");
        let a = format!("{}", string);
        let _ = self.tcp.write(a.as_bytes());
    }
}

pub struct NetworkingOut {}

impl<'ecs_life> System<'ecs_life> for NetworkingOut {
    // SystemData is what you are requesting from the world.
    type SystemData = (
        WriteStorage<'ecs_life, NetClient>,
        ReadStorage<'ecs_life, EventType>,
        ReadStorage<'ecs_life, Event>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut net_client, event_type, event) = data;
        for nc in (&mut net_client).join(){
            println!("NC");
            for (event_type, event) in (&event_type, &event).join(){
                if *event_type == EventType::NetSend {
                    println!("sending line");
                    let line = format!("{}", event.event);
                    println!("sent line");

                    nc.send(line);
                }else{
                    println!("Other event type")
                }
            }

        }

    }
}



pub struct NetworkingIn {}

impl<'ecs_life> System<'ecs_life> for NetworkingIn {
    // SystemData is what you are requesting from the world.
    type SystemData = (WriteStorage<'ecs_life, NetClient>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut a) = data;
        // unsafe {

        //     for nc in a.as_mut_slice().assume_init_mut(){
        //         let mut line = String::new();
        //         let ret = nc.tcp.read_to_string(&mut line);
        //         println!("recv {:?}", ret)
                
        //     }
        // }
       
    }
}
