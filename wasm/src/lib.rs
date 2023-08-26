#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn factorial(n: i64) -> i64 {
    if n == 0 || n == 1 {
        return 1;
    }
    n * factorial(n - 1)
}

// Function to find the square root of a number using Newton's method
#[no_mangle]
pub extern "C" fn newton_sqrt(x: f64) -> f64 {
    const EPSILON: f64 = 1e-6;
    let mut guess = 1.0;

    while (guess * guess - x).abs() > EPSILON {
        guess = 0.5 * (guess + x / guess);
    }

    guess
}

#[no_mangle]
pub extern "C" fn fibonacci(n: i32) -> i32 {
    if n <= 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
