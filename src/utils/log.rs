use slog::Drain;

fn time_formatter(io: &mut dyn std::io::Write) -> ::std::io::Result<()> {
    write!(io, "{}", chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"))
}

pub fn new_logger(path: String, thread: &str, plain: bool) -> slog::Logger {
    let log_file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path.to_string())
        .expect(&(String::from("Failed to create file: ").to_string() + &path));

    let mut drain = slog_term::TermDecorator::new();
    if plain {
        drain = drain.force_plain();
    }

    let drain = slog_term::CompactFormat::new(drain.build())
        .use_custom_timestamp(time_formatter)
        .build()
        .fuse();

    let file_drain = slog_term::CompactFormat::new(slog_term::PlainDecorator::new(log_file))
        .use_custom_timestamp(time_formatter)
        .build()
        .fuse();

    let log = slog::Logger::root(
        slog::Duplicate::new(
            slog_async::Async::new(drain).build().fuse(),
            slog_async::Async::new(file_drain).build().fuse(),
        )
        .fuse(),
        o!("thread" => String::from(thread)),
    );
    return log;
}
