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
    SemiAuto,
    Automatic,
}

#[derive(PartialEq, Debug)]
enum Age {
    New,
    Used,
}

fn car_quality(miles: u32) -> (Age, u32) {
    if miles > 0 {
        (Age::Used, miles)
    } else {
        (Age::New, 0)
    }
}

fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
    if miles > 0 {
        if roof {
            println!(
                "Prepare a used car: {:?}, {}, Hard top, {} miles\n",
                motor, color, miles
            );
        } else {
            println!(
                "Prepare a used car: {:?}, {}, Soft top, {} miles\n",
                motor, color, miles
            );
        }
    } else {
        if roof {
            println!(
                "Prepare a new car: {:?}, {}, Hard top, {} miles\n",
                motor, color, miles
            );
        } else {
            println!(
                "Prepare a new car: {:?}, {}, Soft top, {} miles\n",
                motor, color, miles
            );
        }
    }

    Car {
        color,
        motor,
        roof,
        age: car_quality(miles),
    }
}

pub fn main() {
    // Car order #1: New, Manual, Hard top
    car_factory(String::from("Orange"), Transmission::Manual, true, 0);

    // Car order #2: Used, Semi-automatic, Convertible
    car_factory(String::from("Red"), Transmission::SemiAuto, false, 565);

    // Car order #3: Used, Automatic, Hard top
    car_factory(String::from("White"), Transmission::Automatic, true, 3000);
}
