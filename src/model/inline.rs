use super::{Location, User};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InlineQuery {
    /// Unique identifier for this query
    pub id: String,
    /// Sender
    pub from: User,
    /// Sender location, only for bots that request user location
    pub location: Option<Location>,
    /// Text of the query (up to 256 characters)
    pub query: String,
    /// Offset of the results to be returned, can be controlled by the bot
    pub offset: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChosenInlineResult {
    /// The unique identifier for the result that was chosen
    pub result_id: String,
    /// The user that chose the result
    pub from: User,
    /// Sender location, only for bots that require user location
    pub location: Option<Location>,
    /// Identifier of the sent inline message.
    /// Available only if there is an inline keyboard attached to the message.
    /// Will be also received in callback queries and can be used to edit the message.
    pub query: String,
    /// The query that was used to obtain the result
    pub inline_message_id: Option<String>,
}