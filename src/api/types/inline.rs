use serde::{Deserialize, Serialize};
use crate::model::{InlineKeyboardMarkup, ParseMode};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AnswerInlineQuery {
    /// Unique identifier for the answered query
    pub inline_query_id: String,
    /// A vec of results for the inline query
    pub results: Vec<InlineQueryResult>,
    /// The maximum amount of time in seconds that the result of the inline query may be cached on the server.
    /// Defaults to 300.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i64>,
    /// Pass True, if results may be cached on the server side only for the user that sent the query.
    /// By default, results may be returned to any user who sends the same query
    pub is_personal: bool,
    /// Pass the offset that a client should send in the next query with the same text to receive more results.
    /// Pass an empty string if there are no more results or if you don‘t support pagination.
    /// Offset length can’t exceed 64 bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<String>,
    /// If passed, clients will display a button with specified text that switches the user to a
    /// private chat with the bot and sends the bot a start message with the parameter switch_pm_parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_pm_text: Option<String>,
    /// [Deep-linking](https://core.telegram.org/bots#deep-linking) parameter for the /start message sent to the bot when user presses the switch button.
    /// 1-64 characters, only A-Z, a-z, 0-9, _ and - are allowed.
    ///
    /// Example: An inline bot that sends YouTube videos can ask the user to connect the bot to their YouTube account to adapt search results accordingly.
    /// To do this, it displays a ‘Connect your YouTube account’ button above the results, or even before showing any.
    /// The user presses the button, switches to a private chat with the bot and, in doing so,
    /// passes a start parameter that instructs the bot to return an oauth link. Once done,
    /// the bot can offer a [switch_inline button](https://core.telegram.org/bots/api#inlinekeyboardmarkup)
    /// so that the user can easily return to the chat where they wanted to use the bot's inline capabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_pm_parameter: Option<String>,
}

/// This object represents one result of an inline query.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum InlineQueryResult {
    #[serde(rename = "article")]
    Article(InlineQueryResultArticle),
    #[serde(rename = "audio")]
    Audio(InlineQueryResultAudio),
    #[serde(rename = "contact")]
    Contact(InlineQueryResultContact),
    #[serde(rename = "game")]
    Game(InlineQueryResultGame),
    #[serde(rename = "document")]
    Document(InlineQueryResultDocument),
    #[serde(rename = "gif")]
    Gif(InlineQueryResultGif),
    #[serde(rename = "location")]
    Location(InlineQueryResultLocation),
    #[serde(rename = "mpeg4_gif")]
    Mpeg4Gif(InlineQueryResultMpeg4Gif),
    #[serde(rename = "photo")]
    Photo(InlineQueryResultPhoto),
    #[serde(rename = "venue")]
    Venue(InlineQueryResultVenue),
    #[serde(rename = "video")]
    Video(InlineQueryResultVideo),
    #[serde(rename = "voice")]
    Voice(InlineQueryResultVoice),
}

// TODO: add support for the cached types too. Add enum with url and cache variant?

/// Represents a link to an article or web page.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InlineQueryResultArticle {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Title of the result
    pub title: String,
    /// Content of the message to be sent
    pub input_message_content: InputMessageContent,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// URL of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Pass True, if you don't want the URL to be shown in the message
    pub hide_url: bool,
    /// Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Url of the thumbnail for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,
    /// Thumbnail width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<i64>,
    /// Thumbnail height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<i64>
}

/// Represents a link to a photo. By default, this photo will be sent by the user with optional caption.
/// Alternatively, you can use input_message_content to send a message with the specified content instead of the photo.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InlineQueryResultPhoto {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// A valid URL of the photo. Photo must be in jpeg format. Photo size must not exceed 5MB
    pub photo_url: String,
    /// Url of the thumbnail for the photo
    pub thumb_url: String,
    /// Photo width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<i64>,
    /// Photo height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<i64>,
    /// Title of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Caption of the photo to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Content of the message to be sent instead of the photo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in your bot's message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
}

/// Represents a link to an animated GIF file. By default,
/// this animated GIF file will be sent by the user with optional caption.
/// Alternatively, you can use input_message_content to send a message with the specified content instead of the animation.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InlineQueryResultGif {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// A valid URL for the GIF file. File size must not exceed 1MB
    pub gif_url: String,
    /// URL of the static thumbnail for the result (jpeg or gif)
    pub thumb_url: String,
    /// Width of the GIF
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_width: Option<i64>,
    /// Height of the GIF
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_height: Option<i64>,
    /// Duration of the GIF
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_duration: Option<i64>,
    /// Title of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Caption of the gif to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Content of the message to be sent instead of the gif
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in your bot's message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
}

/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound).
/// By default, this animated MPEG-4 file will be sent by the user with optional caption.
/// Alternatively, you can use input_message_content to send a message with the specified content instead of the animation.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InlineQueryResultMpeg4Gif {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// A valid URL for the MP4 file. File size must not exceed 1MB
    pub mpeg4_url: String,
    /// URL of the static thumbnail (jpeg or gif) for the result
    pub thumb_url: String,
    /// Width of the video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_width: Option<i64>,
    /// Height of the video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_height: Option<i64>,
    /// Duration of the video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_duration: Option<i64>,
    /// Title of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Caption of the MPEG-4 file to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Content of the message to be sent instead of the video animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in your bot's message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
}

/// Represents a link to a page containing an embedded video player or a video file.
/// By default, this video file will be sent by the user with an optional caption.
/// Alternatively, you can use input_message_content to send a message with the specified content instead of the video.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InlineQueryResultVideo {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// A valid URL for the embedded video player or video file
    pub video_url: String,
    /// URL of the thumbnail (jpeg only) for the video
    pub thumb_url: String,
    /// Mime type of the content of video url, “text/html” or “video/mp4”
    pub mime_type: String,
    /// Width of the video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_width: Option<i64>,
    /// Height of the video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_height: Option<i64>,
    /// Duration of the video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_duration: Option<i64>,
    /// Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Title of the result
    pub title: String,
    /// Caption of the video to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Content of the message to be sent instead of the video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in your bot's message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
}

/// Represents a link to an MP3 audio file. By default, this audio file will be sent by the user.
/// Alternatively, you can use input_message_content to send a message with the specified content instead of the audio.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InlineQueryResultAudio {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the audio file
    pub audio_url: String,
    /// Title of the result
    pub title: String,
    /// Caption of the audio to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Audio performer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    /// Audio duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_duration: Option<i64>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in your bot's message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Content of the message to be sent instead of the audio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Represents a link to a voice recording in an .OGG container encoded with OPUS.
/// By default, this voice recording will be sent by the user.
/// Alternatively, you can use input_message_content to send a message with the specified content instead of the the voice message.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InlineQueryResultVoice {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the voice recording
    pub voice_url: String,
    /// Title of the result
    pub title: String,
    /// Caption of the audio to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Recording duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_duration: Option<i64>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in your bot's message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Content of the message to be sent instead of the voice recording
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Represents a link to a file. By default, this file will be sent by the user with an optional caption.
/// Alternatively, you can use input_message_content to send a message with the specified content instead of the file.
/// Currently, only .PDF and .ZIP files can be sent using this method.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InlineQueryResultDocument {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the file
    pub document_url: String,
    /// Title of the result
    pub title: String,
    /// Mime type of the content of the file, either “application/pdf” or “application/zip”
    pub mime_type: String,
    /// Caption of the audio to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Recording duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_duration: Option<i64>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in your bot's message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Content of the message to be sent instead of the document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// URL of the thumbnail (jpeg only) for the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,
    /// Thumbnail width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<i64>,
    /// Thumbnail height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<i64>
}

/// Represents a location on a map. By default, the location will be sent by the user.
/// Alternatively, you can use input_message_content to send a message with the specified content instead of the location.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InlineQueryResultLocation {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// Location latitude in degrees
    pub latitude: f64,
    /// Location longitude in degrees
    pub longitude: f64,
    /// Title of the location
    pub title: String,
    /// Period in seconds for which the location can be updated, should be between 60 and 86400.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<i64>,
    /// Content of the message to be sent instead of the location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Url of the thumbnail for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,
    /// Thumbnail width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<i64>,
    /// Thumbnail height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<i64>
}

/// Represents a venue. By default, the venue will be sent by the user.
/// Alternatively, you can use input_message_content to send a message with the specified content instead of the venue.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InlineQueryResultVenue {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// Latitude of the venue location in degrees
    pub latitude: f64,
    /// Longitude of the venue location in degrees
    pub longitude: f64,
    /// Title of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Foursquare identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue, if known.
    /// (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
    /// Period in seconds for which the location can be updated, should be between 60 and 86400.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<i64>,
    /// Content of the message to be sent instead of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Url of the thumbnail for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,
    /// Thumbnail width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<i64>,
    /// Thumbnail height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<i64>
}

/// Represents a contact with a phone number. By default, this contact will be sent by the user.
/// Alternatively, you can use input_message_content to send a message with the specified content instead of the contact.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InlineQueryResultContact {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Contact's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Additional data about the contact in the form of a vCard, 0-2048 bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
    /// Content of the message to be sent instead of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Url of the thumbnail for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,
    /// Thumbnail width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<i64>,
    /// Thumbnail height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<i64>
}

/// Represents a Game.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InlineQueryResultGame {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// Short name of the game
    pub game_short_name: String,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// This object represents the content of a message to be sent as a result of an inline query.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum InputMessageContent {
    Text(InputTextMessageContent),
    Location(InputLocationMessageContent),
    Venue(InputVenueMessageContent),
    Contact(InputContactMessageContent)
}

/// Represents the content of a text message to be sent as the result of an inline query.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InputTextMessageContent {
    /// Text of the message to be sent, 1-4096 characters
    pub message_text: String,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic,
    /// fixed-width text or inline URLs in your bot's message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Disables link previews for links in the sent message
    pub disable_web_page_preview: bool
}

/// Represents the content of a location message to be sent as the result of an inline query.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InputLocationMessageContent {
    /// Latitude of the location in degrees
    pub latitude: f64,
    /// Longitude of the location in degrees
    pub longitude: f64,
    /// Period in seconds for which the location can be updated, should be between 60 and 86400.
    pub live_period: i64
}

/// Represents the content of a venue message to be sent as the result of an inline query.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InputVenueMessageContent {
    /// Latitude of the venue in degrees
    pub latitude: f64,
    /// Longitude of the venue in degrees
    pub longitude: f64,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Foursquare identifier of the venue, if known
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue, if known.
    /// (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>
}

/// Represents the content of a contact message to be sent as the result of an inline query.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InputContactMessageContent {
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Contact's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Additional data about the contact in the form of a vCard, 0-2048 bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>
}