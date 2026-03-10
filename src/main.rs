use nitronanite_http::*;
use snownite::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = std::env::var("DISCORD")?;
    let client = Http::builder().token(token).build()?;
    let channel_id = Snowflake::from_raw(1451259544243015942);
    let messages = client.get_channel_messages(channel_id, Some(20)).await?;

    println!("{messages:#?}");

    Ok(())
}
