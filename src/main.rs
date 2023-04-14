use mpris::{PlaybackStatus, PlayerFinder};

fn connect() -> Result<mpris::Player, ()> {
    let player = match PlayerFinder::new() {
        Err(_) => return Err(()),
        Ok(v) => v,
    }
    .find_active();
    if let Ok(player) = player {
        return Ok(player);
    } else {
        return Err(());
    }
}

fn get_metadata(player_name: &mpris::Player) -> Vec<String> {
    let mut data: Vec<String> = Vec::new();
    let metadata = match player_name.get_metadata() {
        Err(e) => {
            // no metadata but there is player
            println!("Error : {e}");
            data.push("na".to_string()); // name of player will be `na`
            data.push("Not available".to_string()); // title will be `Not available`
            data.push("None".to_string()); // artist will be `None`
            return data;
        }
        Ok(v) => v,
    };
    data.push(player_name.bus_name_player_name_part().to_string());
    if let Some(mpris::MetadataValue::String(title)) = metadata.get("xesam:title") {
        data.push(title.to_owned());
    } else {
        data.push("Not available".to_string());
    };
    if let Some(mpris::MetadataValue::Array(artist)) = metadata.get("xesam:artist") {
        if artist.len() > 1 {
            let mut data_to_push: String = String::new();
            for n in 0..artist.len() {
                if let Some(mpris::MetadataValue::String(artist_str)) = artist.get(n) {
                    match n {
                        0 => data_to_push.push_str(&artist_str.to_string()),
                        _ => data_to_push.push_str(&format!(", {artist_str}")),
                    };
                } else {
                    data.push("None".to_string());
                }
            }
        } else {
            if let Some(mpris::MetadataValue::String(artist_str)) = artist.get(0) {
                data.push(artist_str.to_string());
            } else {
                data.push("None".to_string());
            }
        }
    };
    if data.len() < 3 {
        data.push("None".to_string());
    } else if data[2].chars().count() < 1 {
        data[2].push_str(&"None");
    }
    return data;
}

fn get_status(player_name: &mpris::Player) -> Result<PlaybackStatus, ()> {
    let player_status= mpris::Player::get_playback_status(&player_name);
    if let Ok(value) = player_status {
        return Ok(value);
    } else {
        return Err(());
    };
}

fn print_json(status: PlaybackStatus, metadata:Vec<String>) {
    let mut icons: String = String::new();
    let mut output_1: String = String::new();
    let mut output_2: String = String::new();
    match status {
        PlaybackStatus::Paused => icons.push_str(""),
        PlaybackStatus::Playing => icons.push_str(""),
        PlaybackStatus::Stopped => icons.push_str(""),
    }
    let mut metadata_filtered: Vec<String> = Vec::new();
    for arg in 0..metadata.len(){
        let arg_with_filter = metadata[arg].replace("\"", "\\\"");
        metadata_filtered.push(format!("{}",arg_with_filter));
    }
    if metadata_filtered[2].chars().count() > 50 || metadata_filtered[2] == "None" {
        output_1.push_str(&metadata_filtered[1]);
        output_2.push_str(&metadata_filtered[2]);
    } else {                              // default => (artist) - (title)
        output_1.push_str(&metadata_filtered[2]);
        output_2.push_str(&metadata_filtered[1]);
    }
    let text: String = format!("{icons} {} - {} ", output_1, output_2);
    let class: String = format!("custom-{}", metadata_filtered[0]);
    let tooltip_b: String = format!("{} by {}", metadata_filtered[1], metadata_filtered[2]);
    let mut tooltip = String::new();
    if tooltip_b.chars().count() > 75 {
        tooltip.push_str(&format!("{}", output_2));
    } else {
        tooltip.push_str(&tooltip_b);
    }

    println!(
        "{{\"text\":\"{}\", \"tooltip\": \"{}\", \"class\": \"{}\", \"alt\": \"{}\"}}",
        &text, &tooltip, &class, &metadata_filtered[0]
        );
}

fn main() {
    let mut arg: Vec<String> = std::env::args().collect();
    if arg.len() < 2 {
        arg.push("1000".to_string());
    }
    let interval: u64 = arg[1].parse().unwrap_or(1000);
    loop {
        let player = connect();
        match player {
            Err(_) => {
                println!("\n");
            }
            Ok(v) => {
                let metadata = get_metadata(&v);
                let status = get_status(&v);
                if let Ok(value) = status {
                    print_json(value, metadata);
                } else {
                    println!("\n");
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(interval));
    }
}
