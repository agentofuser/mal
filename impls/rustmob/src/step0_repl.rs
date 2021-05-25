use std::io::{self, Write};

fn read(s: String) -> String {
    s
}

fn eval(s: String) -> String {
    s
}

fn print(s: String) {
    println!("{}", s);
}

fn rep(s: String) {
    print(eval(read(s)));
}

fn main() {
    loop {
        print!("user> ");
        io::stdout().flush().unwrap();
        //        let input = String::from("Ai ai ai caramba");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                //                println!("{} bytes read", n);
                //                println!("{}", input);
                rep(input);
            }
            Err(_) => std::process::exit(1),
        }
    }
}
