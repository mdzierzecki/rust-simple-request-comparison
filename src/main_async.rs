extern crate log;
extern crate simple_logger;

use std::time::{Instant};
use tokio::task;
use simple_logger::SimpleLogger;
use log::*;
use futures::join;

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
    let task_c = task::spawn(request(3));
    let task_d = task::spawn(request(4));
    let task_e = task::spawn(request(5));

    let _task_a_unwrapped = task_a.await;
    let _task_b_unwrapped = task_b.await;
    let _task_c_unwrapped = task_c.await;
    let _task_d_unwrapped = task_d.await;
    let _task_e_unwrapped = task_e.await;
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
