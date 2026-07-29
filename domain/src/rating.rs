/// Represents a numerical value used for food rating.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Rating(u8);

const MIN_RATING: u8 = 1;

const MAX_RATING: u8 = 5;

impl Rating {
    /// Creates a new `Rating`.
    ///
    /// # Examples
    /// Creating a Rating for a coffee.
    ///
    /// ```
    /// use domain::rating::{Rating};
    ///
    /// let coffee_rating = Rating::try_new(3).unwrap(); // A 3 star rating
    /// assert_eq!(coffee_rating.value(), 3);
    ///
    /// ```
    pub fn try_new(value: u8) -> Result<Self, String> {
        if !(MIN_RATING..=MAX_RATING).contains(&value) {
            return Err(format!(
                "Value must be between {MIN_RATING} and {MAX_RATING}"
            ));
        }

        Ok(Self(value))
    }

    /// Returns the value of the `Rating` object.
    pub fn value(&self) -> u8 {
        self.0
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_create_rating_of_min_rating() {
        // Arrange
        let min_rating = MIN_RATING;

        // Act
        let result = Rating::try_new(min_rating);

        // Assert
        assert!(result.is_ok());
        let rating = result.unwrap();
        assert_eq!(rating.value(), MIN_RATING);
    }

    #[test]
    fn should_create_rating_of_max_rating() {
        // Arrange
        let max_rating = MAX_RATING;

        // Act
        let result = Rating::try_new(max_rating);

        // Assert
        assert!(result.is_ok());
        let rating = result.unwrap();
        assert_eq!(rating.value(), MAX_RATING);
    }

    #[test]
    fn should_fail_when_rating_is_greater_than_max_rating() {
        // Arrange
        let rating = MAX_RATING + 1;

        // Act
        let result = Rating::try_new(rating);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn should_fail_when_rating_is_less_than_min_rating() {
        // Arrange
        let rating = MIN_RATING - 1;

        // Act
        let result = Rating::try_new(rating);

        // Assert
        assert!(result.is_err());
    }
}
