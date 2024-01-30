## Telegram Bot with File-Hosting Function

This is a simple Telegram bot that allows you to upload and host files on a remote server. The bot is built using the Telegram Bot API and Rust programming language.
### Features

 * File hosting: Upload files to a remote server and get a link to share with others.
 * Security: Files can be encrypted with a user-provided key before being uploaded.
 * Easy setup: Simply clone the repository, configure the bot token and server details, and run the bot.

### Getting Started

To get started with the Telegram bot, you will need to:

1. Clone the repository to your local machine.
2. Set up a Telegram bot and obtain a bot token.
3. Configure the bot token and server details in the `.env` file.
4. Run the bot using the `cargo run` command.

Once the bot is running, you can interact with it using the Telegram app. Send a file to the bot and it will upload the file to the remote server and provide you with a link to share with others.
### Security

The bot can use encryption when upload files to ensure that files are secure during transmission and storage. The encryption key is provided by the user and is not stored on the server.
### License

This project is licensed under the Apache License. See the LICENSE file for more information.
### Acknowledgments

This project was inspired by file-hosting services. Special thanks to the Telegram Bot API documentation, Teloxide and the Rust programming language community.

### See also File-hosting site
https://github.com/1101-1/TRustyBox
