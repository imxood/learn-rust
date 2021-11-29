use time::macros::format_description;
use tracing::info;
use tracing_subscriber::fmt::time::LocalTime;
use tracing_unwrap::ResultExt;

fn main() {
    std::env::set_var("RUST_LOG", "DEBUG");

    let local_timer = LocalTime::new(format_description!(
        "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]"
    ));

    let file_appender =
        tracing_appender::rolling::daily(std::env::current_dir().unwrap(), "prefix.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_timer(local_timer)
        .with_ansi(false)
        .with_writer(non_blocking)
        .init();

    info!("now: {:?}", std::time::SystemTime::now());

    info!("hello");

    std::thread::spawn(|| {
        info!("thread1");
        let not_great: Result<(), _> = Result::Err("not terrible");
        let ret = not_great.unwrap_or_log();
    })
    .join()
    .unwrap();
}
