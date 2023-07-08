use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct EquiptmentCSV {
	equiptment: String,
}

fn main() {
	let mut reader = csv::Reader::from_reader(include_bytes!("../assets/card-csv-data/GC-1.csv"));
}
