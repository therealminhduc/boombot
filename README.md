<p align="center">
  <img src="assets/boom-bot.png" alt="Boombot Logo" width="200"/>
</p>

# Boombot

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)

**Boombot** is a Discord bot that cleans URLs by removing tracking parameters. Built in Rust with a modular architecture.

> _"Drop your messy links, Boombot will clean them with a boom !"_

## 🚀 Features

- Removes tracking parameters from URLs (e.g., `utm_*`, `igsh`, etc.).
- Available as a Discord bot with `/clean` command.
- Command-line interface for terminal usage.
- Modular design with reusable components.
- Support for multiple platforms (Google, Instagram, etc.).

## 📦 Installation

### Prerequisites

- Rust 1.70 or higher.
- Discord Bot Token.
- Discord Application ID.

### Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/boombot.git
cd boombot

# Build the project
cargo build --release
```

### Discord Bot Setup

1. Create a new Discord application at [Discord Developer Portal](https://discord.com/developers/applications).
2. Create a bot and get your token.
3. Copy `.env.example` to `.env` and fill in your credentials:
```env
DISCORD_TOKEN=your_bot_token
DISCORD_APPLICATION_ID=your_application_id
```

## 🎮 Usage

### Boombot on Discord

1. Invite Boombot to your server.
2. Use the `/clean` command with a URL:
```
/clean url:https://example.com?utm_source=twitter&utm_medium=social
```

### Command Line

```bash
# Run the CLI tool
cargo run --bin cli

# Or use the binary directly
./target/release/cli
```

## 🏗️ Project Structure

The project consists of three main components:

### 1. `cleaner` (Core Library)
- URL cleaning functionality.
- Trait-based interface (`UrlCleaner`).
- Platform-specific cleaning strategies.
- Reusable across different applications.

### 2. `cli` (Command Line Interface)
- Terminal-based URL cleaning.
- Interactive command-line tool.
- Example of cleaner library integration.

### 3. `discord` (Boombot)
- Discord integration.
- Slash command handling.
- Real-time URL cleaning in Discord servers.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository.
2. Create your feature branch (`git checkout -b feature/amazing-feature`).
3. Commit your changes (`git commit -m 'Add amazing feature'`).
4. Push to the branch (`git push origin feature/amazing-feature`).
5. Open a Pull Request.

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Discord API](https://discord.com/developers/docs/intro)
- [Rust](https://www.rust-lang.org)
- [tokio](https://tokio.rs)
- [serde](https://serde.rs)

