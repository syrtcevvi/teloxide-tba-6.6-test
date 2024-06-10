# Teloxide TBA6.6 update

[Link to TBA6.6](https://core.telegram.org/bots/api-changelog#march-9-2023)

This repository is intended for testing the correctness of new methods from Telegram Bot API (TBA) 6.6. and changed ones.

To try this version out do the following:
- Copy `.env.example` to `.env`
- Fill environment variables with appropriate values
- Run `cargo run --release --bin tba6_6 -F .. -F ..` in the root of this repo, where `-F` are used to select features.

Currently available features:
- `all` - Enable all features
- `description` - test description-related methods
- `send_sticker` - test `send_sticker` method with the associated emoji
- `sticker_set` - test sticker set-related functionality
- `only_edit_sticker_set` - test only sticker set editing methods

Do not reran this program too quickly, there is the `Throttle` adapter used, though this can lead to strange errors (for instance, sticker set would be not found, or different API errors will emerge: Error: Api(InvalidStickersSet)). I think, it's caused by multiple recreation of the sticker set with the same name (delete and create immediately)

> Do not remove the created sticker set manually!

> When the page with sticker sets is opened, it can break the program flow (error in syncing the telegram server state with your app)

To check how some methods work (now, only `send_sticker`) in the previous tba6_5 version use the command:
```
cargo run --release --bin tba6_5
```

So, this version brings the following methods:
- [setMyDescription](https://core.telegram.org/bots/api#setmydescription)
- [getMyDescription](https://core.telegram.org/bots/api#getmydescription)
- [setMyShortDescription](https://core.telegram.org/bots/api#setmyshortdescription)
- [getMyShortDescription](https://core.telegram.org/bots/api#getmyshortdescription)

- [deleteStickerSet](https://core.telegram.org/bots/api#deletestickerset)
- [setStickerSetTitle](https://core.telegram.org/bots/api#setstickersettitle)
- [setStickerEmojiList](https://core.telegram.org/bots/api#setstickeremojilist)
- [setStickerKeywords](https://core.telegram.org/bots/api#setstickerkeywords)

I have no idea, how to properly test the following methods:
- [setCustomEmojiStickerSetThumbnail](https://core.telegram.org/bots/api#setcustomemojistickersetthumbnail)
- [setStickerMaskPosition](https://core.telegram.org/bots/api#setstickermaskposition)

And changes the following methods: 
- [sendSticker](https://core.telegram.org/bots/api#sendsticker)
- [uploadStickerFile](https://core.telegram.org/bots/api#uploadstickerfile)
- [createNewStickerSet](https://core.telegram.org/bots/api#createnewstickerset)