fn push(mut s: String, push: String) -> String {
    s.push_str(&push);
    return s;
}
fn main() {
    let mut a = String::from("A");
    let b = String::from("B");
    for _ in 0..5 {
        a = push(a, b);
    }
    println!("{a}");
}
