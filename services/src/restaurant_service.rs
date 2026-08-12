use domain::restaurant::Restaurant;
use uuid::Uuid;

/// Repository abstraction for persisting `Restaurant` entities.
///
/// Implementors provide a concrete persistence strategy (in-memory, database, etc.).
/// The service depends on this trait and does not make assumptions about storage.
///
/// # Examples
///
/// A small in-memory repository can be used in tests and examples. The example
/// below shows a `MockRepo` that captures saved restaurants so the caller can
/// assert the repository was invoked.
///
/// ```rust
/// use services::restaurant_service::{RestaurantRepository, RestaurantService};
/// use domain::restaurant::Restaurant;
/// use std::cell::RefCell;
/// use std::rc::Rc;
///
/// #[derive(Clone)]
/// struct MockRepo {
///     saved: Rc<RefCell<Vec<Restaurant>>>,
/// }
///
/// impl MockRepo {
///     fn new() -> Self {
///         Self { saved: Rc::new(RefCell::new(Vec::new())) }
///     }
/// }
///
/// impl RestaurantRepository for MockRepo {
///     fn save(&self, restaurant: &Restaurant) -> Result<(), String> {
///         self.saved.borrow_mut().push(restaurant.clone());
///         Ok(())
///     }
/// }
///
/// let repo = MockRepo::new();
/// let saved = repo.saved.clone();
/// let service = RestaurantService::new(repo);
/// let created = service.create_restaurant("Noma".to_string(), "Copenhagen".to_string()).unwrap();
/// assert_eq!(saved.borrow().len(), 1);
/// assert_eq!(saved.borrow()[0], created);
/// ```
pub trait RestaurantRepository {
    /// Persist the provided `restaurant`.
    ///
    /// Implementations should map any storage-specific failure into a `String` error.
    fn save(&self, restaurant: &Restaurant) -> Result<(), String>;
}

/// Service layer for restaurant-related use cases.
///
/// The `RestaurantService` coordinates the creation and persistence of `Restaurant`
/// entities. It receives a `RestaurantRepository` implementation via dependency
/// injection which allows swapping storage strategies in tests and production.
///
/// # Examples
///
/// The example below shows a minimal working usage: an in-memory `MockRepo` is
/// created, the service is constructed, and `create_restaurant` is invoked. The
/// repository's captured state is then asserted.
///
/// ```rust
/// use services::restaurant_service::{RestaurantRepository, RestaurantService};
/// use domain::restaurant::Restaurant;
/// use std::cell::RefCell;
/// use std::rc::Rc;
///
/// #[derive(Clone)]
/// struct MockRepo { saved: Rc<RefCell<Vec<Restaurant>>> }
/// impl MockRepo { fn new() -> Self { Self { saved: Rc::new(RefCell::new(Vec::new())) } } }
/// impl RestaurantRepository for MockRepo {
///     fn save(&self, restaurant: &Restaurant) -> Result<(), String> {
///         self.saved.borrow_mut().push(restaurant.clone());
///         Ok(())
///     }
/// }
///
/// let repo = MockRepo::new();
/// let saved = repo.saved.clone();
/// let service = RestaurantService::new(repo);
/// let created = service.create_restaurant("Noma".to_string(), "Copenhagen".to_string()).unwrap();
/// assert_eq!(saved.borrow().len(), 1);
/// assert_eq!(saved.borrow()[0], created);
/// ```
pub struct RestaurantService<R: RestaurantRepository> {
    repository: R,
}

impl<R: RestaurantRepository> RestaurantService<R> {
    /// Create a new `RestaurantService` with the provided repository.
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    /// Create and persist a new `Restaurant`.
    ///
    /// This method generates a new `Uuid` for the restaurant, validates the input
    /// by delegating to `domain::restaurant::Restaurant::try_new`, and then asks
    /// the injected `RestaurantRepository` to save the entity.
    ///
    /// Returns the created `Restaurant` on success or a `String` error on failure.
    ///
    /// # Examples
    ///
    /// A complete example that compiles and runs as a doctest. It mirrors the
    /// in-memory repository used in other examples to show the end-to-end flow.
    ///
    /// ```rust
    /// use services::restaurant_service::{RestaurantRepository, RestaurantService};
    /// use domain::restaurant::Restaurant;
    /// use std::cell::RefCell;
    /// use std::rc::Rc;
    ///
    /// #[derive(Clone)]
    /// struct MockRepo { saved: Rc<RefCell<Vec<Restaurant>>> }
    /// impl MockRepo { fn new() -> Self { Self { saved: Rc::new(RefCell::new(Vec::new())) } } }
    /// impl RestaurantRepository for MockRepo {
    ///     fn save(&self, restaurant: &Restaurant) -> Result<(), String> {
    ///         self.saved.borrow_mut().push(restaurant.clone());
    ///         Ok(())
    ///     }
    /// }
    ///
    /// let repo = MockRepo::new();
    /// let saved = repo.saved.clone();
    /// let service = RestaurantService::new(repo);
    /// let created = service.create_restaurant("Noma".to_string(), "Copenhagen".to_string()).unwrap();
    /// assert_eq!(saved.borrow().len(), 1);
    /// assert_eq!(saved.borrow()[0], created);
    /// ```
    pub fn create_restaurant(&self, name: String, location: String) -> Result<Restaurant, String> {
        let restaurant = Restaurant::try_new(Uuid::new_v4(), name, location)
            .map_err(|error| error.to_string())?;

        self.repository.save(&restaurant)?;
        Ok(restaurant)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Default, Clone)]
    struct MockRestaurantRepo {
        saved: Rc<RefCell<Vec<Restaurant>>>,
    }

    impl RestaurantRepository for MockRestaurantRepo {
        fn save(&self, restaurant: &Restaurant) -> Result<(), String> {
            self.saved.borrow_mut().push(restaurant.clone());
            Ok(())
        }
    }

    #[test]
    fn create_restaurant_saves_valid_restaurant() {
        let mock_repo = MockRestaurantRepo::default();
        let saved = mock_repo.saved.clone();
        let service = RestaurantService::new(mock_repo);

        let created = service
            .create_restaurant("Noma".to_string(), "Copenhagen".to_string())
            .expect("restaurant should be created successfully");

        let saved_restaurants = saved.borrow();
        assert_eq!(saved_restaurants.len(), 1);
        assert_eq!(saved_restaurants[0], created);
        assert_eq!(saved_restaurants[0].name(), "Noma");
        assert_eq!(saved_restaurants[0].location(), "Copenhagen");
    }

    #[test]
    fn create_restaurant_fails_on_invalid_input() {
        let repo = MockRestaurantRepo::default();
        let service = RestaurantService::new(repo);

        // Pass an empty name, which the Domain should reject
        let result = service.create_restaurant(" ".to_string(), "Copenhagen".to_string());

        assert!(result.is_err());
    }
}
