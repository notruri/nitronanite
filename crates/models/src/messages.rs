use serde::{Deserialize, Serialize};
use serde_json::Value;
use snownite::Snowflake;

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

    const SAMPLE_MESSAGES: &str = r##"
    [
        {
            "type": 0,
            "content": "a",
            "mentions": [],
            "mention_roles": [],
            "attachments": [],
            "embeds": [],
            "timestamp": "2026-03-10T15:08:29.013000+00:00",
            "edited_timestamp": null,
            "flags": 0,
            "components": [],
            "id": "1480945446213124259",
            "channel_id": "1447451123546849424",
            "author": {
                "id": "934336760157192242",
                "username": "ruri.exe",
                "avatar": "20fc807b718249053adda91543b4ebb1",
                "discriminator": "0",
                "public_flags": 256,
                "flags": 256,
                "banner": null,
                "accent_color": 13678281,
                "global_name": "Ruri",
                "avatar_decoration_data": {
                    "asset": "a_671c4fcfb8d06e05fec00b061c720f7d",
                    "sku_id": "1432550258184818808",
                    "expires_at": null
                },
                "collectibles": {
                    "nameplate": {
                        "sku_id": "1458472704524156959",
                        "asset": "nameplates/slumber_party/moon_bloom/",
                        "label": "COLLECTIBLES_SLUMBER_PARTY_MOON_BLOOM_NP_A11Y",
                        "palette": "berry"
                    }
                },
                "display_name_styles": null,
                "banner_color": "#d0b6c9",
                "clan": {
                    "identity_guild_id": null,
                    "identity_enabled": false,
                    "tag": null,
                    "badge": null
                },
                "primary_guild": {
                    "identity_guild_id": null,
                    "identity_enabled": false,
                    "tag": null,
                    "badge": null
                }
            },
            "pinned": false,
            "mention_everyone": false,
            "tts": false,
            "nonce": "1480945448146436096"
        }
    ]
    "##;

    #[test]
    fn parses_messages_array() {
        let messages = match parse_messages(SAMPLE_MESSAGES) {
            Ok(messages) => messages,
            Err(error) => panic!("sample payload should parse: {error}"),
        };

        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].content, "a");
        assert_eq!(messages[0].id.as_u64(), 1480945446213124259);
        assert_eq!(messages[0].author.username, "ruri.exe");
    }
}
