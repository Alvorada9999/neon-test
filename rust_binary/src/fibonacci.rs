pub fn fibonacci(number: u128) -> u128 {
    let mut x: u128 = 1;
    let mut y: u128 = 1;
    let mut result: u128 = 1;
    let mut k: u128 = 1;
    while k < number {
        result = x + y;
        y = x;
        x = result;
        k += 1;
    }
    result
}