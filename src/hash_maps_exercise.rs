#[derive(PartialEq, Debug)]
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32),
}

#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAutomatic,
    Automatic,
}

#[derive(PartialEq, Debug)]
enum Age {
    Used,
    New,
}

// Get the car quality by testing the value of the input argument
// - miles (u32)
// Return tuple with car age  ("News" or "Used") and milleage

fn car_quality(miles: u32) -> (Age, u32) {
    // Check if car has acumulate miles
    // Return tuple early for Used car
    if miles > 0 {
        return (Age::Used, miles);
    }
    (Age::New, miles)
}

// Build "Car using input arguments
fn car_factory(order: i32, miles: u32) -> Car {
    let colors = ["Blue", "Green", "Red", "Silver"];
    // Prevent panic: Check color index for colors array, reset as needed
    // Valid color = 1, 2, 3 or 4
    // if color > 4, reduce color to valid index
    let mut color = order as usize;
    if color > 4 {
        // color = 5 --> index 1, 6 --> 2, 7 --> 3, 8 --> 4
        color = color - 4
    }
    // Add variety to orders for motor type and roof type
    let mut motor = Transmission::Manual; // 1, 5, 7, 11
    let mut roof = true;

    if order % 3 == 0 {
        // 3, 6, 9
        motor = Transmission::Automatic;
    } else if order % 2 == 0 {
        // 2, 4, 8, 10
        motor = Transmission::SemiAutomatic;
        roof = false;
    }

    // Return requested "Car"
    Car {
        color: String::from(colors[(color - 1) as usize]),
        motor: motor,
        roof: roof,
        age: car_quality(miles),
    }
}

pub fn run() {
    // Initialize a hash map for teh car orders
    // - Key: Car order number, i32
    // - Value: Car order details, Car Struct
    use std::collections::HashMap;
    let mut orders: HashMap<i32, Car> = HashMap::new();

    // Initialize counter variable
    let mut order = 1;
    // Declare a car as mutable "Car" struct
    let mut car: Car;
    // Order 6 cars, increment "order" for each reques

    // Car order #1: Used, Hard Top
    car = car_factory(order, 1000);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));

    // Car order #2: Used, Convertible
    order = order + 1;
    car = car_factory(order, 2000);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));

    // Car order #3: New, Hard Top
    order = order + 1;
    car = car_factory(order, 0);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));

    // Car order #4: New, Convertible
    order = order + 1;
    car = car_factory(order, 0);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));

    // Car order #5: Used, Hard Top
    order = order + 1;
    car = car_factory(order, 3000);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));

    // Car order #6: Used, Hard Top
    order = order + 1;
    car = car_factory(order, 4000);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));
}
