use crate::vehicle::Vehicle;

pub fn update_vehicles(vehicles: &mut Vec<Vehicle>, dt: f64) {
    for vehicle in vehicles.iter_mut() {
        vehicle.update(dt);
    }
}

// pub fn check_collisions(vehicles: &mut Vec<Vehicle>, min_distance: f64, stats: &mut crate::stats::Statistics) {
//     let len = vehicles.len();
//     for i in 0..len {
//         for j in (i + 1)..len {
//             let distance = ((vehicles[i].x - vehicles[j].x).powi(2) + (vehicles[i].y - vehicles[j].y).powi(2)).sqrt();
//             if distance < min_distance {
//                 // Handle collision logic
//                 handle_collision(&mut vehicles[i], &mut vehicles[j]);
//                 stats.collisions += 1;
//             }
//         }
//     }
// }

fn handle_collision(vehicle1: &mut Vehicle, vehicle2: &mut Vehicle) {
    // Stop both vehicles upon collision
    vehicle1.velocity = 0.0;
    vehicle2.velocity = 0.0;
}
