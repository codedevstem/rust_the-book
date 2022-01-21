use std::io::stdin;

const LINE: &str = "--------";

pub fn print_header(header: &str) {
    println!("{} {} {}", LINE, String::from(header), LINE);
}

pub fn wait(message: &str) {
    print!("\n{}\n", String::from(message));
    let mut input_string = String::new();
    while input_string == "" {
        stdin().read_line(&mut input_string)
            .ok()
            .expect("Failed to read line");
    }

}


