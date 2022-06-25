use std::io; 

fn main() {
    println!("Please input degrees in Farenheit to convert."); 

    let mut conv = String::new(); 

    io::stdin()
        .read_line(&mut conv)
        .expect("Failed to read line");

    let conv: f64 = conv.trim().parse().expect("Please type a number with at least one decimal point!!");

    let x = conv - 32.0; 
    let y = x / 1.8; 

    println!("Degrees in Celsius ={}", y); 
}
