use std::io;

fn print_string(x: &String) {
    println!("{}", x);
}
fn main() {
    let mut input: String = String::new();
    print_string(&"Type something below...".to_string());
    match io::stdin().read_line(&mut input) {
        Ok(_ok) => {}
        Err(_err) => {}
    };
    print_string(&input);
    input = String::new();
    print_string(&"Type something else...".to_string());
    match io::stdin().read_line(&mut input) {
        Ok(_ok) => {}
        Err(_err) => {}
    };
    print_string(&input)
}
