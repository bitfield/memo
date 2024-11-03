use anyhow::Context;
use clap::Parser;

use std::path::PathBuf;

use memo::{Memos, Status};

#[derive(Parser)]
/// Store and manage simple reminders.
struct Args {
    /// Path to memo file
    #[arg(short, long, default_value = "memos.json")]
    file: PathBuf,
    /// Mark all matching memos as done
    #[arg(short, long)]
    done: bool,
    /// Delete all memos with status “done”
    #[arg(short, long)]
    purge: bool,
    /// Text of the memo to store or mark as done
    words: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut memos = Memos::open(&args.file).with_context(|| format!("reading {:?}", args.file))?;
    let text = args.words.join(" ");
    if args.purge {
        memos.purge_done();
    }
    if args.done {
        for m in memos.find_all(&text) {
            m.status = Status::Done;
            println!("Marked \"{}\" as done.", m.text);
        }
    } else if text.is_empty() {
        print!("{memos}");
    } else {
        memos.add(&text);
        println!("Added \"{}\" as a new memo.", &text);
    }
    memos
        .sync()
        .with_context(|| format!("writing {:?}", args.file))?;
    Ok(())
}
