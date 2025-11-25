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
        let lines = vec!["x".repeat(120); 10];

        // Act
        let result = AsciiArt::validate_height(&lines);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_should_reject_first_line_without_plus_border() {
        // REQ-INIT-ART-003

        // Arrange
        let first_line = "-".repeat(120);

        // Act
        let result = AsciiArt::validate_first_line(&first_line);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_should_reject_first_line_without_dash_fill() {
        // REQ-INIT-ART-004

        // Arrange
        let first_line = "+".to_string() + &"x".repeat(118) + "+";

        // Act
        let result = AsciiArt::validate_first_line(&first_line);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_should_reject_middle_line_without_pipe_border() {
        // REQ-INIT-ART-005

        // Arrange
        let middle_line = " ".repeat(120);

        // Act
        let result = AsciiArt::validate_middle_line(&middle_line);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_should_reject_last_line_without_matching_first_line_format() {
        // REQ-INIT-ART-006

        // Arrange
        let last_line = "|".to_string() + &"-".repeat(118) + "|";

        // Act
        let result = AsciiArt::validate_last_line(&last_line);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_should_reject_invalid_ascii_art_format() {
        // REQ-INIT-ART-007

        // Arrange
        let invalid_art = vec!["short line".to_string()];

        // Act
        let result = AsciiArt::validate(&invalid_art);

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
    pub fn validate(lines: &[String]) -> Result<(), String> {
        Self::validate_height(lines)?;
        Self::validate_width(lines)?;
        Self::validate_first_line(&lines[0])?;

        for line in &lines[1..39] {
            Self::validate_middle_line(line)?;
        }

        Self::validate_last_line(&lines[39])?;
        Ok(())
    }

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

    pub fn validate_first_line(line: &str) -> Result<(), String> {
        if !line.starts_with('+') || !line.ends_with('+') {
            return Err("First line must start and end with '+'".to_string());
        }

        let middle = &line[1..line.len()-1];
        if !middle.chars().all(|c| c == '-') {
            return Err("First line must be filled with '-' between '+' symbols".to_string());
        }

        Ok(())
    }

    pub fn validate_middle_line(line: &str) -> Result<(), String> {
        if !line.starts_with('|') || !line.ends_with('|') {
            return Err("Middle lines must start and end with '|'".to_string());
        }
        Ok(())
    }

    pub fn validate_last_line(line: &str) -> Result<(), String> {
        Self::validate_first_line(line)
    }
}
