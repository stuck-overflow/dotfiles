use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io;
use twitch_api2::twitch_oauth2;
use twitch_api2::twitch_oauth2::TwitchToken;

// Since twitch_oauth2::UserToken is not serializable, create our own
// serializable struct. This struct can be converted to either
// twitch_oauth2::UserToken or twitch_irc::login::UserAccesstoken.
#[derive(Deserialize, Serialize)]
pub struct StoredUserToken {
    /// The access token used to authenticate requests with
    access_token: oauth2::AccessToken,
    client_id: oauth2::ClientId,
    client_secret: Option<oauth2::ClientSecret>,
    /// Username of user associated with this token
    login: String,
    /// User ID of the user associated with this token
    user_id: String,
    /// The refresh token used to extend the life of this user token
    refresh_token: Option<oauth2::RefreshToken>,
    /// Expiration time
    expires_at: Option<DateTime<Utc>>,
    scopes: Option<Vec<twitch_oauth2::Scope>>,
}

pub fn load_token_from_disk(token_filepath: &str) -> io::Result<twitch_oauth2::UserToken> {
    let token = fs::read_to_string(token_filepath)?;
    let token = serde_json::from_str::<StoredUserToken>(&(token)).map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Failed to deserialize token",
        )
    })?;
    let expires_at = token.expires_at.ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Stored token doesn't have an expiration time",
        )
    })?;

    let expires_in = match expires_at.signed_duration_since(Utc::now()).to_std() {
        Ok(e) => e,
        Err(_) => {
            eprintln!("Token expired at {:?}, will attempt refresh", expires_at);
            std::time::Duration::from_secs(0)
        }
    };

    let mut user_token = twitch_oauth2::UserToken::from_existing_unchecked(
        token.access_token.clone(),
        token.refresh_token.clone(),
        token.client_id.clone(),
        token.client_secret.clone(),
        token.login.clone(),
        token.user_id.clone(),
        token.scopes.clone(),
        Some(expires_in),
    );

    let expires_in = expires_in.as_secs();
    if expires_in < 3600 {
        if expires_in > 0 {
            eprintln!(
                "Token expiring in {} seconds, attempting refresh",
                expires_in
            );
        }

        match futures::executor::block_on(
            user_token.refresh_token(twitch_oauth2::client::surf_http_client),
        ) {
            Ok(()) => write_token_to_disk(
                user_token.clone(),
                token.client_secret.clone(),
                token_filepath,
            )?,
            Err(e) => {
                eprintln!("Error refreshing token {}", e);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Can't refresh token",
                ));
            }
        }
    }

    Ok(user_token)
}

pub fn write_token_to_disk(
    user_token: twitch_oauth2::UserToken,
    client_secret: Option<oauth2::ClientSecret>,
    token_filepath: &str,
) -> io::Result<()> {
    let expires_at = Utc::now() + Duration::from_std(user_token.expires_in()).unwrap();
    let stored_token = StoredUserToken {
        access_token: user_token.access_token.clone(),
        client_id: user_token.client_id().clone(),
        client_secret,
        login: user_token.login.clone(),
        user_id: user_token.user_id.clone(),
        refresh_token: user_token.refresh_token.clone(),
        expires_at: Some(expires_at),
        scopes: Some(user_token.scopes().to_vec()),
    };
    let serialized = serde_json::to_string(&stored_token).unwrap();
    let _ = File::create(token_filepath);
    fs::write(token_filepath, serialized).expect("Unable to write token to checkpoint file");
    Ok(())
}
