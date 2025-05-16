trait Vehicle {
    fn name(&self) -> String;
    fn year(&self) -> i32;
}
struct Car {
    name: String,
    year: i32,
}
impl Vehicle for Car {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn year(&self) -> i32 {
        self.year
    }
}
struct Truck {
    name: String,
    year: i32,
}
impl Vehicle for Truck {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn year(&self) -> i32 {
        self.year
    }
}
fn main() {
    let my_car = Car {
        name: String::from("Toyota"),
        year: 2020,
    };
    let my_truck = Truck {
        name: String::from("Ford"),
        year: 2018,
    };
    println!("Car name: {}, Year: {}", my_car.name(), my_car.year());
    println!("Truck name: {}, Year: {}", my_truck.name(), my_truck.year());
}