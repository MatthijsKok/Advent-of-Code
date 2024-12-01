use tracing::trace;

mod day01;

fn main() {
    println!("Hello, world!");

    let tracing_subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(tracing_subscriber)
        .expect("setting default subscriber failed");

    trace!("Starting day 1");
    day01::solve();
}
