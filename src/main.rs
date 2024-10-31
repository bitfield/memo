use clap::Parser;

use memo::{Memos, Status};

#[derive(Parser)]
/// Store and manage simple reminders.
struct Args {
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
    let mut memos = Memos::open("memos.txt")?;
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
    memos.sync()?;
    Ok(())
}
