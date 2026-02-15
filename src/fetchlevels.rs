use std::{ fs, process::Command };

fn decode_weird_unicode(s: &str) -> String {
	let mut result = String::new();
	let mut chars = s.chars().peekable();
	
	while let Some(c) = chars.next() {
		if c == '\\' && chars.peek() == Some(&'u') { //check if there is a sorta \uFFFF 
			chars.next();
			let hex: String = chars.by_ref().take(4).collect();
			if let Ok(code_point) = u32::from_str_radix(&hex, 16) { //convert hex unicode string to u32
			if let Some(ch) = std::char::from_u32(code_point) {
					result.push(ch);
				}
			}
		} else {
			result.push(c);
		}
	}
	
	result
}

pub fn thing2() {
	Command::new("curl")
		.arg("-L")
		.arg("http://www.speedrun.com/api/v1/games/color_book/levels")
		.arg("-o")
		.arg("levels.json")
		.status()
	.expect("Failure");
	
	/*let contents = fs::read_to_string(dsv_file_path)
		.expect("Didnt read file");*/
		
	let raw_levels_json = fs::read_to_string("levels.json")
		.expect("Didnt read file");
	
	let levels_json = decode_weird_unicode(&raw_levels_json);
	
	fs::write("levels.txt", &levels_json).expect("Could not write levels.json file");
	println!("saved levels.json");
}
