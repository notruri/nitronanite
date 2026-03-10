use crate::snowflake::Snowflake;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Message {
    #[serde(rename = "type")]
    pub message_type: u8,
    pub content: String,
    #[serde(default)]
    pub mentions: Vec<Value>,
    #[serde(default)]
    pub mention_roles: Vec<String>,
    #[serde(default)]
    pub attachments: Vec<Value>,
    #[serde(default)]
    pub embeds: Vec<Value>,
    pub timestamp: String,
    pub edited_timestamp: Option<String>,
    pub flags: u64,
    #[serde(default)]
    pub components: Vec<Value>,
    pub id: Snowflake,
    pub channel_id: Snowflake,
    pub author: Author,
    pub pinned: bool,
    pub mention_everyone: bool,
    pub tts: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Author {
    pub id: Snowflake,
    pub username: String,
    pub avatar: Option<String>,
    pub discriminator: String,
    pub public_flags: u64,
    pub flags: u64,
    pub banner: Option<String>,
    pub accent_color: Option<i64>,
    pub global_name: Option<String>,
    pub avatar_decoration_data: Option<AvatarDecorationData>,
    pub collectibles: Option<Value>,
    pub display_name_styles: Option<Value>,
    pub banner_color: Option<String>,
    pub clan: Option<Value>,
    pub primary_guild: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AvatarDecorationData {
    pub asset: String,
    pub sku_id: Snowflake,
    pub expires_at: Option<String>,
}

pub fn parse_messages(input: &str) -> Result<Vec<Message>, serde_json::Error> {
    serde_json::from_str(input)
}

#[cfg(test)]
mod tests {
    use super::parse_messages;

    const SAMPLE_MESSAGES: &str = r#"
[
    {
        "type": 0,
        "content": "we play if you guys are up for it",
        "mentions": [],
        "mention_roles": [],
        "attachments": [],
        "embeds": [],
        "timestamp": "2026-03-10T12:41:45.441000+00:00",
        "edited_timestamp": null,
        "flags": 0,
        "components": [],
        "id": "1480908521355874518",
        "channel_id": "1451259544243015942",
        "author": {
            "id": "409287360946372619",
            "username": "kinugasakaede",
            "avatar": "7e29142c0c3d897d734d61145397a454",
            "discriminator": "0",
            "public_flags": 0,
            "flags": 0,
            "banner": null,
            "accent_color": null,
            "global_name": "Symboli Rudolf",
            "avatar_decoration_data": {
                "asset": "a_9532e6bc08133eb1401c654a4f1a800e",
                "sku_id": "1332505467980873728",
                "expires_at": null
            },
            "collectibles": null,
            "display_name_styles": null,
            "banner_color": null,
            "clan": null,
            "primary_guild": null
        },
        "pinned": false,
        "mention_everyone": false,
        "tts": false
    }
]
"#;

    #[test]
    fn parses_messages_array() {
        let messages = match parse_messages(SAMPLE_MESSAGES) {
            Ok(messages) => messages,
            Err(error) => panic!("sample payload should parse: {error}"),
        };

        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].content, "we play if you guys are up for it");
        assert_eq!(messages[0].id.as_u64(), 1_480_908_521_355_874_518);
        assert_eq!(messages[0].author.username, "kinugasakaede");
    }
}
