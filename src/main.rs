fn main() {
    // Project 1
    // multiples_3_or_5();

    // Project 2
    // sum_of_even_fibonnaci_numbers();

    // Project 3
    // largest_prime_factor(600851475143);

}

fn largest_prime_factor(num: u64) {
    let factors = (1..num)
        .into_iter()
        .filter(|&x| num % x == 0)
        .reduce(|cur_fac, e| {
            println!("{:?}, {:?}", e, cur_fac);
            if cur_fac % 2 == 0 || num % cur_fac != 0 {
                return e;
            }

            // Test is prime.
            if (2..e).all(|v| {
                return e % v != 0;
            }) {
                if (e > cur_fac) {
                    return e;
                }
            }

            return cur_fac;
        });

    println!("Factors: {:#?}", factors);
}

fn sum_of_even_fibonnaci_numbers() {
    let mut fib_nums: Vec<u64> = vec!();
    let mut prev = 1;
    let mut current = 1;

    loop {
        if current > 4000000 {
            break;
        }

        if current % 2 == 0 {
            fib_nums.push(current);
        }

        let temp = current;
        current += prev;
        prev = temp;
    }

    println!(
        "The sum of all even fibonacci numbers up until 4 million: {:?}",
        fib_nums.iter().sum::<u64>()
    );
    println!("");
}

fn multiples_3_or_5() {
    println!(
        "Multiples of three, or five between 1 and 1000: {:?}",
        (1..1000)
            .filter(|n| n % 5 == 0 || n % 3 == 0)
            .sum::<u64>()
     );
    println!("");
}
