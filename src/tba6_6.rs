use std::error::Error;

use dotenvy::dotenv;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use teloxide_v6_6::{
    adaptors::{throttle::Limits, Throttle},
    prelude::*,
    types::{InputFile, InputSticker, StickerFormat},
};

type Bot = Throttle<teloxide_v6_6::Bot>;
type Result = std::result::Result<(), Box<dyn Error>>;

#[derive(Deserialize, Serialize)]
struct EnvConfig {
    pub user_id: UserId,
}

lazy_static! {
    static ref ENV_CONFIG: EnvConfig =
        envy::from_env::<EnvConfig>().expect("Unable to build ENV_CONFIG");
    static ref BOT: Bot = teloxide_v6_6::Bot::from_env().throttle(Limits::default());
}

#[tokio::main]
async fn main() -> Result {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("The program started");

    /*
        Use breakpoints and lldb if you want to check the intermediate results
        Different parts of tba6.6 are put under the conditional compilation flags (use -F to set them: -F description or -F all)
    */

    // set_my_description/set_my_short_description & get_my_description/get_my_short_description
    if cfg!(feature = "description") {
        set_descriptions().await?;
    }

    // send_sticker with the associated emoji
    if cfg!(feature = "send_sticker") {
        send_sticker_with_associated_emoji().await?;
    }

    // upload_sticker_file & create_new_sticker_set & delete_sticker_set
    if cfg!(feature = "sticker_set") {
        create_new_sticker_set().await?;
    }

    Ok(())
}

async fn set_descriptions() -> Result {
    // Without language_code
    log::info!(
        r#"set_my_description/set_my_short_description & get_my_description/get_my_short_description without "language_code""#
    );
    let bot_description = "A snail walks into a bar one day..";
    BOT.set_my_description()
        .description(bot_description)
        .await?;
    assert_eq!(BOT.get_my_description().await?.description, bot_description);

    BOT.set_my_short_description()
        .short_description(bot_description)
        .await?;
    assert_eq!(
        BOT.get_my_short_description().await?.short_description,
        bot_description
    );

    // Set bot's description for "ru" localization
    log::info!(
        r#"set_my_description/set_my_short_description & get_my_description/get_my_short_description with "language_code"="ru""#
    );
    let ru_bot_description = "Заходит как-то улитка в бар..";
    BOT.set_my_description()
        .description(ru_bot_description)
        .language_code("ru")
        .await?;
    assert_eq!(
        BOT.get_my_description()
            .language_code("ru")
            .await?
            .description,
        ru_bot_description
    );
    BOT.set_my_short_description()
        .short_description(ru_bot_description)
        .language_code("ru")
        .await?;
    assert_eq!(
        BOT.get_my_short_description()
            .language_code("ru")
            .await?
            .short_description,
        ru_bot_description
    );
    Ok(())
}

async fn send_sticker_with_associated_emoji() -> Result {
    log::info!("send_sticker with the associated emoji");

    // Each just uploaded sticker can have an associated emoji with it
    let associated_emoji = "🦀";

    BOT.send_sticker(
        ENV_CONFIG.user_id,
        InputFile::file("data/teloxide-logo.webp"),
    )
    .emoji(associated_emoji)
    .await?;

    Ok(())
}

async fn create_new_sticker_set() -> Result {
    log::info!("create_new_sticker_set & delete_sticker_set");

    let bot_info = BOT.get_me().await?.user;
    let sticker_set_name = format!("tba66_by_{}", bot_info.username.unwrap());

    // Sticker set can be created already, so we have to delete it
    // Can fail for the first time (before first sticker set creation)
    let _ = BOT.delete_sticker_set(sticker_set_name.clone()).await?;

    log::info!("upload_sticker_file");
    // Upload sticker files (TODO cache)
    let (fst, snd) = (
        BOT.upload_sticker_file(
            ENV_CONFIG.user_id,
            InputFile::file("data/teloxide-core-logo.webp"),
            StickerFormat::Static,
        )
        .await?,
        BOT.upload_sticker_file(
            ENV_CONFIG.user_id,
            InputFile::file("data/teloxide-logo.webp"),
            StickerFormat::Static,
        )
        .await?,
    );
    // Alert: Don't spam this method too quickly, there can be some anomalies (sticker set could be found or similar, or other )
    // weird errors could happen: Error: Api(InvalidStickersSet), for instance
    log::info!("Creating the sticker set");
    BOT.create_new_sticker_set(
        ENV_CONFIG.user_id,
        sticker_set_name.clone(),
        "Teloxide TBA6.6 TEST",
        [
            InputSticker {
                sticker: InputFile::file_id(fst.id),
                emoji_list: vec!["🦀".to_owned(), "😄".to_owned()],
                mask_position: None,
                keywords: Some(vec![
                    "Teloxide".to_owned(),
                    "TBA6.6".to_owned(),
                    "TEST".to_owned(),
                    "core".to_owned(),
                ]),
            },
            InputSticker {
                sticker: InputFile::file_id(snd.id),
                emoji_list: vec!["🦀".to_owned(), "🥳".to_owned()],
                mask_position: None,
                keywords: Some(vec![
                    "Teloxide".to_owned(),
                    "TBA6.6".to_owned(),
                    "TEST".to_owned(),
                    "main".to_owned(),
                ]),
            },
        ],
        StickerFormat::Static,
    )
    .await?;
    /*
       Created sticker set will be available at the following link:
       https://t.me/addstickers/tba66_by_<the name of your bot>
    */
    log::info!(
        "Your sticker set has been successfully created! Get it: https://t.me/addstickers/{}",
        sticker_set_name
    );

    Ok(())
}
