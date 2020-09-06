extern crate log;
extern crate simple_logger;

use std::time::{Instant};
use tokio::task;
use simple_logger::SimpleLogger;
use log::*;

fn slowwly(delay_ms: u32) -> reqwest::Url {
    let url = format!(
        "http://slowwly.robertomurray.co.uk/delay/{}/url/http://www.google.co.uk",
        delay_ms,
    );
    reqwest::Url::parse(&url).unwrap()
}

async fn request(n: usize) -> Result<(), ()> {
    let _request = reqwest::get(slowwly(1000)).await;
    info!("Got response from {}", n);
    Ok(())
}

async fn app() -> Result<(), ()> {
    info!("Starting program!");
    let task_a = task::spawn(request(1));
    let task_b = task::spawn(request(2));

    let _task_a_unwrapped = task_a.await;
    let _task_b_unwrapped = task_b.await;
    Ok(())
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let _logger_init = SimpleLogger::new().with_module_level("something", LevelFilter::Warn).init();
    let _task = app().await;

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);

}
