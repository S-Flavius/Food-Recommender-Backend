#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Rating(u8);

impl Rating {
    pub fn try_new(value: u8) -> Result<Self, &'static str> {
        if !(1..=5).contains(&value) {
            return Err("Value must be between 1 and 5");
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}

#[cfg(test)]
mod tests {

    // Import everything from parent module
    use super::*;

    #[test]
    fn should_create_rating_of_1() {
        // Arrange
        let valid_rating = 1;

        // Act
        let result = Rating::try_new(valid_rating);

        // Assert
        assert!(result.is_ok());
        let rating = result.unwrap();
        assert_eq!(rating.value(), 1);
    }

    #[test]
    fn should_create_rating_of_5() {
        // Arrange
        let valid_rating = 5;

        // Act
        let result = Rating::try_new(valid_rating);

        // Assert
        assert!(result.is_ok());
        let rating = result.unwrap();
        assert_eq!(rating.value(), 5);
    }

    #[test]
    fn should_fail_when_rating_is_greater_than_5() {
        // Arrange
        let rating = 6;

        // Act
        let result = Rating::try_new(rating);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn should_fail_when_rating_is_less_than_1() {
        // Arrange
        let rating = 0;

        // Act
        let result = Rating::try_new(rating);

        // Assert
        assert!(result.is_err());
    }
}