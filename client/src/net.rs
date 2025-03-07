use specs::{Join, ReadStorage, System};
use std::{
    io::{Read, Write},
    net::TcpStream,
};

use crate::engine::event::*;

use specs::{Component, prelude::*};

#[derive(Component)]
#[storage(VecStorage)]
pub struct NetClient {
    tcp: TcpStream,
}

impl NetClient {
    pub fn new() -> Self {
        let addr = "localhost:45250";
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
        ReadStorage<'ecs_life, Event>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut a, b) = data;
        unsafe {
            for nc in a.as_mut_slice().assume_init_mut() {
                for event in b.join() {
                    let line = format!("{:?}", event);

                    nc.send(line);
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
        unsafe {

            for nc in a.as_mut_slice().assume_init_mut(){
                let mut line = String::new();
                let ret = nc.tcp.read_to_string(&mut line);
                println!("recv {:?}", ret)
                
            }
        }
       
    }
}
