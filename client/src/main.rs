#![feature(maybe_uninit_slice)]

use std::path::PathBuf;

use engine::Engine;

mod engine;
// mod net;

pub fn main() {
    let resource_dir = PathBuf::from("./client/assets");

    let cb = ggez::ContextBuilder::new("simple_game", "ggez")
        .window_setup(ggez::conf::WindowSetup::default().title("Simple 2D Game"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(resource_dir);

    let (mut ctx, event_loop) = cb.build().unwrap();

    let engine = Engine::new(&mut ctx);

    ggez::event::run(ctx, event_loop, engine)
}

// let mut nc = NetClient::new();
// let b = nc.tcp.write(b"register xyz 123\r\n");
// match b {
//    Ok(size) => println!("write size {}", size),
//    Err(err) => println!("err {}", err),
// }

// let mut buf = Vec::new();
// let a = nc.tcp.read(&mut buf);
// match a{
//    Ok(size) => println!("read size {}", size),
//    Err(err) => println!("err {}", err),
// }
// println!("{:?}", buf);
