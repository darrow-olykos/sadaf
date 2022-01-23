mod file_index;
use file_index::FileIndex;

use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;

use clap::{App, Arg, ArgMatches};
use notify::{watcher, RecursiveMode, Watcher, DebouncedEvent};

pub fn init_app() -> App<'static> {
    App::new("watch-files")
        .about("start watching and indexing files for search")
        .arg(
            Arg::new("PATH")
                .short('p')
                .long("path")
                .value_name("PATH")
                .help("Sets path to watch and index files from for search")
                .takes_value(true)
                .default_value("./"),
        )
}

pub fn entry_fn(arg_matches: &ArgMatches) {
    let file_index = &mut FileIndex::new();

    let path = arg_matches.value_of("PATH").unwrap();
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(3)).unwrap();
    watcher.watch(path, RecursiveMode::Recursive).unwrap();
    loop {
        match rx.recv() {
            Ok(event) => match event {
                DebouncedEvent::Create(path_buf) => handle_create_event(&path_buf, file_index),
                _ => () // ignore other events for now
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn handle_create_event(path_buf: &PathBuf, file_index: &mut FileIndex) {
    if let Some(file_ext_os_string) = path_buf.extension() {
        match file_ext_os_string.to_str() {
            Some(file_ext) => file_index.set(file_ext, path_buf.to_path_buf()),
            None => (),
        }
    }
}
