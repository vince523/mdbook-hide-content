use clap::{Arg, Command};
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor};
use mdbook_hide_content::HideContent;
use std::io;
use std::process;

pub fn make_app() -> Command {
    Command::new("mdbook-hide-content")
        .about("A mdbook preprocessor for hiding content")
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
}

fn main() {
    let matches = make_app().get_matches();

    // Check if we are being called by mdbook
    if let Some(sub_args) = matches.subcommand_matches("supports") {
        let renderer = sub_args.get_one::<String>("renderer").expect("Required argument");
        process::exit(if renderer == "html" { 0 } else { 1 });
    }

    let preprocessor = HideContent::new();

    if let Err(e) = handle_preprocessing(&preprocessor) {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;
    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;
    Ok(())
}