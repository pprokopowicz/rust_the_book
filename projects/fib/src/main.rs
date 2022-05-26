fn main() {
    println!("{}", fib(0));
    println!("{}", fib(1));
    println!("{}", fib(20));
}

fn fib(index: usize) -> usize {
    match index {
        0 => 0,
        1 => 1,
        _ => {
            let mut first_number = 1;
            let mut second_number = 1;
            let mut next = 0;
            for _ in 3..=index {
                next = first_number + second_number;
                first_number = second_number;
                second_number = next;
            }
            next
        }
    }
}