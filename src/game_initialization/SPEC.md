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
- [U][X] REQ-INIT-CLI-001: System provides 'game-engine init' CLI command
- [U][X] REQ-INIT-CLI-002: System displays success message after initialization
- [U][X] REQ-INIT-CLI-003: System displays error if game.md already exists
- [U][X] REQ-INIT-CLI-004: System provides --force flag to overwrite existing game.md

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
- [O][O] REQ-INIT-GEN-005: System generates ASCII art 120 characters wide
- [O][O] REQ-INIT-GEN-006: System generates ASCII art 40 lines tall
- [O][O] REQ-INIT-GEN-007: System generates ASCII art with bordered format
- [O][O] REQ-INIT-GEN-008: System generates narrative text
- [O][O] REQ-INIT-GEN-009: System generates minimum 2 choice options
- [O][O] REQ-INIT-GEN-010: System generates maximum 4 choice options
- [O][O] REQ-INIT-GEN-011: System handles Ollama connection errors
- [O][O] REQ-INIT-GEN-012: System displays clear message for connection errors
- [O][O] REQ-INIT-GEN-013: System handles LLM generation errors

### Game File Creation
- [O][O] REQ-INIT-FILE-001: System creates game/game.md file
- [O][O] REQ-INIT-FILE-002: System writes UTF-8 encoded content
- [O][O] REQ-INIT-FILE-003: System writes game session header
- [O][O] REQ-INIT-FILE-004: System writes session start timestamp
- [O][O] REQ-INIT-FILE-005: System writes horizontal rule separator
- [O][O] REQ-INIT-FILE-006: System writes scene header
- [O][O] REQ-INIT-FILE-007: System writes ASCII art
- [O][O] REQ-INIT-FILE-008: System formats ASCII art in code fence
- [O][O] REQ-INIT-FILE-009: System writes blank line after ASCII art
- [O][O] REQ-INIT-FILE-010: System writes scene title
- [O][O] REQ-INIT-FILE-011: System formats scene title in bold
- [O][O] REQ-INIT-FILE-012: System writes narrative text
- [O][O] REQ-INIT-FILE-013: System writes blank line before choices
- [O][O] REQ-INIT-FILE-014: System writes choice prompt
- [O][O] REQ-INIT-FILE-015: System writes each choice as checkbox item
- [O][O] REQ-INIT-FILE-016: System writes final horizontal rule separator
- [O][O] REQ-INIT-FILE-017: System ensures file integrity on error
