fn print(s: &str) {
    println!("print: {s}");
}
fn main() {
    let s1 = String::from("A");
    let r = &s1;
    print(r);
    if r.len() < 3 {
        let s2 = String::from("b");
        r = &s2;
    }
    print(r);
}
