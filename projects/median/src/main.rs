fn main() {
    let mut numbers = vec![6, 1, 2, 0, 5, 6, 8, 0, 1, 10];
    println!("{}", median(&mut numbers));
}

fn median(numbers: &mut [i32]) -> i32 {
    numbers.sort();

    let mid = numbers.len() / 2;

    if numbers.len() % 2 == 0 {
        (numbers[mid] + numbers[mid - 1]) / 2
    } else {
        numbers[mid]
    }
}
