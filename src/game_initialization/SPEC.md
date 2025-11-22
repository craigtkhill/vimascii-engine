# Game Initialization Specification

## Feature: Initialize New Game Session
As a player
I want to run 'game-engine init' command
So that I can start a new interactive story game in my text editor

## Requirements
Format: `[IS-TEST-IMPLEMENTED][IS-CODE-IMPLEMENTED] IDENTIFIER: example case`
- U = implemented via unit test
- A = implemented via acceptance test
- X = implemented
- O = not yet implemented

### CLI Command
- [O][O] REQ-INIT-CLI-001: System provides 'game-engine init' CLI command
- [O][O] REQ-INIT-CLI-002: System displays success message after initialization
- [O][O] REQ-INIT-CLI-003: System displays error if game.md already exists
- [O][O] REQ-INIT-CLI-004: System provides --force flag to overwrite existing game.md

### Directory Structure
- [O][O] REQ-INIT-DIR-001: System creates game/ directory if it does not exist
- [O][O] REQ-INIT-DIR-002: System verifies game/story/ directory exists
- [O][O] REQ-INIT-DIR-003: System displays error if game/story/story.md does not exist

### Story Context Loading
- [O][O] REQ-INIT-STORY-001: System reads game/story/story.md file
- [O][O] REQ-INIT-STORY-002: System parses story.md using UTF-8 encoding
- [O][O] REQ-INIT-STORY-003: System extracts full story context as text
- [O][O] REQ-INIT-STORY-004: System handles file read errors gracefully

### First Scene Generation
- [O][O] REQ-INIT-GEN-001: System connects to Ollama API
- [O][O] REQ-INIT-GEN-002: System sends story context to LLM
- [O][O] REQ-INIT-GEN-003: System requests first scene generation
- [O][O] REQ-INIT-GEN-004: System uses structured generation for consistent format
- [O][O] REQ-INIT-GEN-005: System generates ASCII art (120x40, bordered with +--|)
- [O][O] REQ-INIT-GEN-006: System generates narrative text (1 paragraph, ~50 words)
- [O][O] REQ-INIT-GEN-007: System generates 2-4 choice options
- [O][O] REQ-INIT-GEN-008: System handles Ollama connection errors with clear message
- [O][O] REQ-INIT-GEN-009: System handles LLM generation errors gracefully

### Game File Creation
- [O][O] REQ-INIT-FILE-001: System creates game/game.md file
- [O][O] REQ-INIT-FILE-002: System writes UTF-8 encoded content
- [O][O] REQ-INIT-FILE-003: System writes "# Game Session" header
- [O][O] REQ-INIT-FILE-004: System writes session start timestamp
- [O][O] REQ-INIT-FILE-005: System writes horizontal rule separator (---)
- [O][O] REQ-INIT-FILE-006: System writes "## Scene 1" header
- [O][O] REQ-INIT-FILE-007: System writes ASCII art in code fence (```)
- [O][O] REQ-INIT-FILE-008: System writes blank line after code fence
- [O][O] REQ-INIT-FILE-009: System writes scene title in bold (**title**)
- [O][O] REQ-INIT-FILE-010: System writes narrative text
- [O][O] REQ-INIT-FILE-011: System writes blank line before choices
- [O][O] REQ-INIT-FILE-012: System writes "**What do you do?**" prompt
- [O][O] REQ-INIT-FILE-013: System writes each choice as "- [ ] {text}"
- [O][O] REQ-INIT-FILE-014: System writes final horizontal rule separator (---)
- [O][O] REQ-INIT-FILE-015: System ensures file integrity (no partial writes on error)

### ASCII Art Format Validation
- [O][O] REQ-INIT-ART-001: Generated ASCII art is exactly 120 characters wide per line
- [O][O] REQ-INIT-ART-002: Generated ASCII art is exactly 40 lines tall
- [O][O] REQ-INIT-ART-003: First line starts with + and ends with +
- [O][O] REQ-INIT-ART-004: First line is filled with - characters between + symbols
- [O][O] REQ-INIT-ART-005: Lines 2-39 start with | and end with |
- [O][O] REQ-INIT-ART-006: Last line matches first line format
- [O][O] REQ-INIT-ART-007: System validates format before writing to file

### Structured Generation Schema
- [O][O] REQ-INIT-SCHEMA-001: System enforces ASCII art dimensions via schema
- [O][O] REQ-INIT-SCHEMA-002: System enforces choice count (2-4) via schema
- [O][O] REQ-INIT-SCHEMA-003: System enforces narrative length constraints via schema
- [O][O] REQ-INIT-SCHEMA-004: System validates generated content matches schema
