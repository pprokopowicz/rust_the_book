fn main() {
    float_main();
    operations_main();
    bool_main();
    char_main();
    tup_main();
    array_main();
}

fn float_main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}

fn operations_main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;
}

fn bool_main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}

fn char_main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}

fn tup_main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}

fn array_main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}