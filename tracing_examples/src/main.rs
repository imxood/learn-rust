use std::io::Error;
use time::macros::format_description;
use tracing::metadata::LevelFilter;
use tracing::{error, info};
use tracing_subscriber::fmt::time::LocalTime;

struct TestWriter;

impl std::io::Write for TestWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let buf_len = buf.len();
        println!("{:?}", buf);
        Ok(buf_len)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn main() {
    let local_timer = LocalTime::new(format_description!(
        "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]"
    ));

    let file_appender =
        tracing_appender::rolling::daily(std::env::current_dir().unwrap(), "prefix.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(TestWriter {});

    tracing_subscriber::fmt()
        .with_timer(local_timer)
        .with_max_level(LevelFilter::DEBUG)
        .with_ansi(false)
        .with_writer(non_blocking)
        .init();

    info!("now: {:?}", std::time::SystemTime::now());

    info!("hello");
    error!("hello");

    // std::thread::spawn(|| {
    //     info!("thread1");
    //     let not_great: Result<(), _> = Result::Err("not terrible");
    //     let ret = not_great.unwrap_or_log();
    // })
    // .join()
    // .unwrap();
}
