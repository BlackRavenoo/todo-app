use todo_app::cli::{self, parse_args};
use todo_app::config::get_config;
use todo_app::files::check_dir;
use todo_app::ui::app;

fn main() {
    check_dir();

    let config = get_config().unwrap();

    match cli::get_args() {
        Ok(args) => match args.subcommand {
            Some(subcmd) => parse_args(subcmd, config),
            None => app(),
        },
        Err(e) => eprintln!("{}", e),
    }
}