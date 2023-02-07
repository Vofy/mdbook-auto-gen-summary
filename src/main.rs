mod auto_gen_summary;

use auto_gen_summary::AutoGenSummary;
use clap::{Command, Arg, ArgMatches};
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor};
use std::io;
use std::process;

pub fn make_app() -> Command {
    Command::new("auto-gen-summary-preprocessor")
        .about("A mdbook preprocessor to auto generate book summary")
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
        .subcommand(
           Command::new("gen")
                .arg(Arg::new("dir").required(true).help("the dir of mdbook markdown src"))
                .arg(Arg::new("title").required(false).short('t').help("make the first line of markdown file as line text in SUMMARY.md"))
                .about("gen SUMMARY.md"),
        )
}

fn main() {
    let matches = make_app().get_matches();

    let preprocessor = AutoGenSummary::new();

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(&preprocessor, sub_args);
    } else if let Some(sub_args) = matches.subcommand_matches("gen") {
        let source_dir = sub_args
            .get_one::<String>("dir")
            .expect("Required argument")
            .to_string();

        let use_first_line_as_link_text = sub_args.get_one::<String>("title");

        auto_gen_summary::gen_summary(&source_dir, use_first_line_as_link_text.is_some());
    } else if let Err(e) = handle_preprocessing(&preprocessor) {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    if ctx.mdbook_version != mdbook::MDBOOK_VERSION {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

fn handle_supports(pre: &dyn Preprocessor, sub_args: &ArgMatches) -> ! {
    let renderer = sub_args.get_one::<String>("renderer").expect("Required argument");
    let supported = pre.supports_renderer(&renderer);

    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}
