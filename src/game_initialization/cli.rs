// ============================================
// TESTS
// ============================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_provide_init_command() {
        // REQ-INIT-CLI-001

        // Arrange
        let args = vec!["game-engine", "init"];

        // Act
        let result = Cli::try_parse_from(args);

        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_should_display_success_message_after_initialization() {
        // REQ-INIT-CLI-002

        // Arrange
        let message = "Game initialized successfully";

        // Act
        let result = SuccessMessage::initialization();

        // Assert
        assert_eq!(result, message);
    }

    #[test]
    fn test_should_display_error_if_game_file_already_exists() {
        // REQ-INIT-CLI-003

        // Arrange
        let expected = "Error: game.md already exists";

        // Act
        let result = ErrorMessage::game_file_exists();

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn test_should_provide_force_flag() {
        // REQ-INIT-CLI-004

        // Arrange
        let args = vec!["game-engine", "init", "--force"];

        // Act
        let result = Cli::try_parse_from(args);

        // Assert
        assert!(result.is_ok());
    }
}

// ============================================
// IMPLEMENTATION
// ============================================
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "game-engine")]
#[command(about = "LLM-powered ASCII art story game")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init {
        #[arg(long)]
        force: bool,
    },
}

pub struct SuccessMessage;

impl SuccessMessage {
    pub fn initialization() -> String {
        "Game initialized successfully".to_string()
    }
}

pub struct ErrorMessage;

impl ErrorMessage {
    pub fn game_file_exists() -> String {
        "Error: game.md already exists".to_string()
    }
}
