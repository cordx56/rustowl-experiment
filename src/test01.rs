fn main() {
    let mut s = String::from("Hello");
    let r = &s;
    s.push_str(", world!");
    println!("{r}");
    println!("{s}");
}
