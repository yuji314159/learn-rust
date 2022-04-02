fn main() {
    let a = (1, 2, 3);
    let b = a;
    println!("{} {}", a.0, b.0);

    let c = [1, 2, 3];
    let mut d = c;
    d[0] = 0;
    println!("{} {}", c[0], d[0]);

    let mut s = String::from("hello");
    let t = &s;
    f1(&s);
    // f2(&mut s);
    println!("{} {}", s, t);
}

fn f1(s: &String) {
    println!("{}", s);
}

fn f2(s: &mut String) {
    println!("{}", s);
}
