use clap::App;
use watch_files;

fn main() {
    let matches = App::new("sadaf")
        .subcommand(watch_files::init_app())
        .get_matches();

    if let Some((name, arg_matches)) = matches.subcommand() {
        match name {
            "watch-files" => watch_files::entry_fn(arg_matches),
            _ => todo!()
        }
    }
}
