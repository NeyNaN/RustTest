fn greetings() {
    let text_var = "variable";
    println!("Hello, world!");
    println!(
        "Testing formatting of {} {} {}",
        "formatted", text_var, "inputs"
    )
}

fn fib_simple(n: i32)
{
    let mut fib_n = 0;
    if n == 0 {
        // Do nothing
    } else if n == 1 {
        fib_n = 1;
    } else if n > 1 {
        fib_n = 1;
        let mut fib_nminus1 = 1;
        for _ in 2..n {
            let fib_n_old = fib_n;
            fib_n = fib_n + fib_nminus1;
            fib_nminus1 = fib_n_old;
        }
    }
    println!("Fib {n} is {fib_n}");
}

fn main() {
    greetings();
    fib_simple(6);
    fib_simple(12);
}
