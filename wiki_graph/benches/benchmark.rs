macro_rules! benchmark {
    ($code:expr) => {
        let t = Instant::now();
        $code;
        println!(
            "{:?} seconds elapsed for: {}",
            t.elapsed(),
            stringify!($code)
        );
    };
}

fn main()
{
    //TODO: add benchmarks
}
