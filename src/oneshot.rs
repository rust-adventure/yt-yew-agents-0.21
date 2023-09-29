use yew_agent::prelude::*;

#[oneshot]
pub async fn FibonacciTask(n: u32) -> u32 {
    fn fib(n: u32) -> u32 {
        if n <= 1 {
            1
        } else {
            fib(n - 1) + fib(n - 2)
        }
    }

    fib(n)
}
