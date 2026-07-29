/// Represents a currency used in a `Price`.
///
/// This enum lists the currencies currently supported by the application.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Currency {
    Eur,
    Usd,
}

/// Represents a monetary value combining an amount and a specific currency.
///
/// This value object prevents invalid states such as negative money and
/// ensures that cross-currency operations are handled explicitly.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Price {
    currency: Currency,
    amount: u32,
}

impl Price {
    /// Creates a new `Price`.
    ///
    /// # Examples
    /// Creating a `Price` for a coffee.
    ///
    /// ```
    /// use domain::price::{Price, Currency};
    ///
    /// let coffee_price = Price::new(Currency::Eur, 350); // 3.50 EUR
    /// assert_eq!(coffee_price.amount(), 350);
    /// ```
    pub fn new(currency: Currency, amount: u32) -> Self {
        Self { currency, amount }
    }

    /// Returns the `amount` of the `Price` object.
    pub fn amount(&self) -> u32 {
        self.amount
    }

    /// Returns the `Currency` of the `Price` object.
    pub fn currency(&self) -> Currency {
        self.currency
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_valid_price() {
        // Arrange
        let currency = Currency::Eur;
        let amount = 500;

        // Act
        let result = Price::new(currency, amount);

        // Assert
        assert_eq!(result.amount(), 500);
        assert_eq!(result.currency(), Currency::Eur);
    }

    #[test]
    fn should_create_free_meal_price_at_zero() {
        // Arrange
        let currency = Currency::Usd;
        let amount = 0;

        // Act
        let result = Price::new(currency, amount);

        // Assert
        assert_eq!(result.amount(), 0);
        assert_eq!(result.currency(), Currency::Usd);
    }
}
