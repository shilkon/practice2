use clap::Parser;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{BufReader, Read, Write};

#[derive(Parser, Serialize, Deserialize)]
#[command(author, version, about, long_about = None)]
struct Args {
   #[arg(long)]
   i_file: String,

   #[arg(long)]
   i_format: String,

   #[arg(long)]
   o_file: String,

   #[arg(long)]
   o_format: String,
}

#[derive(Serialize, Deserialize)]
struct Data {
   name: String,
   age: u8,
   address: String,
   phones: [String; 2]
}

fn main() {
   let args = Args::parse();

   let mut file = File::open(args.i_file)
      .expect("can't open the file for reading");
   let mut reader = BufReader::new(file);

   let data: Data = match args.i_format.as_str() {
      "json" => serde_json::from_reader(reader)
         .expect("can't deserialize an object from file of JSON"),
      "yaml" => serde_yaml::from_reader(reader)
         .expect("can't deserialize an object from file of YAML"),
      "ron" => ron::de::from_reader(reader)
         .expect("can't deserialize an object from file of RON"),
      "toml" => {
         let mut temp = String::new();
         reader.read_to_string(&mut temp).unwrap();
         toml::de::from_str(temp.as_str())
            .expect("can't deserialize an object from file of TOML")
      }
      _ => panic!("invalid input file format")
   };

   file = File::create(args.o_file)
      .expect("can't open the file for writing");
   match args.o_format.as_str() {
      "json" => serde_json::to_writer_pretty(file, &data)
         .expect("can't serialize the object as JSON"),
      "yaml" => serde_yaml::to_writer(file, &data)
         .expect("can't serialize the object as YAML"),
      "ron" => ron::ser::to_writer(file, &data)
         .expect("can't serialize the object as RON"),
      "toml" => write!(file, "{}", toml::to_string(&data)
         .expect("can't serialize the object as TOML")).unwrap(),
      _ => panic!("invalid output file format")
   };
}
