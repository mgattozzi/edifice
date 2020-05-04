use structopt::StructOpt;
use std::{error::Error, fs, process, path::PathBuf};
use serde_json::Value;

#[derive(Debug, StructOpt)]
#[structopt(name = "edifice", about = "A structured text diff tool")]
struct Args {
  #[structopt(parse(from_os_str))]
  first_file: PathBuf,
  #[structopt(parse(from_os_str))]
  second_file: PathBuf
}

fn main() {
  if let Err(e) = run() {
    eprintln!("edifice failed with: {}", e);
    process::exit(1);
  }
}

fn run() -> Result<(), Box<dyn Error>> {
  let args = Args::from_args();
  let first_file: Value = serde_json::from_slice(&fs::read(&args.first_file)?)?;
  let second_file: Value = serde_json::from_slice(&fs::read(&args.second_file)?)?;
  json(first_file, second_file)
}

fn json(fst: Value, snd: Value) -> Result<(), Box<dyn Error>> {
  match (fst, snd) {
    (Value::Object(fst), Value::Object(snd)) => {
      for (key, fst_value) in fst.iter() {
        if let Some(snd_value) = snd.get(key) {
          if *fst_value != *snd_value {
            println!("Key: {} Values: {} => {}", key, fst_value, snd_value);
          }
        }
      }
    }
    (_,_) => todo!("other variants"),
  }
  Ok(())
}
