pub struct Vehicle {
    pub x: f64,
    pub y: f64,
    pub velocity: f64,
    pub route: char,
    pub direction: char,
    pub time: f64,
    pub distance: f64,
}

impl Vehicle {
    pub fn new(x: f64, y: f64, velocity: f64, route: char, direction: char) -> Vehicle {
        Vehicle {
            x,
            y,
            velocity,
            route,
            direction,
            time: 0.0,
            distance: 0.0,
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.time += dt;
        self.distance += self.velocity * dt;
        match (self.route, self.direction) {
            ('n', 'r') => self.y += self.velocity * dt, // North, turning right
            ('n', 's') => self.y += self.velocity * dt, // North, going straight
            ('n', 'l') => self.y += self.velocity * dt, // North, turning left
            ('s', 'r') => self.y -= self.velocity * dt, // South, turning right
            ('s', 's') => self.y -= self.velocity * dt, // South, going straight
            ('s', 'l') => self.y -= self.velocity * dt, // South, turning left
            ('e', 'r') => self.x -= self.velocity * dt, // East, turning right
            ('e', 's') => self.x -= self.velocity * dt, // East, going straight
            ('e', 'l') => self.x -= self.velocity * dt, // East, turning left
            ('w', 'r') => self.x += self.velocity * dt, // West, turning right
            ('w', 's') => self.x += self.velocity * dt, // West, going straight
            ('w', 'l') => self.x += self.velocity * dt, // West, turning left
            _ => (),
        }
    }

    pub fn get_angle(&self) -> f64 {
        match self.route {
            'e' => 90.0,
            'w' => 270.0,
            's' => 180.0,
            'n' => 0.0,
            _ => 0.0,
        }
    }
}
