use std::io;
// 0 1 1 2 3 5 8 13 21 34
fn generate_nth_fibonacci_number_iterative(nth: i128) -> u128 {
    let mut prev: u128 = 0;
    let mut sum: u128 = 0;
    let mut prev_aux: u128 = 0;
    let mut counter: i128 = 0;

    while counter < nth {
        prev_aux = sum;
        if counter == 0 {
            sum = 0
        }

        if counter == 1 {
            sum = 1
        }
        sum = sum + prev;
        prev = prev_aux;
        counter += 1;
        println!("sum: {}", sum);
    }

    sum
}

fn fibonacci_recursive(n: u128) -> u128 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }
    fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
}
fn main() {
    let input_nth: u128 = loop {
        let mut input_nth = String::new();

        println!("\nEnter the nth fibonacci number to generate:");

        io::stdin()
            .read_line(&mut input_nth)
            .expect("\nFailed to read line ❌");

        match input_nth.trim().parse::<u128>() {
            Ok(nth) => break nth,
            Err(_) => {
                println!("Please provide a valid value!");
                continue;
            }
        }
    };

    let total_sum = fibonacci_recursive(input_nth);

    println!("{}", total_sum);
}
