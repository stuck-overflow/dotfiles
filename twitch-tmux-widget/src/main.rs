mod token_storage;

use chrono::{DateTime, Utc};
use log::LevelFilter;
use serde::Deserialize;
use simple_logger::SimpleLogger;
use std::fs;
use structopt::StructOpt;
use twitch_api2::helix::streams::GetStreamsRequest;
use twitch_api2::helix::subscriptions::GetBroadcasterSubscriptionsRequest;
use twitch_api2::helix::users::GetUsersRequest;
use twitch_api2::twitch_oauth2::Scope;
use twitch_api2::TwitchClient;

#[derive(Clone, Deserialize)]
struct TwitchTmuxWidget {
    twitch: TwitchConfig,
}

#[derive(Clone, Deserialize)]
struct TwitchConfig {
    login_name: String,
    channel_name: String,
    client_id: String,
    client_secret: String,
    token_filepath: String,
}
// Command-line arguments for the tool.
#[derive(StructOpt)]
struct Cli {
    /// Log level
    #[structopt(short, long, case_insensitive = true, default_value = "ERROR")]
    log_level: LevelFilter,

    /// Twitch credential files.
    #[structopt(short, long, default_value = "twitch-tmux-widget.toml")]
    config_file: String,

    /// Obtain user token.
    #[structopt(short, long)]
    auth: bool,
}

#[tokio::main]
pub async fn main() {
    let args = Cli::from_args();
    SimpleLogger::new()
        .with_level(args.log_level)
        .init()
        .unwrap();

    let config = match fs::read_to_string(&args.config_file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Unable to load config from {}: {}", args.config_file, e);
            // print an empty line so tmux can render something.
            println!("");
            std::process::exit(1);
        }
    };

    let config = match toml::from_str::<TwitchTmuxWidget>(&config) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Unable to parse config from {}: {}", args.config_file, e);
            // print an empty line so tmux can render something.
            println!("");
            std::process::exit(1);
        }
    };

    if args.auth {
        let user_token = twitch_oauth2_auth_flow::auth_flow(
            &config.twitch.client_id,
            &config.twitch.client_secret,
            Some(vec![Scope::ChannelReadSubscriptions]),
        );
        token_storage::write_token_to_disk(
            user_token.clone(),
            Some(oauth2::ClientSecret::new(
                config.twitch.client_secret.clone(),
            )),
            &config.twitch.token_filepath,
        )
        .expect("Can't write token to disk");
    }

    let token = match token_storage::load_token_from_disk(&config.twitch.token_filepath) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error loading token: {}", e);
            println!("run w/ --auth");
            std::process::exit(1);
        }
    };

    let client = TwitchClient::with_client(surf::Client::new());
    let req = GetStreamsRequest::builder()
        .user_login(vec![config.twitch.login_name.clone()])
        .build();

    let response = client.helix.req_get(req, &token).await.unwrap();
    let viewers = if response.data.len() > 0 {
        response.data[0].viewer_count
    } else {
        0
    };
    let elapsed_stream_time = if response.data.len() > 0 {
        Utc::now().timestamp()
            - DateTime::parse_from_rfc3339(&response.data[0].started_at)
                .unwrap()
                .timestamp()
    } else {
        0
    };
    let secs = elapsed_stream_time % 60;
    let mins = elapsed_stream_time / 60;
    let mins_rem = mins % 60;
    let hrs = mins / 60;

    let formatted_time = format!("{:02}:{:02}:{:02}", hrs, mins_rem, secs);
    let req = GetUsersRequest::builder()
        .login(vec![config.twitch.login_name.clone()])
        .build();
    let req = client.helix.req_get(req, &token).await.unwrap();
    let broadcaster = req.data.get(0).unwrap();
    let req = GetBroadcasterSubscriptionsRequest::builder()
        .broadcaster_id(broadcaster.id.clone())
        .first(Some(String::from("100")))
        .build();
    let response = client.helix.req_get(req, &token).await.unwrap();
    let subs = if response.data.len() > 0 {
        response.data.len()
    } else {
        0
    };

    let display_rota = vec![
        format!("    Viewers: {}     ", viewers),
        format!("  Subscribers: {}  ", subs),
        format!("Stream 🕒: {}", formatted_time),
    ];

    let tick = Utc::now().timestamp() / 5;
    let display = (tick as usize) % display_rota.len();
    println!("{}", display_rota[display]);
}
