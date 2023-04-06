struct ParkingSystem {
    spaces: Vec<i32>,
}

impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        ParkingSystem {
            spaces: vec![big, medium, small],
        }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        let index = (car_type - 1) as usize;
        if self.spaces[index] > 0 {
            self.spaces[index] -= 1;
            true
        } else {
            false
        }
    }
}

fn main() {
    let mut parking_system = ParkingSystem::new(1, 1, 0);
    println!("{}", parking_system.add_car(1));
    println!("{}", parking_system.add_car(2));
    println!("{}", parking_system.add_car(3));
    println!("{}", parking_system.add_car(1));
}
