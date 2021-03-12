use futures::executor::block_on;
use log::LevelFilter;
use serde::Deserialize;
use simple_logger::SimpleLogger;
use std::fs;
use structopt::StructOpt;
use twitch_api2::helix::streams::GetStreamsRequest;
use twitch_api2::helix::subscriptions::GetBroadcasterSubscriptionsRequest;
use twitch_api2::helix::users::GetUsersRequest;
use twitch_api2::TwitchClient;
use twitch_oauth2::{client::surf_http_client, AppAccessToken, Scope};
use twitch_oauth2::{ClientId, ClientSecret};

#[derive(Clone, Deserialize)]
struct TmuxWidget {
    twitch: TwitchConfig,
}

#[derive(Clone, Deserialize)]
struct TwitchConfig {
    login_name: String,
    channel_name: String,
    client_id: String,
    client_secret: String,
}
// Command-line arguments for the tool.
#[derive(StructOpt)]
struct Cli {
    /// Log level
    #[structopt(short, long, case_insensitive = true, default_value = "ERROR")]
    log_level: LevelFilter,

    /// Twitch credential files.
    #[structopt(short, long, default_value = "tmuxwidget.toml")]
    config_file: String,
}

async fn do_request() {
    let args = Cli::from_args();
    SimpleLogger::new()
        .with_level(args.log_level)
        .init()
        .unwrap();

    let config = fs::read_to_string(args.config_file).unwrap();
    let config: TmuxWidget = toml::from_str(&config).unwrap();

    let token = match AppAccessToken::get_app_access_token(
        surf_http_client,
        ClientId::new(config.twitch.client_id),
        ClientSecret::new(config.twitch.client_secret),
        vec![Scope::ChannelReadSubscriptions],
    )
    .await
    {
        Ok(t) => t,
        Err(e) => panic!("got error: {:?}", e),
    };
    let client = TwitchClient::with_client(surf::Client::new());
    let req = GetStreamsRequest::builder()
        .user_login(vec![config.twitch.login_name.clone()])
        .build();

    let response = client.helix.req_get(req, &token).await.unwrap();

    println!("Viewers: {}", response.data[0].viewer_count);

    // let req = GetUsersRequest::builder()
    //     .login(vec![config.twitch.login_name.clone()])
    //     .build();
    // let req = client.helix.req_get(req, &token).await.unwrap();
    // let broadcaster = req.data.get(0).unwrap();
    // let req = GetBroadcasterSubscriptionsRequest::builder()
    //     .broadcaster_id(broadcaster.id.clone())
    //     .build();
    // println!(
    //     "{:?}",
    //     client.helix.req_get(req, &token).await
    //         .unwrap());
}

fn main() {
    block_on(do_request())
}
