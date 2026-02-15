use std::{env, process, process::Command, fs};

const HELP: &str = "HELP:
====
arg 1 - speedrun.com api key from settings -> api key -> show api key
arg 2 - text file path
=
text file format:
player1ID,player2ID | Map Name | 1:23.456 | Gear/Gearless | main video link | comment(additional povs) | word 'split' if you are doing split duo, otherwise this column is not needed

-if you are submitting solo runs, you dont need to put your id, you can put your name or anything there.
each line is a seperate run.
";

fn check_dependancy_curl() -> bool {
	let status = Command::new("curl")
		.arg("--version")
	.status();

	match status {
		Ok(status) => status.success(), //pub fn success(&self) -> bool
		Err(_) => false, //this is maybe unreachable i think, only if a cosmic ray hits the computer this can happen
	}
}

fn parse_to_milliseconds(input: &str) -> String {
	let parts: Vec<&str> = input.split(':').collect();
		//last is 01.123
	let mut h = 0.0;
	let mut m = 0.0;
	let mut s = 0.0;
	let mut ms = 0.0;
	
	match parts.len() {
		1 => { //s
				s = parts[parts.len()-1].parse().expect("not a valid float");
			},
		2 => { //m s
				m = parts[parts.len()-2].parse().expect("not a valid float");
				s = parts[parts.len()-1].parse().expect("not a valid float");
			},
		3 => { //h m s
				h = parts[parts.len()-3].parse().expect("not a valid float");
				m = parts[parts.len()-2].parse().expect("not a valid float");
				s = parts[parts.len()-1].parse().expect("not a valid float");
			},
		_ => { eprintln!("INVALID TEXT FILE FORMAT, wrong time"); process::exit(5) },
	}
	
	ms = s*1000.0 + m*60.0 + h*60.0*60.0;
	
	return ms.to_string();
}

mod fetchlevels;
mod getlevelid;
mod createpayload;

fn main() {
	let args: Vec<_> = env::args().collect();
	
	if args.len() < 3 {
		eprintln!("{}", HELP);
		process::exit(1);
	}
	if !check_dependancy_curl() {
		eprintln!("CURL UNAVAILABLE");
		process::exit(2);
	}
	let api_key = &args[1];
	let text_file_path = &args[2];

	let contents =  fs::read_to_string(text_file_path).expect("CANT READ TEXT FILE");
	
	let lines: Vec<&str> = contents.lines().collect();
	
	fetchlevels::thing2();
	
	for i in 0..lines.len() {
		let mut parts: Vec<&str> = Vec::new();

		for part in lines[i].split('|') {
			parts.push(part.trim());
		}
		
		//let parts: Vec<&str> = lines[i].split('|').collect();
		
		if parts.len() < 6 {
			eprintln!("INVALID TEXT FILE FORMAT, columns dont add up");
			process::exit(3);
		}
		
		let players: Vec<&str> = parts[0].split(',').collect(); //players.len()
		
		//map name into Id
		let map_name = &parts[1];
		let mapId = getlevelid::thing3(map_name);
		
		//turn time into api/v1 format (milliseconds only)
		let time_format = &parts[2];
		let time = parse_to_milliseconds(time_format);
		
		//figure out gear variable
		let mut gear_variable_value = "";
		match parts[3] {
			"Gearless" => { gear_variable_value = "q757r8p1" },
			"Gear" => { gear_variable_value = "1gnx2m6l" },
			_ => { eprintln!("INVALID TEXT FILE FORMAT, Gear/Gearless"); process::exit(3); },
		}
		
		let main_video_link = &parts[4];
		let comment = &parts[5];
		
		//figure out category
		let mut split_duo = false;
		if parts.len() > 6 {
			if parts[6] == "split" {
				split_duo = true;
			}
		}
		let mut categoryId = "";
		if !split_duo {
			match players.len() {
				0 => { eprintln!("INVALID TEXT FILE FORMAT, invalid players column"); process::exit(3); },
				1 => { categoryId = "7kj0wn32" },
				2 => { categoryId = "xk9z7mv2" },
				3 => { categoryId = "z27zjx4k" },
				4 => { categoryId = "q25z6vj2" },
				_ => { categoryId = "jdzxzjr2" }
			}
		} else {
			categoryId = "xd1plgzd" //split duo
		}
		
		//debug printing
		println!("{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}", players, map_name, mapId, categoryId, gear_variable_value, time, main_video_link, comment );
		
		//now, assembling the data json, for the request
		let run_settings = createpayload::thing(players, &mapId, categoryId, gear_variable_value, &time, main_video_link, comment);
	
		println!("{}", run_settings);
		//submit runs maybe
		
		let output = Command::new("curl")
			.arg("-L")
			.arg("-X")
			.arg("POST")
			.arg("https://www.speedrun.com/api/v1/runs")
			.arg("-H")
			.arg("Content-Type: application/json")
			.arg("-H")
			.arg(format!("X-Api-Key: {}", api_key))
			.arg("--data-raw")
			.arg(run_settings)		
			.output()
		.expect("Failure");
				
		let string_to_print = String::from_utf8_lossy(&output.stdout);
		println!("{:?}", string_to_print);
		
	}
	/*
	Command::new("curl")
        .arg("-L")
        .arg("-X")
        .arg("POST")
        .arg("https://www.speedrun.com/api/v1/runs")
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("-H")
        .arg("X-Api-Key: ")
        .arg("--data-raw")
        .arg(runsettings)
        .arg("-o")
        .arg("output.json")
        .status()
        .expect("Failure");
        */
    println!("Done.");
}
