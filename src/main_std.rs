extern crate log;

use std::time::{Instant};
use log::*;
use simple_logger::SimpleLogger;
use futures::join;

fn slowwly(delay_ms: u32) -> reqwest::Url {
    let url = format!(
        "http://slowwly.robertomurray.co.uk/delay/{}/url/http://www.google.co.uk",
        delay_ms,
    );
    reqwest::Url::parse(&url).unwrap()
}

fn analyze(txt: &str) -> (u64, u64) {
    let txt = txt.as_bytes();
    // Let's spend as much time as we can and count them in two passes
    let ones = txt.iter().fold(0u64, |acc, b: &u8| acc + b.count_ones() as u64);
    let zeros = txt.iter().fold(0u64, |acc, b: &u8| acc + b.count_zeros() as u64);
    (ones, zeros)
}

async fn get_and_analyze(n: usize) -> Result<(u64, u64), ()> {
    let response: reqwest::Response = reqwest::get(slowwly(1000)).await.unwrap();
    info!("Dataset {}", n);

    // we get the body of the request
    let txt = response.text().await.unwrap();

    let res = analyze(&txt);
    info!("Processed {}", n);
    Ok(res)
}

async fn app() -> Result<(), ()> {
    info!("Starting program!");
    let mut results = vec![];

    for i in 1..=10 {
        let fut = get_and_analyze(i).await;
        results.push(fut);
    }

    let mut total_ones = 0;
    let mut total_zeros = 0;

    for result in results {
        let ones_res: Result<(u64, u64), ()> = result;
        let (ones, zeros) = ones_res?;
        total_ones += ones;
        total_zeros += zeros;
    }

    info!("Ratio of ones/zeros: {:.02}",total_ones as f64 / total_zeros as f64);
    Ok(())
}

#[tokio::main(core_threads = 1)]
async fn main() {
    let start = Instant::now();
    let _logger_init = SimpleLogger::new().with_module_level("something", LevelFilter::Warn).init();

    let _result = join!(app());

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);

}


