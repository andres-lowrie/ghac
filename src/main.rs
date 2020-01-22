#[macro_use]
extern crate clap;

use clap::App;

fn main() {
    App::new("GitHub API for CLI")
        .version(crate_version!())
        .get_matches();
}
