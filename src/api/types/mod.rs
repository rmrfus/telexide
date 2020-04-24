//! This modules provides all the objects describing the payloads to be send to the different telegram API endpoints

mod commands;
mod send_messages;
mod updates;
mod webhooks;
mod edit_messages;
mod chat;
mod other;
mod input_media;
mod stickers;
mod games;
mod payments;
mod inline;
mod passport;

pub use commands::*;
pub use chat::*;
pub use send_messages::*;
pub use edit_messages::*;
pub use updates::{GetUpdates, UpdateType};
pub use other::*;
pub use input_media::*;
pub use stickers::*;
pub use games::*;
pub use payments::*;
pub use inline::*;
pub use passport::*;
pub use webhooks::SetWebhook;
