use cursive::{CursiveExt, Cursive};

#[derive(Debug, clap::Parser)]
struct Cli {
    #[clap(long)]
    log: bool,
}

fn main() {
    let cli: Cli = clap::Parser::parse();

    if cli.log {
        env_logger::builder()
            .filter(Some("tui_do_dev"), log::LevelFilter::Trace)
            .format_timestamp(None)
            .init();
    }

    let mut siv = Cursive::new();

    siv.set_global_callback('q', |s| s.quit());

    siv.run();
}
