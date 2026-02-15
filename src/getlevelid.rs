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

use std::{ fs };

pub fn thing3(map_name: &str) -> String {
	let levels_json = fs::read_to_string("levels.json").expect("Could not read levels.json file");
	
	let formatted_name = format!("\"{}\"", map_name);
	
	if let Some(level_name_index) = levels_json.find(&formatted_name) {
        return levels_json[(level_name_index - 17)..(level_name_index - 9)].to_string();
    }

    panic!("Level '{}' not found", map_name);
}

