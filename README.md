# vimascii-engine

**LLM-powered ASCII art story game for Vim/Neovim practice**

A text-based interactive story game that combines AI-generated ASCII art with narrative text. Play the game in your favorite text editor while improving your modal editing skills.

## Features

- 🎨 **AI-Generated ASCII Art** - Scenes created by local LLMs
- 📖 **Dynamic Storytelling** - Narratives that respond to your choices and edits
- ⌨️ **Editor-First Gameplay** - Practice Vim/Neovim motions while playing
- 🤖 **Local LLM Integration** - Powered by Ollama (no API keys needed)
- 📝 **Markdown-Based** - Human-readable game files you can edit freely
- 🎮 **Bring Your Own Story** - Use custom story contexts for different adventures

## How It Works

1. **Initialize** a new game with `game-engine init`
2. **Open** `game/game.md` in your editor (Vim, Neovim, Helix, VSCode, etc.)
3. **Read** the scene with ASCII art and narrative text
4. **Choose** by marking a choice with `[x]`
5. **Generate** the next scene with `game-engine next`
6. **Navigate** to the new scene at the bottom using your editor skills (`G` in Vim)
7. **Repeat** and enjoy the story!

### Setup

```bash
# Clone the repository
git clone https://github.com/craigtkhill/vimascii-engine.git
cd vimascii-engine

# Build the project
cargo build --release

# Run the CLI
cargo run -- init
```

- Uses [ATUI Tools](https://github.com/craigtkhill/atui-tools) for development workflow
