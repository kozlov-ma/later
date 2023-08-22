use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use dirs;
use std::io::prelude::*;
use std::path::PathBuf;
use std::{
    fs::remove_file,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about=None)]
struct Args {
    /// (optional) one-line body of the task. If omitted, will print all tasks
    task: Option<String>,
    /// (optional) remove all tasks
    #[arg(long, short)]
    clear: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    match (args.task, args.clear) {
        (Some(task), false) => add_task(&task)?,
        (None, false) => print_todos()?,
        (Some(_task), true) => println!(
            "{}",
            "Cannot clear and print tasks simultaneously.".red().bold()
        ),
        (None, true) => clear_todos()?,
    }

    Ok(())
}

fn get_data_path() -> Result<PathBuf> {
    let mut path = dirs::data_dir().expect("Data directory is not available or OS is unknown.");
    path.push("later");
    #[cfg(debug_assertions)]
    {
        path.set_extension("dbg.txt");
    }
    #[cfg(not(debug_assertions))]
    {
        path.set_extension("txt");
    }
    Ok(path)
}

fn get_file(read_only: bool) -> Result<File> {
    let data_path = get_data_path()?;
    let file = match (read_only, data_path.exists()) {
        (true, true) => File::open(&data_path)?,
        _ => File::options()
            .read(true)
            .create(true)
            .write(true)
            .append(true)
            .open(&data_path)?,
    };

    Ok(file)
}

fn add_task(task: &str) -> Result<()> {
    if task.contains("\n") {
        println!("{}", "task body should be single-line.".red().bold());
        return Ok(());
    }

    let mut file = get_file(false)?;
    writeln!(file, "{task}")?;
    println!("{}", "Task added!".green().bold());
    Ok(())
}

fn print_todos() -> Result<()> {
    let file = get_file(true)?;
    let mut lines = BufReader::new(file).lines().peekable();
    if lines.peek().is_none() {
        println!("{}", "No tasks to print.".yellow().bold());
        return Ok(());
    }

    for (idx, line) in lines.enumerate() {
        println!(
            "{}. {}",
            (idx + 1).to_string().bold().green(),
            line?.bold().blue()
        )
    }

    Ok(())
}

fn clear_todos() -> Result<()> {
    let path = get_data_path()?;
    if path.exists() {
        remove_file(path)?;
    }

    println!("{}", "Cleared tasks.".bold().green());
    Ok(())
}
