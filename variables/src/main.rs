fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6;
    // error[E0384]: cannot assign twice to immutable variable `x`
    // println!("The value of x is: {}", x);

    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let x = 5 / 2;
    println!("{}", x);

    let x = (1, 2, 3);
    let y = x.0;
    println!("{} {}", x.0, y);

    another_function();
}

fn another_function() -> i32 {
    let x = {
        5
    };
    x
}
