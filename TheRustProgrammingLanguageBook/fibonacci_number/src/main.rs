use std::io;

fn main() {
    let  mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: u32 = number.trim()
        .parse()
        .expect("Please type a number!");

    let mut ant = 0;
    let mut sig = 1;

    for _ in 0..number {
        let num = ant + sig;
        println!("{}", num);

        ant = sig;
        sig = num;
    }
}
