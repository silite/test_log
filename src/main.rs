use std::{thread::sleep, time::Duration};

use ftlog::appender::FileAppender;

fn main() {
    ftlog::Builder::new()
        .root(std::io::stdout())
        .root(FileAppender::new("backtest.log"))
        .max_log_level(ftlog::LevelFilter::Debug)
        .try_init()
        .expect("logger failed");

    ftlog::info!("test");
    sleep(Duration::from_secs(3))
}
