use uuid::Uuid;

/// Represents the restaurant entity.
///
/// A restaurant has a `Uuid` `id`, a `name`, and a `location`.
/// It is a data container and does not contain any deep logic.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Restaurant {
    id: Uuid,
    name: String,
    location: String,
}

impl Restaurant {
    /// Creates a new `Restaurant`.
    ///
    /// # Examples
    ///
    /// ```
    /// use uuid::Uuid;
    /// use domain::restaurant::{Restaurant};
    ///
    /// let name = String::from("SushiPlace");
    /// let location = String::from("Japan");
    /// let id = Uuid::new_v4();
    ///
    /// let restaurant = Restaurant::try_new(id, name, location);
    ///
    /// assert!(restaurant.is_ok());
    /// ```
pub fn try_new(id: Uuid, name: String, location: String) -> Result<Self, &'static str> {
    if name.trim().is_empty() {
        return Err("Name cannot be empty");
    }
    if location.trim().is_empty() {
        return Err("Location cannot be empty");
    }

    Ok(Self { id, name, location })
}

        Ok(Self { id, name, location })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn location(&self) -> &str {
        &self.location
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn should_create_restaurant() {
        // Arrange
        let id = Uuid::new_v4();
        let name = String::from("KFC");
        let location = String::from("Bucharest, Romania");

        // Act
        let restaurant = Restaurant::try_new(id, name.clone(), location.clone());

        // Assert
        assert!(restaurant.is_ok());
        let restaurant = restaurant.unwrap();
        assert_eq!(restaurant.id(), id);
        assert_eq!(restaurant.name(), name);
        assert_eq!(restaurant.location(), location);
    }

    #[test]
    fn should_fail_on_empty_name() {
        // Arrange
        let id = Uuid::new_v4();
        let name = String::from("");
        let location = String::from("France");

        // Act
        let restaurant = Restaurant::try_new(id, name.clone(), location.clone());

        // Assert
        assert!(restaurant.is_err());
    }

    #[test]
    fn should_fail_on_empty_location() {
        // Arrange
        let id = Uuid::new_v4();
        let name = String::from("McDonald's");
        let location = String::from(" ");

        // Act
        let restaurant = Restaurant::try_new(id, name.clone(), location.clone());

        // Assert
        assert!(restaurant.is_err());
    }
}
