// ============================================
// TESTS
// ============================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_reject_line_width_less_than_120() {
        // REQ-INIT-ART-001

        // Arrange
        let lines = vec!["short line".to_string()];

        // Act
        let result = AsciiArt::validate_width(&lines);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_should_reject_height_less_than_40() {
        // REQ-INIT-ART-002

        // Arrange
        let lines = vec!["x".repeat(120); 10]; // Only 10 lines

        // Act
        let result = AsciiArt::validate_height(&lines);

        // Assert
        assert!(result.is_err());
    }
}

// ============================================
// IMPLEMENTATION
// ============================================
pub struct AsciiArt {
    lines: Vec<String>,
}

impl AsciiArt {
    pub fn validate_width(lines: &[String]) -> Result<(), String> {
        for line in lines {
            if line.len() != 120 {
                return Err(format!("Line width must be exactly 120 characters, got {}", line.len()));
            }
        }
        Ok(())
    }

    pub fn validate_height(lines: &[String]) -> Result<(), String> {
        if lines.len() != 40 {
            return Err(format!("Height must be exactly 40 lines, got {}", lines.len()));
        }
        Ok(())
    }
}
