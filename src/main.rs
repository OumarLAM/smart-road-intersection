mod vehicle;
mod renderer;
mod physics;
mod stats;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::LoadTexture;
use std::time::{Duration, Instant};
use rand::seq::SliceRandom;
use rand::Rng;
use crate::vehicle::Vehicle;
use crate::renderer::Renderer;
use crate::stats::Statistics;

const SPAWN_DELAY: Duration = Duration::from_secs(1);

struct SpawnTracker {
    last_spawn_n: Instant,
    last_spawn_s: Instant,
    last_spawn_e: Instant,
    last_spawn_w: Instant,
}

impl SpawnTracker {
    fn new() -> Self {
        let now = Instant::now();
        SpawnTracker {
            last_spawn_n: now,
            last_spawn_s: now,
            last_spawn_e: now,
            last_spawn_w: now,
        }
    }

    fn can_spawn(&self, route: char) -> bool {
        let now = Instant::now();
        match route {
            'n' => now.duration_since(self.last_spawn_n) >= SPAWN_DELAY,
            's' => now.duration_since(self.last_spawn_s) >= SPAWN_DELAY,
            'e' => now.duration_since(self.last_spawn_e) >= SPAWN_DELAY,
            'w' => now.duration_since(self.last_spawn_w) >= SPAWN_DELAY,
            _ => false,
        }
    }

    fn update_last_spawn(&mut self, route: char) {
        let now = Instant::now();
        match route {
            'n' => self.last_spawn_n = now,
            's' => self.last_spawn_s = now,
            'e' => self.last_spawn_e = now,
            'w' => self.last_spawn_w = now,
            _ => (),
        }
    }
}

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
    let mut spawn_tracker = SpawnTracker::new();

    let mut renderer = Renderer::new(canvas, background_texture, vehicle_texture);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    if spawn_tracker.can_spawn('n') {
                        let (x, y, direction) = spawn_vehicle('n');
                        vehicles.push(Vehicle::new(x, y, 100.0, 50.0, 150.0, 'n', direction));
                        spawn_tracker.update_last_spawn('n');
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    if spawn_tracker.can_spawn('s') {
                        let (x, y, direction) = spawn_vehicle('s');
                        vehicles.push(Vehicle::new(x, y, 100.0, 50.0, 150.0, 's', direction));
                        spawn_tracker.update_last_spawn('s');
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    if spawn_tracker.can_spawn('e') {
                        let (x, y, direction) = spawn_vehicle('e');
                        vehicles.push(Vehicle::new(x, y, 100.0, 50.0, 150.0, 'e', direction));
                        spawn_tracker.update_last_spawn('e');
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    if spawn_tracker.can_spawn('w') {
                        let (x, y, direction) = spawn_vehicle('w');
                        vehicles.push(Vehicle::new(x, y, 100.0, 50.0, 150.0, 'w', direction));
                        spawn_tracker.update_last_spawn('w');
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    let routes = "nsew";
                    let random_char = routes.chars().nth(rand::thread_rng().gen_range(0..routes.len()));
                    match random_char {
                        Some(c) => {
                            if spawn_tracker.can_spawn(c) {
                                let (x, y, direction) = spawn_vehicle(c);
                                vehicles.push(Vehicle::new(x, y, 100.0, 50.0, 150.0, c, direction));
                                spawn_tracker.update_last_spawn(c);
                            } 
                        },
                        None => println!("Cannot found random vehicle direction"),
                    }
                },
                _ => {}
            }
        }

        let dt = 1.0 / 60.0;
        physics::update_vehicles(&mut vehicles, dt);
        physics::check_collisions(&mut vehicles, 50.0, &mut stats);

        renderer.render(&vehicles);
        stats.update(&vehicles);

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    stats.display();
    Ok(())
}

fn spawn_vehicle(route: char) -> (f64, f64, char) {
    let positions = match route {
        's' => [(443.0, 900.0), (491.0, 900.0), (541.0, 900.0)], // South side
        'n' => [(309.0, 0.0), (358.0, 0.0), (407.0, 0.0)], // North side
        'w' => [(0.0, 441.0), (0.0, 491.0), (0.0, 539.0)], // West side
        'e' => [(900.0, 307.0), (900.0, 357.0), (900.0, 405.0)], // East side
        _ => todo!(), // Default center
    };

    let mut rng = rand::thread_rng();
    let &(x, y) = positions.choose(&mut rng).unwrap();
    let direction = random_route(route, x, y);

    (x, y, direction)
}

fn random_route(a: char, x: f64, y: f64) -> char {
    let returned_route = match a {
        's' => match x {
            443.0 => 'l',
            491.0 => 's',
            541.0 => 'r',
            _ => todo!()
        },
        'n' => match x {
            309.0 => 'r',
            358.0 => 's',
            407.0 => 'l',
            _ => todo!()
        },
        'w' => match y {
            441.0 => 'l',
            491.0 => 's',
            539.0 => 'r',
            _ => todo!()
        },
        'e' => match y {
            307.0 => 'r',
            357.0 => 's',
            405.0 => 'l',
            _ => todo!()
        }
        _ => todo!()
    };

    returned_route
}
