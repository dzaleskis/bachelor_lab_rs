// returns elapsed time in microseconds
pub fn with_time<F, R>(f: F) -> (u64, R)
where
    F: Fn() -> R,
{
    let now = std::time::Instant::now();
    let result = f();

    (now.elapsed().as_micros() as u64, result)
}
