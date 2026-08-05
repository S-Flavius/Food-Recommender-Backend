use crate::price::Price;
use crate::rating::Rating;
use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MealType {
    HomeCooked,
    DineIn { restaurant_id: Uuid },
    Ordered { restaurant_id: Uuid },
    StoreBought { store_id: Uuid },
}

/// Represents a meal entity in the app.
///
/// The `Meal` is represented by a structured domain object containing the
/// `Uuid` of the meal, the meal name, the date it was eaten, optional notes,
/// a `MealType`, a `Rating`, and a `Price`.
#[derive(Debug)]
pub struct Meal {
    id: Uuid,
    name: String,
    date: NaiveDate,
    notes: Option<String>,
    meal_type: MealType,
    rating: Rating,
    price: Price,
}

impl Meal {
    /// Creates a new `Meal`.
    ///
    /// # Examples
    ///
    /// ```
    /// use chrono::NaiveDate;
    /// use domain::meal::Meal;
    /// use domain::meal::MealType;
    /// use domain::price::Price;
    /// use domain::rating::Rating;
    /// use rust_decimal::Decimal;
    /// use std::str::FromStr;
    /// use uuid::Uuid;
    ///
    /// let id = Uuid::new_v4();
    /// let name = String::from("Pasta");
    /// let date = NaiveDate::from_ymd_opt(2023, 11, 16).unwrap();
    /// let notes = Some(String::from("Delicious sauce"));
    /// let meal_type = MealType::HomeCooked;
    /// let rating = Rating::try_new(3).unwrap();
    /// let price = Price::try_new("eur", Decimal::from_str("26.25").unwrap()).unwrap();
    ///
    /// let meal = Meal::try_new(id, name, date, notes, meal_type, rating, price);
    /// assert!(meal.is_ok());
    /// ```
    pub fn try_new(
        id: Uuid,
        name: String,
        date: NaiveDate,
        notes: Option<String>,
        meal_type: MealType,
        rating: Rating,
        price: Price,
    ) -> Result<Self, &'static str> {
        if name.trim().is_empty() {
            return Err("Name is empty");
        }

        Ok(Self {
            id,
            name,
            date,
            notes,
            meal_type,
            rating,
            price,
        })
    }

    /// Returns the `name` of the `Meal` object.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the `id` of the `Meal` object.
    pub fn id(&self) -> Uuid {
        self.id
    }

    /// Returns the `date` of the `Meal` object.
    pub fn date(&self) -> NaiveDate {
        self.date
    }

    /// Returns the `notes` of the `Meal` object, if any.
    pub fn notes(&self) -> Option<&str> {
        self.notes.as_deref()
    }

    /// Returns the `meal_type` of the `Meal` object.
    pub fn meal_type(&self) -> MealType {
        self.meal_type
    }

    /// Returns the `rating` of the `Meal` object.
    pub fn rating(&self) -> Rating {
        self.rating
    }

    /// Returns the `price` of the `Meal` object.
    pub fn price(&self) -> &Price {
        &self.price
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::price::Price;
    use crate::rating::Rating;
    use rust_decimal::Decimal;
    use std::str::FromStr;
    use uuid::Uuid;

    #[test]
    fn should_create_valid_meal() {
        // Arrange
        let id = Uuid::new_v4();
        let name = String::from("Pasta");
        let date = NaiveDate::from_ymd_opt(2023, 11, 16).unwrap();
        let notes = String::from("Delicious sauce");
        let meal_type = MealType::HomeCooked;
        let rating = Rating::try_new(3).unwrap();
        let price = Price::try_new("eur", Decimal::from_str("26.25").unwrap()).unwrap();

        // Act
        let meal = Meal::try_new(id, name, date, Some(notes), meal_type, rating, price);

        // Assert
        assert!(meal.is_ok());
        let meal = meal.unwrap();
        assert_eq!(meal.name(), "Pasta");
        assert_eq!(meal.date(), date);
        assert_eq!(meal.notes(), Some("Delicious sauce"));
        assert_eq!(meal.meal_type(), meal_type);
        assert_eq!(meal.rating(), rating);
        assert_eq!(meal.price().amount(), &Decimal::from_str("26.25").unwrap());
        assert_eq!(meal.price().currency(), "EUR");
    }

    #[test]
    fn should_fail_on_empty_name() {
        // Arrange
        let id = Uuid::new_v4();
        let name = String::from(" ");
        let date = NaiveDate::from_ymd_opt(2023, 11, 16).unwrap();
        let meal_type = MealType::HomeCooked;
        let rating = Rating::try_new(3).unwrap();
        let price = Price::try_new("eur", Decimal::from_str("26.25").unwrap()).unwrap();

        // Act
        let meal = Meal::try_new(id, name, date, None, meal_type, rating, price);

        // Assert
        assert!(meal.is_err());
    }

    #[test]
    fn should_create_dine_in_meal_with_restaurant() {
        // Arrange
        let id = Uuid::new_v4();
        let name = String::from("Spicy Ramen");
        let date = NaiveDate::from_ymd_opt(2026, 8, 5).unwrap();

        let restaurant_id = Uuid::new_v4();
        let meal_type = MealType::DineIn { restaurant_id }; // <-- Data inside the enum!

        let rating = Rating::try_new(5).unwrap();
        let price = Price::try_new("eur", Decimal::from_str("14.50").unwrap()).unwrap();

        // Act
        let meal = Meal::try_new(id, name, date, None, meal_type, rating, price);

        // Assert
        assert!(meal.is_ok());
        let meal = meal.unwrap();
        assert_eq!(meal.meal_type(), MealType::DineIn { restaurant_id });
    }

    #[test]
    fn should_create_store_bought_meal_with_store() {
        // Arrange
        let id = Uuid::new_v4();
        let name = String::from("Frozen Pizza");
        let date = NaiveDate::from_ymd_opt(2026, 8, 6).unwrap();

        let store_id = Uuid::new_v4();
        let meal_type = MealType::StoreBought { store_id };

        let rating = Rating::try_new(2).unwrap();
        let price = Price::try_new("usd", Decimal::from_str("5.99").unwrap()).unwrap();

        // Act
        let meal = Meal::try_new(id, name, date, None, meal_type, rating, price);

        // Assert
        assert!(meal.is_ok());
        let meal = meal.unwrap();
        assert_eq!(meal.meal_type(), MealType::StoreBought { store_id });
    }
}
