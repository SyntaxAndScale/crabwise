use std::collections::HashSet;
use cargo_lock::Lockfile;
use reqwest;
use comfy_table::Table;

const OSE_RESULTS_URL: &str = "https://github.com/osendowment/model/raw/refs/heads/main/data/sources/crates/results.csv";
const OSE_RISK_URL: &str = "https://raw.githubusercontent.com/osendowment/model/refs/heads/main/data/risk/risk.csv";

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let project_dependencies = get_project_deps()?;
	let project_deps_result_table = get_deps_result(&project_dependencies)?;
	let project_deps_risk_table = get_deps_risk(&project_deps_result_table)?;
	print_risk_table(project_deps_risk_table);
	Ok(())
}

fn print_risk_table(risk_table: HashSet<OSERiskRecord>) {

	let mut table = Table::new();
	
	table.set_header(vec!["Repo","Risk", "Security", "Concentration","Complexity", "Workload"]);
	for record in risk_table {
		table.add_row(vec![
			record.clone().repo,
			record.clone().risk_score.unwrap_or_default(), 
			record.clone().security.unwrap_or_default(),
			record.clone().concentration.unwrap_or_default(),
			record.clone().complexity.unwrap_or_default(),
			record.clone().workload.unwrap_or_default()
		]);
	}
	println!("{table}");
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

fn get_project_deps() -> Result<HashSet<String>, Box<dyn std::error::Error>> {
	let lockfile = read_cargo_lock()?;
	let mut hashset : HashSet<String> =  HashSet::new();
	lockfile.packages.iter().for_each(|p| {
		hashset.insert(String::from(p.name.clone()));
	});
	Ok(hashset)
}

fn read_cargo_lock() -> Result<Lockfile, Box<dyn std::error::Error>> {
	let lockfile = Lockfile::load("Cargo.lock")?;
	Ok(lockfile)
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