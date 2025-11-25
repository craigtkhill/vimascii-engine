// ============================================
// TESTS
// ============================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_enforce_ascii_art_width_in_schema() {
        // REQ-INIT-SCHEMA-001

        // Arrange
        let schema = SceneSchema::new();

        // Act
        let width = schema.ascii_art_width();

        // Assert
        assert_eq!(width, 120);
    }

    #[test]
    fn test_should_enforce_ascii_art_height_in_schema() {
        // REQ-INIT-SCHEMA-001

        // Arrange
        let schema = SceneSchema::new();

        // Act
        let height = schema.ascii_art_height();

        // Assert
        assert_eq!(height, 40);
    }

    #[test]
    fn test_should_enforce_minimum_choice_count_in_schema() {
        // REQ-INIT-SCHEMA-002

        // Arrange
        let schema = SceneSchema::new();

        // Act
        let min = schema.min_choices();

        // Assert
        assert_eq!(min, 2);
    }

    #[test]
    fn test_should_enforce_maximum_choice_count_in_schema() {
        // REQ-INIT-SCHEMA-002

        // Arrange
        let schema = SceneSchema::new();

        // Act
        let max = schema.max_choices();

        // Assert
        assert_eq!(max, 4);
    }
}

// ============================================
// IMPLEMENTATION
// ============================================
pub struct SceneSchema {
    ascii_art_width: usize,
    ascii_art_height: usize,
    min_choices: usize,
    max_choices: usize,
}

impl SceneSchema {
    pub fn new() -> Self {
        Self {
            ascii_art_width: 120,
            ascii_art_height: 40,
            min_choices: 2,
            max_choices: 4,
        }
    }

    pub fn ascii_art_width(&self) -> usize {
        self.ascii_art_width
    }

    pub fn ascii_art_height(&self) -> usize {
        self.ascii_art_height
    }

    pub fn min_choices(&self) -> usize {
        self.min_choices
    }

    pub fn max_choices(&self) -> usize {
        self.max_choices
    }
}
