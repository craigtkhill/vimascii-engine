# Product Requirements Document: Game Engine

## Overview

**Game Engine** is an LLM-powered interactive story game that combines AI-generated ASCII art with narrative text using local models. Players interact through their preferred text editor (ideally this project is for Neovim/Evil-Helix practice), making choices by editing markdown files.

## Vision

Create a text-based story game that:
- Helps players improve their editor skills (especially Neovim/Evil-Helix)
- Uses locally-hosted LLMs for dynamic content generation
- Provides an engaging, replayable story experience
- Maintains a simple, file-based architecture

## Target Users

- Developers learning/practicing Neovim/Evil-Helix or other text editors
- Text adventure game/ ASCII art enthusiasts
- Users interested in AI-generated interactive fiction

## Core Features (MVP)

### 1. LLM-Powered Content Generation
- **ASCII Art Generation**: Create visual scenes using ASCII characters
- **Story Generation**: Generate narrative text and dialogue
- **Combined Output**: ASCII art appears above story text in a cohesive scene
- **Story Context**: Use markdown files containing story to provide context for the narrative
- **Ollama Integration**: Use locally-hosted models (no API keys required)

### 2. Editor-Based Gameplay
- **Single Markdown File**: `game.md` contains the entire game state (for now)
- **Choice Markers**: Players select options by editing markdown checkboxes `[x]`
- **Freeform Editing**: Players can modify ASCII art, narrative or options freely
- **Editor Agnostic**: Works with any text editor (Vim, Neovim, VSCode, Zed etc.)

### 3. File-Based Save System
- **Persistent State**: Current game state stored directly in `game.md`
- **No Database**: Filesystem is the database
- **Human Readable**: All game data is plain text
- **Version Control Friendly**: Can track game history with git

### 4. CLI Game Engine
- **Command**: `game-engine` (Rust binary)
- **Subcommands**:
  - `init` - Create a new game file
  - `next` - Process player choice and generate next scene
  - `reset` - Start over from the beginning
  **Bevy**: Use Bevy game engine if required for task.

## User Flow

### Initial Setup
1. Player installs `game-engine` CLI
2. Runs `game-engine init` to create `game.md`
3. Opens `game.md` in their preferred editor

### Gameplay Loop
1. **Read**: Player reads current scene (ASCII art + story)
2. **Choose**: Player marks their choice using `[x]` checkbox
3. **Generate**: Player runs `game-engine next` in terminal
4. **Update**: CLI updates `game.md` with new scene
5. **Repeat**: Back to step 1

### Example Game File Structure

```markdown
# Game State

Last Updated: 2025-11-22 16:30:00

## Current Scene

```
    /\_/\
   ( o.o )
    > ^ <
```

You stand at a crossroads in the forest. A mysterious cat sits on a stone,
watching you with knowing eyes. The path splits in three directions.

## Your Choice

What do you do?

- [ ] Ask the cat for directions
- [ ] Take the left path into the dark woods
- [x] Take the right path toward the distant mountains
- [ ] Sit and rest by the stone

---
Previous scenes stored below...
```

## Technical Architecture

### Language & Tools
- **Language**: Rust
- **LLM Backend**: Ollama (local)
- **File Format**: Markdown
- **CLI Framework**: clap or similar
- **Ollama Client**: ollama-rs or reqwest
- **Bevy**: If required
- **Structured Generation** Microsoft Guideline or Outlines

### Design Principles

1. **Clean Architecture**: Separate domain logic from infrastructure
2. **Testability**: Core game logic has no external dependencies
3. **Simplicity First**: Start simple, add complexity incrementally
4. **File-Based**: Filesystem is the primary data store
5. **Genre-Agnostic**: Support any story genre/theme using bring your own story markdown files

## Success Criteria

1. Player can complete a full game loop (read → choose → generate → repeat)
2. LLM generates coherent ASCII art and story text
3. Game state persists across sessions
4. Experience feels smooth in Neovim workflow
5. Code is well-tested and maintainable

## Stakeholders

- **Developer**: Craig (primary developer)
- **Player**: Neovim learners, text adventure enthusiasts
