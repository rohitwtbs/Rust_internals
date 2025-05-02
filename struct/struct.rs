struct Car{
    name: String,
    year: i32,
}

fn main(){
    let my_car = Car{
        name: String::from("Toyota"),
        year: 2020,
    }
    println!("Car name: {}, Year: {}", my_car.name, my_car.year);
}