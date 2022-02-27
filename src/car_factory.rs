// Declare Car struct to describe vehicle with four named fields.
struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    milleage: u32,
}
#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

// Build a "Car" by using values from the input arguments
// - color of car (String)
// - Transmission type (enum value)
// - Convertible (boolean, true if car is a convertible)

fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {
    Car {
        color: color,
        transmission: transmission,
        convertible: convertible,
        milleage: 0,
    }
}

pub fn run() {
    // We have orders for three new cars!
    // We'll declare a mutable car variable and reuse it for all teh cars
    let mut car = car_factory(String::from("Red"), Transmission::Automatic, false);
    println!(
        "Car_1 = {}, {:#?} transmission, convertible: {}, milleage: {}",
        car.color, car.transmission, car.convertible, car.milleage
    );

    car = car_factory(String::from("Yellow"), Transmission::SemiAuto, true);
    println!(
        "Car_2 = {}, {:#?} transmission, convertible: {}, milleage: {}",
        car.color, car.transmission, car.convertible, car.milleage
    );

    car = car_factory(String::from("Grey"), Transmission::Manual, false);
    println!(
        "Car_3 = {}, {:#?} transmission, convertible: {}, milleage: {}",
        car.color, car.transmission, car.convertible, car.milleage
    );
}
