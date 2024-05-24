use crate::vehicle::Vehicle;
pub struct Statistics {
    max_vehicles: usize,
    max_velocity: f64,
    min_velocity: f64,
    max_time: f64,
    min_time: f64,
    close_calls: usize,
}

impl Statistics {
    pub fn new() -> Statistics {
        Statistics {
            max_vehicles: 0,
            max_velocity: 0.0,
            min_velocity: f64::MAX,
            max_time: 0.0,
            min_time: f64::MAX,
            close_calls: 0,
        }
    }

    pub fn update(&mut self, vehicles: &[Vehicle]) {
        self.max_vehicles = vehicles.len().max(self.max_vehicles);
        for vehicle in vehicles {
            self.max_velocity = vehicle.velocity.max(self.max_velocity);
            self.min_velocity = vehicle.velocity.min(self.min_velocity);
            self.max_time = vehicle.time.max(self.max_time);
            self.min_time = vehicle.time.min(self.min_time);
        }
    }

    // pub fn increment_close_calls(&mut self) {
    //     self.close_calls += 1;
    // }

    pub fn display(&self) {
        println!("Max vehicles: {}", self.max_vehicles);
        println!("Max velocity: {}", self.max_velocity);
        println!("Min velocity: {}", self.min_velocity);
        println!("Max time: {}", self.max_time);
        println!("Min time: {}", self.min_time);
        println!("Close calls: {}", self.close_calls);
    }
}
