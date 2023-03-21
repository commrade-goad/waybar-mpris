use mpris::{PlaybackStatus, PlayerFinder};

fn connect() -> Result<mpris::Player, ()> {
    let player = PlayerFinder::new()
        .expect("Couldn't connect to D-bus!")
        .find_active();
    if let Ok(player) = player {
        return Ok(player);
    } else {
        return Err(());
    }
}

fn get_metadata(player_name: &mpris::Player) -> Vec<String> {
    let mut data: Vec<String> = Vec::new();
    let metadata = player_name.get_metadata().expect("Cant get the metadata!");
    data.push(player_name.bus_name_player_name_part().to_string());
    if let Some(mpris::MetadataValue::String(title)) = metadata.get("xesam:title") {
        data.push(title.to_owned());
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
                }
            }
        } else {
            if let Some(mpris::MetadataValue::String(artist_str)) = artist.get(0) {
                data.push(artist_str.to_string());
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

fn get_status(player_name: &mpris::Player) -> PlaybackStatus {
    let player_status: PlaybackStatus = mpris::Player::get_playback_status(&player_name)
        .expect("Error : Cant get the player status!");
    return player_status;
}

fn main() {
    let arg: Vec<String> = std::env::args().collect();
    let interval:u64 = arg[0].parse().unwrap_or(1000);
    loop {
        let player = connect();
        match player {
            Ok(v) => {
                let metadata = get_metadata(&v);
                let status = get_status(&v);
                let mut icons: String = String::new();
                match status {
                    PlaybackStatus::Paused => icons.push_str(""),
                    PlaybackStatus::Playing => icons.push_str(""),
                    PlaybackStatus::Stopped => icons.push_str(""),
                }
                let text: String = format!("{icons} {} - {}", metadata[2], metadata[1]);
                let class: String = format!("custom-{}", metadata[0]);
                let tooltip_b: String = format!("{} by {}", metadata[1], metadata[2]);
                let mut tooltip = String::new();
                if tooltip_b.chars().count() > 100 {
                    tooltip.push_str(&"Too long...".to_string());
                } else {
                    tooltip.push_str(&tooltip_b);
                }

                println!(
                    "{{\"text\":\"{}\", \"tooltip\": \"{}\", \"class\": \"{}\", \"alt\": \"{}\"}}",
                    &text, &tooltip, &class, &metadata[0]
                );
            }
            Err(()) => {
                println!("\n");
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(interval));
    }
}
