// Given a number `n`, return the `n+1`th number in the Fibonacci sequence.
//
// The Fibonacci sequence is defined as follows:
//
// - The first number of the sequence is 0.
// - The second number of the sequence is 1.
// - Every subsequent number is the sum of the two preceding numbers.
//
// So the sequence goes: 0, 1, 1, 2, 3, 5, 8, 13, 21, and so on.
//
// We expect `fibonacci(0)` to return `0`, `fibonacci(1)` to return `1`,
// `fibonacci(2)` to return `1`, and so on.
pub fn fibonacci(n: u32) -> u32 {
    // TODO: implement the `fibonacci` function
    //
    // Hint: use a `Vec` to memoize the results you have already calculated
    // so that you don't have to recalculate them several times.
    let mut fib_vec: Vec<u32> = Vec::new();
    fib_vec.insert(0, 0);
    fib_vec.insert(1, 1);

    if n == 0 {
        let value =  fib_vec.get(n as usize);
        return *value.unwrap();
    }
    if n == 1 {
        let value = fib_vec.get(n as usize);
        return *value.unwrap()

    }

    for i in 2..n+1 {
        let first = i - 1;
        let second = i - 2;
        let value = fib_vec.get(first as usize).unwrap() + fib_vec.get(second as usize).unwrap();
        println!("{} - {}",i,  value);
        fib_vec.insert(i as usize, value);
    }

    return *fib_vec.get(n as usize).unwrap();


}

#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn first() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn tenth() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn thirthieth() {
        assert_eq!(fibonacci(30), 832040);
    }
}
