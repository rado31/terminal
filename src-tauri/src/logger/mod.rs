use chrono::Local;
use fern::Dispatch;

pub fn init() {
    let dispatch = Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .chain(std::io::stdout());

    let file = fern::log_file("error.log").unwrap();
    let dispatch = dispatch.level(log::LevelFilter::Error).chain(file);

    dispatch.apply().unwrap();
}
