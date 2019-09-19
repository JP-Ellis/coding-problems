use clap::{crate_authors, crate_version, load_yaml, App};
use coding_problems::{Error, Problem};
use colored::Colorize;
use fern::colors;
use regex::Regex;
use std::{error, io, io::prelude::*};

/// Global configuration
pub struct Args<'a> {
    summary: bool,
    problems: Vec<&'a dyn Problem>,
}

impl<'a> Default for Args<'a> {
    fn default() -> Self {
        Args {
            summary: false,
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
fn parse_args<'a>() -> Result<Args<'a>, Box<dyn error::Error>> {
    let yml = load_yaml!("args.yml");
    let app = App::from_yaml(yml)
        .author(crate_authors!())
        .version(crate_version!());

    let matches = app.get_matches();

    // Initialize the logger ASAP in case something goes wrong.
    let verbosity = matches.occurrences_of("verbose") as usize;
    setup_logging(verbosity)?;

    let mut args = Args::default();

    if matches.occurrences_of("summary") == 1 {
        args.summary = true;
    }

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
                let num = caps.name("num").unwrap().as_str().parse()?;
                match coding_problems::daily_coding_problem::problem(num) {
                    Ok(p) => args.problems.push(p),
                    Err(i) => Err(format!("Unable to find Daily Coding Problem #{}", i))?,
                }
            } else if let Some(caps) = pe_regex.captures(problem) {
                let num = caps.name("num").unwrap().as_str().parse()?;
                match coding_problems::project_euler::problem(num) {
                    Ok(p) => args.problems.push(p),
                    Err(i) => Err(format!("Unable to find Project Euler #{}", i))?,
                }
            }
        }
    }

    Ok(args)
}

/// Main function
fn main() -> Result<(), Box<dyn error::Error>> {
    let args = parse_args()?;

    if args.summary {
        for problem in args.problems {
            summary(problem)?;
        }
    } else {
        for problem in args.problems {
            run(problem)?;
        }
    }

    Ok(())
}

/// Run the problems within the config, displaying both the problem and the outputs
fn run(problem: &dyn Problem) -> Result<(), Error> {
    let mut out = io::stdout();
    writeln!(
        out,
        "================================================================================"
    )?;
    writeln!(out, "{}", Colorize::bold(problem.name()))?;
    writeln!(
        out,
        "--------------------------------------------------------------------------------"
    )?;
    writeln!(out, "{}", problem.statement())?;
    writeln!(
        out,
        "--------------------------------------------------------------------------------"
    )?;
    match problem.solve(&mut out) {
        Ok(()) => writeln!(out, "{}", "Success!".green())?,
        Err(Error::Unimplemented) => writeln!(out, "{}", "unimplemented".red())?,
        Err(Error::Failed(s)) => writeln!(out, "{}", format!("Failed: {}", s).bright_red())?,
        Err(Error::Io(e)) => writeln!(out, "Io: {}", e)?,
    }

    Ok(())
}

/// Provide a summary of whether each problem succeeds
fn summary(problem: &dyn Problem) -> Result<(), Error> {
    let mut out = io::stdout();
    write!(out, "{}: ", problem.name())?;

    match problem.solve(&mut io::sink()) {
        Ok(()) => writeln!(out, "{}", "Success!".green())?,
        Err(Error::Unimplemented) => writeln!(out, "{}", "unimplemented".red())?,
        Err(Error::Failed(s)) => writeln!(out, "{}", format!("Failed: {}", s).bright_red())?,
        Err(Error::Io(e)) => writeln!(out, "Io: {}", e)?,
    }

    Ok(())
}
