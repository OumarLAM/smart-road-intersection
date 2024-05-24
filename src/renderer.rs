use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use sdl2::rect::Rect;
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
        self.canvas.copy(&self.background_texture, None, None).unwrap();

        for vehicle in vehicles {
            let angle = vehicle.get_angle();
            let dst = Rect::new(vehicle.x as i32, vehicle.y as i32, 50, 50);
            self.canvas.copy_ex(&self.vehicle_texture, None, dst, angle, None, false, false).unwrap();
        }

        self.canvas.present();
    }
}
