use std::error::Error;

use dotenvy::dotenv;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use teloxide_v6_6::{
    adaptors::{throttle::Limits, Throttle},
    payloads::SetStickerKeywordsSetters,
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
    if cfg!(any(
        feature = "sticker_set",
        feature = "only_edit_sticker_set"
    )) {
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

    if cfg!(not(feature = "only_edit_sticker_set")) {
        // Sticker set can be created already, so we have to delete it
        // Can fail for the first time (before first sticker set creation)
        match BOT.delete_sticker_set(sticker_set_name.clone()).await {
            Ok(_) => log::info!(r#"sticker set "{sticker_set_name}" has been deleted"#),
            Err(err) => log::error!("error when deleting the sticker set: {err}"),
        }

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
                    keywords: vec!["teloxide".to_owned(), "f".to_owned(), "core".to_owned()],
                },
                InputSticker {
                    sticker: InputFile::file_id(snd.id),
                    emoji_list: vec!["🦀".to_owned(), "🥳".to_owned()],
                    mask_position: None,
                    keywords: vec![
                        "teloxide".to_owned(),
                        "second".to_owned(),
                        "main".to_owned(),
                    ],
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
    }

    // Changing the sticker in the sticker set. You should place a breakpoint here

    {
        log::info!("get_sticker_set");
        let sticker_set = BOT.get_sticker_set(sticker_set_name.clone()).await?;

        log::info!("set_sticker_set_title");
        let new_sticker_set_title = "New sticker set title";
        BOT.set_sticker_set_title(sticker_set_name.clone(), new_sticker_set_title)
            .await?;

        let changing_sticker = &sticker_set.stickers[0];
        log::info!("set_sticker_emoji_list");
        let new_emoji_list = vec!["🤔".to_owned()];
        BOT.set_sticker_emoji_list(changing_sticker.file.id.clone(), new_emoji_list)
            .await?;

        log::info!("set_sticker_keywords");
        let new_sticker_keywords = vec!["core".to_owned()];
        BOT.set_sticker_keywords(changing_sticker.file.id.clone())
            .keywords(new_sticker_keywords.clone())
            .await?;

        log::info!("verifying the sticker changes");

        let sticker_set = BOT.get_sticker_set(sticker_set_name.clone()).await?;
        assert_eq!(sticker_set.title, new_sticker_set_title);

        /*
           There is no way to check automatically what keywords and emojis are attached to the
           sticker. The `Sticker` type contains only the one optional emoji

           Frankly speaking, I have no idea how the stickers' keywords can be used
           when you try to search for sticker the input text is compared to the name
           of the sticker set and to the name of associated emojis. Maybe, currently there is
           no way to use them in the telegram app, only with the help of API
        */
        log::info!(
            r#"you can check the changes of the first sticker in the "{}" sticker set"#,
            sticker_set_name
        );

        log::info!("add_sticker_to_set");
        let new_sticker = BOT
            .upload_sticker_file(
                ENV_CONFIG.user_id,
                InputFile::file("data/teloxide-logo-blur.webp"),
                StickerFormat::Static,
            )
            .await?;

        BOT.add_sticker_to_set(
            ENV_CONFIG.user_id,
            sticker_set_name,
            InputSticker {
                sticker: InputFile::file_id(new_sticker.id),
                emoji_list: vec!["😄".to_owned()],
                keywords: vec![],
                mask_position: None,
            },
        )
        .await?;

        log::info!("new sticker has been successfully added to your sticker set!");
    }

    Ok(())
}
