pub struct Vehicle {
    pub x: f64,
    pub y: f64,
    pub velocity: f64,
    pub initial_velocity: f64,
    pub slow_velocity: f64,
    pub fast_velocity: f64,
    pub route: char,
    pub direction: char,
    pub time: f64,
    pub distance: f64,
    pub angle: f64,
}

impl Vehicle {
    pub fn new(x: f64, y: f64, initial_velocity: f64, slow_velocity: f64, fast_velocity: f64, route: char, direction: char) -> Vehicle {
        Vehicle {
            x,
            y,
            velocity: initial_velocity,
            initial_velocity,
            slow_velocity,
            fast_velocity,
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
        
        // Adjust velocity based on position
        if self.is_approaching_intersection() {
            self.velocity = self.slow_velocity;
        } else if self.is_inside_intersection() {
            self.velocity = self.fast_velocity;
        } else {
            self.velocity = self.initial_velocity;
        }

        match (self.route, self.direction) {
            ('n', 'r') => {
                self.y += self.velocity * dt;
                if self.y > 300.0 {
                    self.angle = 90.0;
                    self.x -= self.velocity * dt;
                    self.y = 307.0;
                }
            },
            ('n', 's') => self.y += self.velocity * dt,
            ('n', 'l') => {
                self.y += self.velocity * dt;
                if self.y > 441.0 {
                    self.angle = 270.0;
                    self.x += self.velocity * dt;
                    self.y = 441.0;
                }
            },
            ('s', 'r') => {
                self.y -= self.velocity * dt;
                if self.y < 539.0 {
                    self.angle = 270.0;
                    self.x += self.velocity * dt;
                    self.y = 539.0;
                }
            },
            ('s', 's') => self.y -= self.velocity * dt,
            ('s', 'l') => {
                self.y -= self.velocity * dt;
                if self.y < 405.0 {
                    self.angle = 90.0;
                    self.x -= self.velocity * dt;
                    self.y = 405.0;
                }
            },
            ('e', 'r') => {
                self.x -= self.velocity * dt;
                if self.x < 541.0 {
                    self.angle = 180.0;
                    self.y -= self.velocity * dt;
                    self.x = 541.0;
                }
            },
            ('e', 's') => self.x -= self.velocity * dt,
            ('e', 'l') => {
                self.x -= self.velocity * dt;
                if self.x < 405.0 {
                    self.angle = 0.0;
                    self.y += self.velocity * dt;
                    self.x = 405.0;
                }
            },
            ('w', 'r') => {
                self.x += self.velocity * dt;
                if self.x > 307.0 {
                    self.angle = 0.0;
                    self.y += self.velocity * dt;
                    self.x = 307.0;
                }
            },
            ('w', 's') => self.x += self.velocity * dt,
            ('w', 'l') => {
                self.x += self.velocity * dt;
                if self.x > 447.0 {
                    self.angle = 180.0;
                    self.y -= self.velocity * dt;
                    self.x = 447.0;
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

    fn is_approaching_intersection(&self) -> bool {
        match self.route {
            'n' => self.y > 250.0 && self.y < 300.0,
            's' => self.y < 600.0 && self.y > 540.0,
            'e' => self.x < 600.0 && self.x > 540.0,
            'w' => self.x > 250.0 && self.x < 300.0,
            _ => false,
        }
    }

    fn is_inside_intersection(&self) -> bool {
        match self.route {
            'n' | 's' => self.y > 300.0 && self.y < 540.0,
            'e' | 'w' => self.x > 300.0 && self.x < 540.0,
            _ => false,
        }
    }
}
