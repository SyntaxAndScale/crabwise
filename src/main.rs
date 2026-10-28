use std::{collections::HashSet, io::{Read}};
use cargo_lock::Lockfile;
use reqwest;
use tar;
use csv::Reader;
// use flate2::*;
// use std::io::prelude::*;

const CRATESDB_URL : &str = "https://static.crates.io/db-dump.tar.gz";

fn main() {
    println!("Hello, world!");
	println!("Packages: {:#?}", create_dependency_hashset());
	get_crates_db(CRATESDB_URL);
}

fn read_cargo_lock() -> Lockfile {
	let lockfile = Lockfile::load("Cargo.lock").unwrap();
	println!("Number of Packages: {:#?}", lockfile.packages.len());
	lockfile
}

fn create_dependency_hashset() -> HashSet<String> {
	let lockfile = read_cargo_lock();
	let mut hashset : HashSet<String>= HashSet::new();
	lockfile.packages.iter().for_each(|p| {
		hashset.insert(String::from(p.name.clone()));
	});
	hashset
}

struct CratesDB {
	crates: Crates,
	versions: Versions,
	default_versions: DefaultVersions,
	dependencies: Dependencies
}

struct Crates;
struct Versions;
struct DefaultVersions;
struct Dependencies;

fn get_crates_db(url: &str) -> Result<String, Box<dyn std::error::Error>> {
	let response = reqwest::blocking::get(url)?;
	let decoder = flate2::read::GzDecoder::new(response);
	let mut archive = tar::Archive::new(decoder);
	let mut entries = archive.entries()?;
	
	let mut num_entries_written = 0;
	for mut entry in entries {
		let mut entry_contents = entry?;
		
		let path = String::from(entry_contents.path()?.to_str().unwrap_or_default());
		println!("entry contents path: {}", path);
		let filename : &str = path.split('/').nth(1).unwrap_or_default();
// 		println!("parsed filename: {:#?}", filename); 
		if matches!(filename, "README.md" | "crates.csv" | "versions.csv" | "default_versions.csv" | "dependencies.csv") {
			if read_buffer(&path, &mut entry_contents).is_ok() {
				num_entries_written += 1;
			}
		};
		
		if num_entries_written == 1 {
			break;
		}
	}
	Ok(String::from("My String"))
}

fn read_buffer(path: &str, entry_contents: &mut impl Read) -> Result<String, Box<dyn std::error::Error>> {
	let mut buffer = String::new();
	entry_contents.read_to_string(&mut buffer)?;
	println!("{:#?} contains: {:#?}", path, buffer);
	Ok(String::from("Ok"))
}
