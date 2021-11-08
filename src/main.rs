mod ch03;
mod ch04;

fn main() {
    println!("===== chapter 03 =====");
    // ch03::hello_world::hello_world();
    ch03::functions::function();
    ch03::functions::fn_or_statement();

    ch03::data_struct::data_struct();

    let n = 10;
    ch03::control::fib_loop(n);
    ch03::control::fib_while(n);
    ch03::control::fib_for(n);

    println!("===== chapter 04 =====");
}
