use clap::*;
use git2::*;
use ansi_term::*;
use std::env::*;
use std::process::exit;
use std::path::*;
use indicatif::ProgressBar;
fn main() {
    let mut matches = App::new("lorry-cli")
    .about("Helps tasks with lorry lib")
    .setting(AppSettings::SubcommandRequired)
    .version("1.0.0")
    .subcommand(clap::SubCommand::with_name("init")
        .about("initializes a new cargo project with lorry")
        .arg(Arg::with_name("project_name")
        .required(true)
        )

    )
    .get_matches();
    match matches.subcommand_matches("init") {
        Some(matchu) => init(matchu),
        None => {}
    }
}
fn init(a: &ArgMatches) {
    println!("{}",ansi_term::Color::Green.bold().paint("Downloading Repo...") );
    let mut pathtoTemplate = PathBuf::new();
    pathtoTemplate.push(std::env::current_dir().unwrap());
    pathtoTemplate.push(a.value_of("project_name").unwrap());
    let mut spinner = ProgressBar::new_spinner();
    spinner.inc(1);
    let   repo =  match git2::Repository::clone("https://github.com/jakobneufeld/lorry-template.git", pathtoTemplate) {
        Ok(repo) => repo,
        Err(_) => error(" Folder exists, or not empty, No intenet, or some git related issue.")

    };
    spinner.finish();
    println!("{}",ansi_term::Color::Green.bold().paint("Succes...") );
    println!("{}",ansi_term::Color::Green.bold().paint("One more step") );
    println!("{}",ansi_term::Color::Green.bold().paint("Go to spec.rs and change the name placeholder to the appropiate name.") );






}
fn error(reason: &str) -> ! {
    eprintln!(
        "{}: {}",
        ansi_term::Color::Red
            .bold()
            .paint("Whoops an error has occured"),
        Color::Cyan.paint(reason)
    );
    exit(1);
}
