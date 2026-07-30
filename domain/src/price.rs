use rust_decimal::Decimal;
use rusty_money::iso::Currency;
use rusty_money::{FormattableCurrency, Money, iso};

/// Represents a monetary value combining an amount and a specific currency.
///
/// The `Price` is represented by wrapping the `Money` struct from rusty_money.
/// The amount of the price is represented by a decimal value using `Decimal`
/// struct from rust_decimal.
/// The currency of the price is represented by the struct `Currency` in the
/// iso module of rusty_money.
#[derive(Debug, PartialEq, Eq)]
pub struct Price {
    inner: Money<'static, Currency>,
}

impl Price {
    /// Creates a new `Price`.
    ///
    /// # Examples
    /// Creating a `Price` for a coffee.
    ///
    /// ```
    /// use rust_decimal::Decimal;
    /// use domain::price::{Price};
    /// use std::str::FromStr;
    ///
    /// let currency = "eUr";
    /// let amount = Decimal::from_str("3.50").unwrap();
    ///
    /// let coffee_price = Price::new(currency, amount);
    /// // In this case the price of the coffee is 3.50 Euros
    /// assert!(coffee_price.is_ok());
    /// ```
    pub fn new(currency: &str, amount: Decimal) -> Result<Self, &'static str> {
        let upper_currency = currency.to_uppercase();
        let currency_option = iso::find(&upper_currency);

        match currency_option {
            Some(valid_currency) => {
                let money = Money::from_decimal(amount, valid_currency);

                Ok(Self { inner: money })
            }

            None => Err("Currency not found"),
        }
    }

    /// Returns the `amount` of the `Price` object.
    pub fn amount(&self) -> &Decimal {
        self.inner.amount()
    }

    /// Returns the `Currency` of the `Price` object.
    pub fn currency(&self) -> &str {
        self.inner.currency().code()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn should_create_valid_price() {
        // Arrange
        let currency = "Eur";
        let amount = Decimal::from_str("1.23").unwrap();

        // Act
        let result = Price::new(currency, amount);

        // Assert
        assert!(result.is_ok());
        let price = result.unwrap();
        assert_eq!(price.amount(), &amount);
        assert_eq!(price.currency(), "EUR");
    }

    #[test]
    fn should_create_free_meal_price_at_zero() {
        // Arrange
        let currency = "usd";
        let amount = Decimal::from_str("0").unwrap();

        // Act
        let result = Price::new(currency, amount);

        // Assert
        assert!(result.is_ok());
        let price = result.unwrap();
        assert_eq!(price.amount(), &amount);
        assert_eq!(price.currency(), "USD");
    }

    #[test]
    fn should_fail_on_invalid_currency() {
        // Arrange
        let currency = "FaKe";
        let amount = Decimal::from_str("200.00").unwrap();

        // Act
        let result = Price::new(currency, amount);
        assert!(result.is_err());
    }
}
