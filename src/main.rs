// __ ADD
// track file
// check file diff
// commit changes to delta file

// __ REVERT
// check file diff
// change file to previous version (opposite operation)

use clap::{Parser, Subcommand};
use std::{fs::{self, create_dir, File, OpenOptions}, io::{Write, Read}};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Subcommand)]
enum SubCommand{
    Status,
    Add {
        file: String,
    },
    Knot {
        #[clap(short, long)]
        message: Option<String>,
    },
    Init,

}
fn main() {
    let cli = Cli::parse();

    match cli.subcmd {
        SubCommand::Init => {
            create_dir(".rift").expect("Failed to create .rift directory");
            create_dir(".rift/deltas").expect("Failed to create .rift/deltas directory"); // changes to files
            File::create(".rift/flagship").expect("Failed to create .rift/flagship file"); // list of files to track
            println!("Initialized empty rift repository in .rift");
        }
        SubCommand::Add { file } => {
            let file_name = file.clone();
            let mut tracked_files = OpenOptions::new()
                .read(true)
                .append(true)
                .open(".rift/flagship")
                .expect("Failed to open .rift/flagship file\nRun `rift init` to initialize a rift repository");
            
            if fs::read_to_string(".rift/flagship").unwrap().contains(&file_name) {
                println!("File already tracked");
            } else {
                tracked_files.write((file + "\n").as_bytes()).expect("Failed to write to .rift/flagship file");
                println!("tracking {}", file_name);
            }
        },
        SubCommand::Knot { message } => {
            unimplemented!("Knot command not implemented yet");
        },
        SubCommand::Status => {
            unimplemented!()
        },
    }
}
