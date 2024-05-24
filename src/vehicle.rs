pub struct Vehicle {
    pub x: f64,
    pub y: f64,
    pub velocity: f64,
    pub route: char,
    pub direction: char,
    pub time: f64,
    pub distance: f64,
    pub angle: f64,
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
            angle: Self::calculate_initial_angle(route),
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.time += dt;
        self.distance += self.velocity * dt;
        match (self.route, self.direction) {
            ('n', 'r') => {
                self.y += self.velocity * dt;
                if self.y > 300.0 {
                    self.angle = 90.0;
                    self.x -= self.velocity * dt;
                    self.y = 300.0;
                }
            },
            ('n', 's') => self.y += self.velocity * dt,
            ('n', 'l') => {
                self.y += self.velocity * dt;
                if self.y > 450.0 {
                    self.angle = 270.0;
                    self.x += self.velocity * dt;
                    self.y = 450.0;
                }
            },
            ('s', 'r') => {
                self.y -= self.velocity * dt;
                if self.y < 550.0 {
                    self.angle = 270.0;
                    self.x += self.velocity * dt;
                    self.y = 550.0;
                }
            },
            ('s', 's') => self.y -= self.velocity * dt,
            ('s', 'l') => {
                self.y -= self.velocity * dt;
                if self.y < 400.0 {
                    self.angle = 90.0;
                    self.x -= self.velocity * dt;
                    self.y = 400.0;
                }
            },
            ('e', 'r') => {
                self.x -= self.velocity * dt;
                if self.x < 550.0 {
                    self.angle = 180.0;
                    self.y -= self.velocity * dt;
                    self.x = 550.0;
                }
            },
            ('e', 's') => self.x -= self.velocity * dt,
            ('e', 'l') => {
                self.x -= self.velocity * dt;
                if self.x < 400.0 {
                    self.angle = 0.0;
                    self.y += self.velocity * dt;
                    self.x = 400.0;
                }
            },
            ('w', 'r') => {
                self.x += self.velocity * dt;
                if self.x > 300.0 {
                    self.angle = 0.0;
                    self.y += self.velocity * dt;
                    self.x = 300.0;
                }
            },
            ('w', 's') => self.x += self.velocity * dt,
            ('w', 'l') => {
                self.x += self.velocity * dt;
                if self.x > 450.0 {
                    self.angle = 180.0;
                    self.y -= self.velocity * dt;
                    self.x = 450.0;
                }
            },
            _ => (),
        }
    }


    fn calculate_initial_angle(route: char) -> f64 {
        match route {
            'n' => 0.0,
            's' => 180.0,
            'e' => 90.0,
            'w' => 270.0,
            _ => 0.0,
        }
    }
}
