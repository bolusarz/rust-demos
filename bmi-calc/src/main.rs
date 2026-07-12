use std::io;

fn main() {
    println!("Enter your weight: ");

    let weight: f64 = match get_input_as_f64() {
        Some(num) => num,
        None => {
            eprintln!("❌ Invalid value for weight");

            return;
        }
    };

    println!("Enter your height: ");

    let height: f64 = match get_input_as_f64() {
        Some(num) => num,
        None => {
            eprintln!("❌ Invalid value for height");

            return;
        }
    };

    if height == 0.0 {
        eprintln!("Height can not be zero");

        return;
    }

    let bmi = calculate_bmi(weight, height);

    classify_bmi(bmi);
}

fn get_input_as_f64() -> Option<f64> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("❌ Failed to read input.");

    match input.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => None,
    }
}

fn calculate_bmi(weight: f64, height: f64) -> f64 {
    weight / (height * height)
}

fn classify_bmi(bmi: f64) {
    if bmi < 18.5 {
        println!("You're underweight. Eat more!");
    } else if bmi < 24.9 {
        println!("You are doing well.");
    } else if bmi < 29.9 {
        println!("You're overweight. Eat less!");
    } else {
        println!("You're obese. You need to seriously think about your lifestyle.");
    }
}
