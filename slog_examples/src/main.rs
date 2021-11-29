use log::{debug, error, info, trace, warn};
use slog::o;
use slog::Drain;

fn main() {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let _log = slog::Logger::root(drain, o!());

    debug!("logging a trace message");
    error!("logging a trace message");
    info!("logging a trace message");
    trace!("logging a trace message");
    warn!("logging a trace message");

    // trace!(log, "logging a trace message");
    // debug!(log, "debug values"; "x" => 1, "y" => -1);
    // info!(log, "some interesting info"; "where" => "right here");
    // warn!(log, "be cautious!"; "why" => "you never know...");
    // error!(log, "wrong {}", "foobar"; "type" => "unknown");
    // crit!(log, "abandoning test");
}
