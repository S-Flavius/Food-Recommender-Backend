use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Restaurant {
    id: Uuid,
    name: String,
    location: String,
}

impl Restaurant {
    pub fn new(id: Uuid, name: String, location: String) -> Restaurant {
        Self { id, name, location }
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
    use uuid::Uuid;
    use super::*;

    #[test]
    fn should_create_restaurant() {
        // Arrange
        let id = Uuid::new_v4();
        let name = String::from("KFC");
        let location = String::from("Bucharest, Romania");

        // Act
        let restaurant = Restaurant::new(id, name.clone(), location.clone());

        // Assert
        assert_eq!(restaurant.id(), id);
        assert_eq!(restaurant.name(), name);
        assert_eq!(restaurant.location(), location);
    }
}