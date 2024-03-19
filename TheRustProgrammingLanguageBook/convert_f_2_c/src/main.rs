fn main() {
    println!("Convert fahrenheit to celsius");
    let mut fahrenheit = String::new();
    
    println!("Enter fahrenheit temperature:");
    std::io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read line");

    let fahrenhit: Vec<&str> = fahrenheit.trim().split(' ').collect();
    let mut num: Vec<f64> = Vec::new();
    //let fahrenheit = fahrenhit[0].parse::<f64>().unwrap();
    for f in fahrenhit {
        let f = f.parse::<f64>().unwrap();
        let celsius = convert_f_to_c(f);
        println!("{} fahrenheit = {} celsius", f, celsius);
    }
}

fn convert_f_to_c(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}
