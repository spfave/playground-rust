fn main() {
    // Create car color array
    let color = ["Blue", "Green", "Red", "Silver"];

    // Declare the car type and initial values
    let mut car: Car;
    let mut engine: Transmission = Transmission::Manual;

    // Order 3 cars, one car for each type of transmission
    // Car order #1: New, Manual, Hard top
    car = car_factory(String::from(color[0]), engine, true, 0);
    // println!(
    //     "Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles",
    //     car.age.0, car.roof, car.motor, car.color, car.age.1
    // );

    // Car order #2: Used, Semi-automatic, Convertible
    engine = Transmission::SemiAuto;
    car = car_factory(String::from(color[3]), engine, false, 100);
    // println!(
    //     "Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles",
    //     car.age.0, car.roof, car.motor, car.color, car.age.1
    // );

    // Car order #3: Used, Automatic, Hard top
    engine = Transmission::Automatic;
    car = car_factory(String::from(color[1]), engine, true, 200);
    // println!(
    //     "Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles",
    //     car.age.0, car.roof, car.motor, car.color, car.age.1
    // );
}

// LESSON 4.4: Work with compound types
// LESSON 4.6: Work with if/else conditions

#[derive(PartialEq, Debug)]
// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32), // mileage: u32, // todo!("Move `mileage: u32` field into `age` field - a tuple with two fields: an `Age` enum, u32");
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

#[derive(PartialEq, Debug)]
enum Age {
    New,
    Used,
}

// Get the car quality by testing the value of the input argument - miles (u32)
// Create a tuple for the car quality with the Age ("New" or "Used") and mileage
fn car_quality(miles: u32) -> (Age, u32) {
    // Declare and initialize the return tuple value
    // For a new car, set the miles to 0
    // let quality: (Age, u32) = (Age::New, miles);

    if miles > 0 {
        return (Age::Used, miles);
    }
    (Age::New, miles)

    // Return the completed tuple to the caller
    // quality
}

// Build a new "Car" using the values of four input arguments
// - color (String)
// - motor (Transmission enum)
// - roof (boolean, true if the car has a hard top roof)
// - miles (u32)
// Call the car_quality(miles) function to get the car age
// Return an instance of a "Car" struct with the arrow `->` syntax
fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
    // Show details about car order
    // - Check if order is for Used or New car, then check the roof type
    // - Print details for New or Used car based on roof type
    if car_quality(miles).0 == Age::Used {
        if roof {
            println!(
                "Prepare a used car: {:?}, {}, Hard top, {} miles",
                motor, color, miles
            );
        } else {
            println!(
                "Prepare a used car: {:?}, {}, Convertible, {} miles",
                motor, color, miles
            );
        }
    } else {
        if roof {
            println!(
                "Prepare a new car: {:?}, {}, Hard top, {} miles",
                motor, color, miles
            );
        } else {
            println!(
                "Prepare a new car: {:?}, {}, Convertible, {} miles",
                motor, color, miles
            );
        }
    }
    // print_car_order()

    // Create a new "Car" instance as requested
    // - Bind first three fields to values of input arguments
    // - "age" field calls "car_quality" function with "miles" input argument
    Car {
        color: color,
        motor: motor,
        roof: roof,
        age: car_quality(miles),
    }
}

// fn print_car_order(car: Car) {
//     let top = if car.roof { "Hard top" } else { "Convertible" };
//     println!(
//         "Prepare a {:?} car: {:?}, {}, {}, {} miles",
//         car.age.0, car.motor, car.color, top, car.age.1
//     );
// }
