fn print(s: String) {
    println!("length is less than 3: {s}");
}

fn main() {
    let s = String::from("Hello");
    if s.len() < 3 {
        print(s);
    }
    println!("long length: {s}");
}
