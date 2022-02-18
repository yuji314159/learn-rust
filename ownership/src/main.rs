fn main() {
    let a = (1, 2, 3);
    let b = a;
    println!("{} {}", a.0, b.0);

    let c = [1, 2, 3];
    let mut d = c;
    d[0] = 0;
    println!("{} {}", c[0], d[0]);
}
