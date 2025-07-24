<p align="center">
  <img src="assets/boom-bot.png" alt="Boombot Logo" width="200"/>
</p>

# Boombot

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)

**Boombot** is a modular toolset for cleaning URLs by removing tracking parameters. It's available as a Discord bot, a command-line tool and a webapp.

> _"Drop your messy links, Boombot will clean them with a boom !"_

## ✨ What is Boombot ?

Boombot helps you keep your links clean and private by removing tracking parameters (like `utm_*`, `igsh`, etc.) from URLs.

You can use the bot in three ways:
- **Discord Bot:** Clean links directly in your Discord server.
- **Command-Line Tool:** Clean links from your terminal.
- **Web App:** Contribute new cleaning rules and explore the showcase homepage.

## 🚀 Features

- Removes tracking parameters from URLs.
- Available as a Discord bot with `/clean` command.
- Command-line interface for terminal usage.
- Modular design with reusable components.
- Support for multiple platforms (Google, Instagram, etc.).

## 📦 Installation

### Prerequisites

- Rust 1.70 or higher.
- (For Discord) Discord Bot Token & Application ID.
- (For Web) Node.js

### Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/boombot.git
cd boombot

# Build the project
cargo build --release
```

## 🎮 Usage

### 1. Start the Core Engine

Before running the Discord bot or the CLI tool, you need to start the core engine:

```bash
cargo run --bin cleaner
```

### 2. Discord Bot

1. Create a Discord application and bot at [Discord Developer Portal](https://discord.com/developers/applications).
2. Copy `.env.example` to `.env` and fill in your credentials:

```env
DISCORD_TOKEN=your_bot_token
DISCORD_APPLICATION_ID=your_application_id
```
3. Run the bot:
```bash
cargo run --bin discord
```
4. Invite Boombot to your server and use `/clean url:<your-url>`.


### 3. Command-Line Tool

```bash
cargo run --bin cli
# Or use the binary directly after building:
./target/release/cli
```

### 4. Webapp

The webapp allows members to contribute new URL cleaning rules and serves as a showcase site with a homepage.

#### Running the Web App (Development)

```bash
cd web
npm install
npm run dev
```
- Open [http://localhost:5173](http://localhost:5173) in your browser.

#### Building for Production

```bash
cd web
npm run build
```
- The static files will be in `web/dist`.


### 5. Running Backend and Core Engine

You can run both the backend and the core engine together using a single command from the root of the repository:

```bash
cargo run
```

## 🏗️ Project Structure

- **cleaner/**: Core library for URL cleaning logic (Rust).
- **cli/**: Command-line interface for cleaning URLs.
- **discord/**: Discord bot integration.
- **web/**: Web frontend for contributing rules and showcasing features.
- **back/**: Backend API for the web app and admin panel.


## 🤝 Contributing

Contributions are welcome! Feel free to submit a Pull Request.

1. Fork the repository.
2. Create your feature branch (`git checkout -b feature/amazing-feature`).
3. Commit your changes (`git commit -m 'Add amazing feature'`).
4. Push to the branch (`git push origin feature/amazing-feature`).
5. Open a Pull Request.

## 📝 License

MIT License. See [LICENSE](LICENSE) for details.

## 🙏 Acknowledgments

- [Discord API](https://discord.com/developers/docs/intro)
- [Rust](https://www.rust-lang.org)
- [tokio](https://tokio.rs)
- [serde](https://serde.rs)

