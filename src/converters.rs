use std::io;

pub fn converter_celcius_and_fahrenheit() -> Result<f32, &'static str> {
    println!("(1) -> °C to °F | (2) -> °F to °C");
    let mut user_input: String = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    let mode = match user_input.trim().parse::<u8>() {
        Ok(n) => n,
        Err(_) => return Err("Invalid input value"),
    };

    match mode {
        1 | 2 => {}
        _ => return Err("Invalid input value"),
    }

    println!("Input your temperature: ");
    let mut temperature: String = String::new();
    io::stdin().read_line(&mut temperature).unwrap();
    let value = match temperature.trim_end().parse::<f32>() {
        Ok(n) => n,
        Err(_) => return Err("Invalid input value"),
    };

    return match mode {
        1 => Ok(value * 9. / 5. + 32.),// (0 °C × 9/5) + 32 = 32 °F
        2 => Ok((value - 32.) * 5. / 9.),// (0 °F − 32) × 5/9 = -17,78 °C
        _ => Err("Unexpected error!"),
    };
}