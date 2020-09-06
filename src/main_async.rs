#[macro_use]
extern crate log;
extern crate simple_logger;

use std::time::{Duration, Instant};
use futures::prelude::*;
use tokio::prelude::*;
use tokio::task;
use log::*;


fn slowwly(delay_ms: u32) -> reqwest::Url {
    let url = format!(
        "http://slowwly.robertomurray.co.uk/delay/{}/url/http://www.google.co.uk",
        delay_ms,
    );
    reqwest::Url::parse(&url).unwrap()
}

async fn request(n: usize) -> Result<(), ()> {
    reqwest::get(slowwly(1000)).await;
    info!("Got response from {}", n);
    Ok(())
}

async fn app() -> Result<(), ()> {
    info!("Starting program!");
    let a = task::spawn(request(1));
    let b = task::spawn(request(2));

    a.await;
    b.await;
    Ok(())
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    app().await;

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);

}
