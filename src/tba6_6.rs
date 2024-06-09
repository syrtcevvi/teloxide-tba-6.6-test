use std::{env::var, error::Error};

use dotenvy::dotenv;

use teloxide_v6_6::{
    adaptors::{throttle::Limits, Throttle},
    prelude::*,
    types::InputFile,
};

type Bot = Throttle<teloxide_v6_6::Bot>;
type Result = std::result::Result<(), Box<dyn Error>>;

#[tokio::main]
async fn main() -> Result {
    dotenv().ok();
    pretty_env_logger::init();

    let user_tg_id = UserId(
        var("USER_TG_ID")
            .expect("USER_TG_ID must be set")
            .parse::<u64>()
            .expect("USER_TG_ID must be the user_id identificator number"),
    );

    log::info!("The program started");
    let bot = teloxide_v6_6::Bot::from_env().throttle(Limits::default());
    log::info!("Bot initialized");

    // Use breakpoints and lldb if you want to check the intermediate results

    // SetMyDescription/SetMyShortDescription & GetMyDescription/GetMyShortDescription
    test_set_descriptions(&bot).await?;

    log::info!("SendSticker with the associated emoji");

    // Each just uploaded sticker can have an associated emoji with it
    let associated_emoji = "🦀";

    bot.send_sticker(user_tg_id, InputFile::file("data/teloxide-logo.png"))
        .emoji(associated_emoji)
        .await?;

    Ok(())
}

async fn test_set_descriptions(bot: &Bot) -> Result {
    // Without language_code
    log::info!(
        r#"SetMyDescription/SetMyShortDescription & GetMyDescription/GetMyShortDescription without "language_code""#
    );
    let bot_description = "A snail walks into a bar one day..";
    bot.set_my_description()
        .description(bot_description)
        .await?;
    assert_eq!(bot.get_my_description().await?.description, bot_description);

    bot.set_my_short_description()
        .short_description(bot_description)
        .await?;
    assert_eq!(
        bot.get_my_short_description().await?.short_description,
        bot_description
    );

    // Set bot's description for "ru" localization
    log::info!(
        r#"SetMyDescription/SetMyShortDescription & GetMyDescription/GetMyShortDescription with "language_code"="ru""#
    );
    let ru_bot_description = "Заходит как-то улитка в бар..";
    bot.set_my_description()
        .description(ru_bot_description)
        .language_code("ru")
        .await?;
    assert_eq!(
        bot.get_my_description()
            .language_code("ru")
            .await?
            .description,
        ru_bot_description
    );
    bot.set_my_short_description()
        .short_description(ru_bot_description)
        .language_code("ru")
        .await?;
    assert_eq!(
        bot.get_my_short_description()
            .language_code("ru")
            .await?
            .short_description,
        ru_bot_description
    );
    Ok(())
}
