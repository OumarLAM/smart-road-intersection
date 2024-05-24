mod vehicle;
mod renderer;
mod physics;
mod stats;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::LoadTexture;
use std::time::Duration;
use rand::seq::SliceRandom;
use crate::vehicle::Vehicle;
use crate::renderer::Renderer;
use crate::stats::Statistics;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(sdl2::image::InitFlag::PNG | sdl2::image::InitFlag::JPG)?;
    
    let window = video_subsystem.window("smart-road", 900, 900)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    
    let canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();
    let background_texture = texture_creator.load_texture("assets/intersection.jpg")?;
    let vehicle_texture = texture_creator.load_texture("assets/vehicle.png")?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut vehicles: Vec<Vehicle> = Vec::new();
    let mut stats = Statistics::new();

    let mut renderer = Renderer::new(canvas, background_texture, vehicle_texture);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    let (x, y, direction) = spawn_vehicle('s');
                    vehicles.push(Vehicle::new(x, y, 100.0, 's', direction));
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    let (x, y, direction) = spawn_vehicle('n');
                    vehicles.push(Vehicle::new(x, y, 100.0, 'n', direction));
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    let (x, y, direction) = spawn_vehicle('e');
                    vehicles.push(Vehicle::new(x, y, 100.0, 'e', direction));
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    let (x, y, direction) = spawn_vehicle('w');
                    vehicles.push(Vehicle::new(x, y, 100.0, 'w', direction));
                },
                _ => {}
            }
        }

        let dt = 1.0 / 60.0;
        physics::update_vehicles(&mut vehicles, dt);
        // physics::check_collisions(&mut vehicles, 50.0, &mut stats);

        renderer.render(&vehicles);
        stats.update(&vehicles);

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    stats.display();
    Ok(())
}

fn spawn_vehicle(route: char) -> (f64, f64, char) {
    let positions = match route {
        's' => [(450.0, 900.0), (500.0, 900.0), (550.0, 900.0)], // South side
        'n' => [(300.0, 0.0), (350.0, 0.0), (400.0, 0.0)], // North side
        'w' => [(0.0, 450.0), (0.0, 500.0), (0.0, 550.0)], // West side
        'e' => [(900.0, 300.0), (900.0, 350.0), (900.0, 400.0)], // East side
        _ => todo!(), // Default center
    };

    let mut rng = rand::thread_rng();
    let &(x, y) = positions.choose(&mut rng).unwrap();
    let direction = random_route();

    (x, y, direction)
}

fn random_route() -> char {
    let routes = ['r', 's', 'l'];
    let mut rng = rand::thread_rng();
    *routes.choose(&mut rng).unwrap()
}
