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
    while color > 4 {
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

    // Declare a car as mutable "Car" Struct
    let mut car: Car;
    // Start with zero miles
    let mut miles = 0;

    for order in 1..12 {
        // Call car_factory to fulfill order
        // Add order <K, V> pair to "orders" hash map
        // Call println! to show order details from the hash map

        car = car_factory(order, miles);
        orders.insert(order, car);
        println!("Car order {}: {:?}", order, orders.get(&order));

        // Reset miles for order variety
        if miles == 2100 {
            miles = 0;
        } else {
            miles = miles + 700;
        }
    }
}
