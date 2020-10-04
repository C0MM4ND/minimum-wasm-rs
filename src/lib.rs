#[no_mangle]
pub extern fn sum(x: i32, y: i32) -> i32 {
    x + y
}

#[no_mangle]
pub extern fn fib(n: u32) -> u32 {
    fibonacci(n)
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}
