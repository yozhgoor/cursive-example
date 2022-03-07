use cursive::{Cursive, CursiveExt};

mod data;
mod user;

#[derive(Debug, clap::Parser)]
struct Cli {
    user: Option<String>,
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
    siv.set_user_data(data::ProgramData::mock());

    let res = siv
        .with_user_data(|data: &mut data::ProgramData| {
            if let Some(input) = &cli.user {
                if data.user_list.contains_key(input) {
                    data.active_user = cli.user;
                    true
                } else {
                    false
                }
            } else {
                false
            }
        })
        .expect("cannot get user data");

    if res {
        user::user_welcome(&mut siv)
    } else {
        user::user_list(&mut siv)
    }

    siv.run();
}
