use std::{collections::{HashSet,HashMap}, io::{Read}};
use cargo_lock::Lockfile;
use reqwest;
use tar;
use csv::Reader;
// use flate2::*;
// use std::io::prelude::*;
const OSE_RESULTS_URL: &str = "https://github.com/osendowment/model/raw/refs/heads/main/data/sources/crates/results.csv";
const OSE_RISK_URL: &str = "https://raw.githubusercontent.com/osendowment/model/refs/heads/main/data/risk/risk.csv";

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let project_dependencies = get_project_deps();
	let project_deps_result_table = get_deps_result(&project_dependencies)?;
	let project_deps_risk_table = get_deps_risk(&project_deps_result_table)?;
// 	println!("Project Dependency Result Table: {:#?}", project_deps_result_table);
// 	println!("Project Dependencies: {:#?}", project_dependencies);
	println!("Project Risk Scores: {:#?}", project_deps_risk_table);
// 	get_crates_db(CRATESDB_URL);
	Ok(())
}

fn get_deps_risk(project_deps_result_table: &HashSet<OSEResultsRecord>) -> Result<HashSet<OSERiskRecord>, Box<dyn std::error::Error>> {
	let mut deps_risk : HashSet<OSERiskRecord> = HashSet::new();
	
	let response = reqwest::blocking::get(OSE_RISK_URL)?;
	let mut reader = csv::Reader::from_reader(response);
	
	let repo_ids: Vec<Option<String>> = project_deps_result_table.iter().map(|r| r.repo_id.clone()).collect();
	
	for riskrecord in reader.deserialize::<OSERiskRecord>() {
		let record: OSERiskRecord = riskrecord?;
		if repo_ids.contains(&record.repo_id) {
			deps_risk.insert(record.clone());
		}
			
// 			if resultrecord.repo_id == record.repo_id {
// 				deps_risk.insert(record);
// 			}
	}
	
	Ok(deps_risk)
}

fn get_deps_result(project_deps: &HashSet<String>) -> Result<HashSet<OSEResultsRecord>,Box<dyn std::error::Error>> {
	let mut deps_result : HashSet<OSEResultsRecord> = HashSet::new();
	
	let response = reqwest::blocking::get(OSE_RESULTS_URL)?;
	let mut reader = csv::Reader::from_reader(response);

	for result in reader.deserialize::<OSEResultsRecord>() {
		let record: OSEResultsRecord = result?;
		if project_deps.contains(&record.package) {
			deps_result.insert(record.clone());
		}
	}
	Ok(deps_result)
}

fn get_project_deps() -> HashSet<String> {
	let lockfile = read_cargo_lock();
	let mut hashset : HashSet<String> =  HashSet::new();
	lockfile.packages.iter().for_each(|p| {
		hashset.insert(String::from(p.name.clone()));
	});
	hashset
}

fn read_cargo_lock() -> Lockfile {
	let lockfile = Lockfile::load("Cargo.lock").unwrap();
	println!("Number of Packages: {:#?}", lockfile.packages.len());
	lockfile
}

#[derive(Debug, serde::Deserialize, Clone, Eq, Hash, PartialEq)]
struct OSERiskRecord {
	repo: String,
	repo_id: Option<String>,
	concentration: Option<String>,
	complexity: Option<String>,
	security: Option<String>,
	workload: Option<String>,
	risk_score: Option<String>
}

#[derive(Debug, serde::Deserialize, Clone, Eq, Hash, PartialEq)]
struct OSEResultsRecord {
	package: String,
	github_repo: Option<String>,
	
	#[serde(rename(deserialize = "avg_downloads"))]
	downlads_avg: usize, //may need to 'grow' these to u64
	
	#[serde(rename(deserialize = "2021"))]
	downlads_2021: usize, 
	
	#[serde(rename(deserialize = "2022"))]
	downlads_2022: usize, 
	
	#[serde(rename(deserialize = "2023"))]
	downloads_2023: usize,
	
	#[serde(rename(deserialize = "2024"))]
	downloads_2024: usize,
	
	#[serde(rename(deserialize = "2025"))]
	downloads_2025: usize,
	
	#[serde(rename(deserialize = "top"))]
	is_top: String, //should be cast into bool post-Deeserialize
	pagerank: String, //should be f32, but would have to be done post-Deserialize
	value_class: char,
	git: Option<String>, 
	eco_guess: Option<String>, 
	repo_id: Option<String>,
	canonical_url: Option<String>,
	license: Option<String>
}

const CRATESDB_URL : &str = "https://static.crates.io/db-dump.tar.gz";

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
