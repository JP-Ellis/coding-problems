use clap::{crate_authors, crate_version, load_yaml, App};
use coding_problems::Problem;
use colored::Colorize;
use fern::colors;
use regex::Regex;
use std::{error::Error, io};

/// Global configuration
pub struct Args {
    problems: Vec<Box<dyn Problem>>,
}

impl Default for Args {
    fn default() -> Self {
        Args {
            problems: Vec::new(),
        }
    }
}

/// Setup logging
fn setup_logging(verbosity: usize) -> Result<(), log::SetLoggerError> {
    let mut base_config = fern::Dispatch::new();

    let colors = colors::ColoredLevelConfig::new()
        .error(colors::Color::Red)
        .warn(colors::Color::Yellow)
        .info(colors::Color::Green)
        .debug(colors::Color::White)
        .trace(colors::Color::Black);

    let lvl = match verbosity {
        0 => log::LevelFilter::Warn,
        1 => log::LevelFilter::Info,
        2 => log::LevelFilter::Debug,
        _3_or_more => log::LevelFilter::Trace,
    };
    base_config = base_config.level(lvl);

    let stderr_config = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{level}] {target} - {message}",
                target = record.target(),
                level = colors.color(record.level()),
                message = message
            ))
        })
        .chain(io::stderr());

    base_config.chain(stderr_config).apply()?;

    log::debug!("Verbosity set to Debug.");
    log::trace!("Verbosity set to Trace.");

    Ok(())
}

/// Parse command line arguments and store them in the `Args` struct.
fn parse_args() -> Result<Args, Box<dyn Error>> {
    let yml = load_yaml!("args.yml");
    let app = App::from_yaml(yml)
        .author(crate_authors!())
        .version(crate_version!());

    let matches = app.get_matches();

    // Initialize the logger ASAP in case something goes wrong.
    let verbosity = matches.occurrences_of("verbose") as usize;
    setup_logging(verbosity)?;

    let mut args = Args::default();

    let dcp_regex = Regex::new(r"^(dcp|dc)(?P<num>\d+)$")?;
    let pe_regex = Regex::new(r"^pe(?P<num>\d+)$")?;

    if matches.occurrences_of("all") == 1 {
        for i in 0..1_000_000 {
            if let Ok(p) = coding_problems::daily_coding_problem::problem(i) {
                args.problems.push(p);
            }
        }
        for i in 0..1_000_000 {
            if let Ok(p) = coding_problems::project_euler::problem(i) {
                args.problems.push(p);
            }
        }
        return Ok(args);
    }

    if let Some(problems) = matches.values_of("problem") {
        for problem in problems {
            if let Some(caps) = dcp_regex.captures(problem) {
                args.problems
                    .push(coding_problems::daily_coding_problem::problem(
                        caps.name("num").unwrap().as_str().parse()?,
                    )?)
            } else if let Some(caps) = pe_regex.captures(problem) {
                args.problems.push(coding_problems::project_euler::problem(
                    caps.name("num").unwrap().as_str().parse()?,
                )?)
            }
        }
    }

    Ok(args)
}

/// Main function
fn main() -> Result<(), Box<dyn Error>> {
    let args = parse_args()?;

    for problem in args.problems {
        println!(
            "================================================================================"
        );
        problem.statement();
        println!(
            "--------------------------------------------------------------------------------"
        );
        if let Err(e) = problem.solve() {
            println!("{}", e.red());
        } else {
            println!("{}", "Success!".green());
        }
    }

    Ok(())
}
