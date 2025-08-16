mod pi;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let digits = if args.len() > 1 {
        match args[1].parse::<usize>() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Please provide a valid number of digits for PI.");
                return;
            }
        }
    } else {
        eprintln!("Usage: {} <number_of_digits>", args[0]);
        return;
    };

    println!("Hello, world! This is PI={}", pi::sprint(digits as u32));
}
