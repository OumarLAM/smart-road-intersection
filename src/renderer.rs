use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use sdl2::rect::{Point, Rect};
use crate::vehicle::Vehicle;

pub struct Renderer<'a> {
    canvas: Canvas<Window>,
    background_texture: Texture<'a>,
    vehicle_texture: Texture<'a>,
}

impl<'a> Renderer<'a> {
    pub fn new(canvas: Canvas<Window>, background_texture: Texture<'a>, vehicle_texture: Texture<'a>) -> Renderer<'a> {
        Renderer {
            canvas,
            background_texture,
            vehicle_texture,
        }
    }

    pub fn render(&mut self, vehicles: &[Vehicle]) {
        self.canvas.clear();
        self.canvas.copy(&self.background_texture, None, None).unwrap();

        for vehicle in vehicles {
            let dst = Rect::new(vehicle.x as i32, vehicle.y as i32, 50, 50);
            let center = Point::new(25, 25);
            self.canvas.copy_ex(&self.vehicle_texture, None, dst, vehicle.angle, center, false, false).unwrap();
        }

        self.canvas.present();
    }
}
