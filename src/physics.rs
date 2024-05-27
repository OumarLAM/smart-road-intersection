use crate::vehicle::Vehicle;
use crate::stats::Statistics;

pub fn update_vehicles(vehicles: &mut Vec<Vehicle>, dt: f64) {
    for vehicle in vehicles.iter_mut() {
        vehicle.update(dt);
    }
}

pub fn check_collisions(vehicles: &mut Vec<Vehicle>, min_distance: f64, stats: &mut Statistics) {
    let len = vehicles.len();

    for i in 0..len {
        for j in (i + 1)..len {
            let distance = ((vehicles[i].x - vehicles[j].x).powi(2) + (vehicles[i].y - vehicles[j].y).powi(2)).sqrt();
            if distance < min_distance && can_collide(&vehicles[i], &vehicles[j]) {
                stats.collisions += 1;
            }
        }
    }
}

fn can_collide(vehicle1: &Vehicle, vehicle2: &Vehicle) -> bool {
    (vehicle1.direction == 's' || vehicle1.direction == 'l') &&
    (vehicle2.direction == 's' || vehicle2.direction == 'l')
}
