use std::{error::Error, fmt::Display};

use discord_sdk::{activity::Activity, Discord};
use serde::Serialize;
use tracing::error;

pub struct Client {
  pub discord: discord_sdk::Discord,
  pub user: discord_sdk::user::User,
  pub wheel: discord_sdk::wheel::Wheel,
  pub activity_set: bool,
}

impl Client {
  pub async fn new(application_id: i64) -> Result<Self, ClientError> {
    let (wheel, handler) = discord_sdk::wheel::Wheel::new(Box::new(|e| {
      error!("{}", e);
    }));

    let mut user = wheel.user();

    let discord = Discord::new(
      discord_sdk::DiscordApp::PlainId(application_id),
      discord_sdk::Subscriptions::ACTIVITY,
      Box::new(handler),
    )
    .map_err(|e| ClientError::Create(e))?;

    user
      .0
      .changed()
      .await
      .map_err(|e| ClientError::ConnectionDropped(e))?;

    let user = match &*user.0.borrow() {
      discord_sdk::wheel::UserState::Connected(user) => Ok(user.clone()),
      discord_sdk::wheel::UserState::Disconnected(e) => {
        Err(ClientError::Disconnected(e.to_string()))
      }
    }?;

    Ok(Self {
      discord,
      user,
      wheel,
      activity_set: false,
    })
  }

  pub async fn clear_activity(&mut self) -> Result<Option<Activity>, discord_sdk::Error> {
    let activity = self.discord.clear_activity().await?;
    self.activity_set = false;
    crate::log::info("Discord activity cleared");
    Ok(activity)
  }

  pub fn user_data(&self) -> UserData {
    let user = &self.user;
    UserData {
      id: user.id.0,
      discriminator: user.discriminator.unwrap_or_else(|| 0000),
      name: user.username.clone(),
    }
  }
}

#[derive(Serialize, Debug, Clone)]
pub struct UserData {
  id: u64,
  discriminator: u32,
  name: String,
}

#[derive(Debug)]
pub enum ClientError {
  Create(discord_sdk::Error),
  ConnectionDropped(tokio::sync::watch::error::RecvError),
  Disconnected(String),
}

impl Display for ClientError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Create(e) => f.write_fmt(format_args!("Error creating Discord client: {}", e)),
      Self::ConnectionDropped(e) => f.write_fmt(format_args!("Discord connection dropped: {}", e)),
      Self::Disconnected(e) => f.write_fmt(format_args!("Couldn't connect to Discord: {}", e)),
    }
  }
}

impl Error for ClientError {
  fn cause(&self) -> Option<&dyn Error> {
    Some(self)
  }
}
