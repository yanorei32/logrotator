use clap::Parser;
use std::borrow::Cow;
use std::io::prelude::*;
use std::path::PathBuf;
use std::time::SystemTime;
use std::{error::Error, fs::File};
use tokio::io::AsyncBufReadExt;
use tokio::time::{self, Duration};
use validator::Validate;

mod model;

fn create_an_file(dir: &PathBuf) -> Result<File, Box<dyn Error>> {
    let mut path = dir.clone();

    let unixtime = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_millis();

    let filename = format!("{unixtime}.log");
    eprintln!("Create a file: {filename}");
    path.push(&filename);

    Ok(File::create(path.into_os_string())?)
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = model::Args::parse();

    if args.gzip {
        unimplemented!("gzip対応、いつか、やる。");
    }

    if let Err(v) = args.validate() {
        for (k, v) in v.field_errors() {
            let errors: Vec<Cow<'static, str>> =
                v.iter().map(|v| v.code.clone()).collect();
            eprintln!("Invalid {k}, reason: {}", errors.join(", "));
        }

        return;
    }

    let mut output_file = create_an_file(&args.dir).expect("Failed to create inital file");
    let mut interval = time::interval(Duration::from_secs(args.split_interval.get()));
    let mut lines = tokio::io::BufReader::new(tokio::io::stdin()).lines();
    let mut line_count = 0;

    // drop first tick
    interval.tick().await;

    loop {
        tokio::select! {
            _ = interval.tick() => {
                eprintln!("Syncing {line_count} lines...");
                output_file.flush().expect("Failed to sync");
                output_file = create_an_file(&args.dir).expect("Failed to rotate");
                line_count = 0;
            }
            l = lines.next_line() => {
                let Some(l) = l.unwrap() else {
                    eprintln!("EOF received");
                    break;
                };

                output_file.write_all(l.as_bytes()).expect("Failed to write line");
                output_file.write_all(b"\n").expect("Failed to write endline");
                line_count += 1;
            },
        }
    }

    eprintln!("Syncing {line_count} lines...");
    output_file.flush().expect("Failed to sync");
}
