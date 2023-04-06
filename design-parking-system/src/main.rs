struct ParkingSystem {
    big: i32,
    medium: i32,
    small: i32,
}

impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        ParkingSystem { big, medium, small }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        if car_type == 1 {
            if self.big == 0 {
                return false;
            }
            self.big -= 1;
            true
        } else if car_type == 2 {
            if self.medium == 0 {
                return false;
            }
            self.medium -= 1;
            true
        } else {
            if self.small == 0 {
                return false;
            }
            self.small -= 1;
            true
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
