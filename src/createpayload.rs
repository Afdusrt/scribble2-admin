use std::{fs, process, env};

use serde_json::json;

pub fn thing(
    players: Vec<&str>,
    map_name: &str,
    categoryId: &str,
    gear_variable_value: &str,
    time: &str,
    main_video_link: &str,
    comment: &str,
) -> String {
    let time_millis: u64 = time.parse().unwrap_or(0);

    let players_json: Vec<_> = players
        .into_iter()
        .map(|id| json!({"rel": "user", "id": id}))
        .collect();
	
    let variables_json = json!({
        "onv520ml": {
            "type": "pre-defined",
            "value": gear_variable_value
        }
    });

    // Build the full payload
    let mut payload = json!({});
    if players_json.len() != 1 {
		payload = json!({
        "run": {
            "category": categoryId,
            "level": map_name,
            "date": chrono::Utc::now().format("%Y-%m-%d").to_string(),
            "platform": "8gej2n93", //PC
            "times": {
                "realtime": time_millis
            },
            "players": players_json,
            "video": main_video_link,
            "comment": comment,
            "variables": variables_json
        }
		});
	} else {
		payload = json!({ //dont iclude players value for solo
			"run": {
				"category": categoryId,
				"level": map_name,
				"date": chrono::Utc::now().format("%Y-%m-%d").to_string(),
				"platform": "8gej2n93", //PC
				"times": {
					"realtime": time_millis
				},
				"video": main_video_link,
				"comment": comment,
				"variables": variables_json
			}
		});
	}

    return serde_json::to_string_pretty(&payload).unwrap();
}
