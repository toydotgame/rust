/*
 * AUTHOR: toydotgame
 * CREATED: 2025-11-24
 * REALLY BAD AND BASIC temperature converted (32-bit floats).
 */

use std::io;

fn main() {
    println!("Temperature Converter\n");
    let mut temp = String::new();
    let mut unit = String::new();

    println!("Temperature (without unit):");
    io::stdin()
        .read_line(&mut temp)
        .expect("Couldn't read temperature input!");
    let mut temp: f32 = temp.trim().parse().expect("NaN!");

    println!("Unit (F/C):");
    io::stdin()
        .read_line(&mut unit)
        .expect("Couldn't read unit input!");
    let mut unit: char = unit.trim().parse().expect("Not a char!");
    unit.make_ascii_uppercase();

    if unit == 'C' {
        temp = temp / (5.0 / 9.0) + 32.0;
    } else if unit == 'F' {
        temp = (temp - 32.0) * (5.0 / 9.0);
    } else {
        println!("Invalid unit!");
        return;
    }

    let unit = if unit == 'C' { "F" } else { "C" };
    println!("That's {temp} °{unit}");
}
