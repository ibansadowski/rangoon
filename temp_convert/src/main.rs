use std::io;

fn main() {
    println!("Hello :3");

    loop {
        println!("Please input your origin units (C or F): ");

        let mut unit = String::new();

        io::stdin()
            .read_line(&mut unit)
            .expect("Failed to real line");

        let unit: char = match unit.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please try a valid input");
                continue
            },
        };

        if unit == 'c' || unit == 'C' {
            loop {
                println!("Input your temperature in Celsius, or 'break' to switch units: ");
                let mut temp = String::new();

                io::stdin()
                    .read_line(&mut temp)
                    .expect("Failed to real line");

                let temp: f32 = match temp.trim().parse() {
                    Ok(num) => num,
                    Err(_) => break,
                };

                let f_temp = {let temp = temp * 9.0 / 5.0; temp + 32.0};

                println!("Your temperature in Fahrenheit is {f_temp}");

            }
        }

        else if unit == 'f' || unit == 'F' {
            loop {
                println!("Input your temperature in Fahrenheit, or 'break' to switch units: ");
                let mut temp = String::new();

                io::stdin()
                    .read_line(&mut temp)
                    .expect("Failed to real line");

                let temp: f32 = match temp.trim().parse() {
                    Ok(num) => num,
                    Err(_) => break,
                };

                let c_temp = {let temp = temp - 32.0; temp * 5.0 / 9.0};

                println!("Your temperature in Celsius is {c_temp}");

            }
        }
        else {
            println!("INVALID!!");
            continue
        }
        
    }
}
