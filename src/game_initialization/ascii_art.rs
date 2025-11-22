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
}
