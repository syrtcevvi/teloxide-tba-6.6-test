use std::{env::var, error::Error};

use dotenvy::dotenv;

use teloxide_v6_5::{
    adaptors::{throttle::Limits, Throttle},
    prelude::*,
    types::InputFile,
};

type Bot = Throttle<teloxide_v6_5::Bot>;
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
    let bot = teloxide_v6_5::Bot::from_env().throttle(Limits::default());
    log::info!("Bot initialized");

    // Use breakpoints and lldb if you want to check the intermediate results

    bot.send_sticker(user_tg_id, InputFile::file("data/cat_and_cup.webp"))
        .await?;

    Ok(())
}
