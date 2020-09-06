extern crate log;

use std::time::{Instant};
use log::*;
use simple_logger::SimpleLogger;

fn slowwly(delay_ms: u32) -> reqwest::Url {
    let url = format!(
        "http://slowwly.robertomurray.co.uk/delay/{}/url/http://www.google.co.uk",
        delay_ms,
    );
    reqwest::Url::parse(&url).unwrap()
}

async fn request(n: usize) -> Result<(), ()> {
    let _request_result = reqwest::get(slowwly(1000)).await;
    info!("Got response from {}", n);
    Ok(())
}

async fn app() -> Result<(), ()> {
    info!("Starting program!");
    let result_a = request(1);
    let result_b = request(2);

    let _result_a_unwrapped = result_a.await;
    let _result_b_unwrapped = result_b.await;

    Ok(())
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let _logger_init = SimpleLogger::new().with_module_level("something", LevelFilter::Warn).init();

    let result = app().await;
    match result {
        Ok(_) => info!("Done"),
        Err(_) => error!("An error ocurred"),
    }

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);

}


