function fibonacciJs(number) {
    let x = 1.0;
    let y = 1.0;
    let result = 1.0;
    let k = 1.0;
    while (k < number) {
        result = x + y;
        y = x;
        x = result;
        k += 1.0;
    }
    return result;
}

module.exports = fibonacciJs;