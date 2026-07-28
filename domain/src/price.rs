#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Currency {
    EUR,
    USD,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Price {
    currency: Currency,
    amount: u32,
}

impl Price {
    pub fn new(currency: Currency, amount: u32) -> Self {
        Price { currency, amount }
    }

    pub fn amount(&self) -> u32 {
        self.amount
    }

    pub fn currency(&self) -> Currency {
        self.currency
    }
}

#[cfg(test)]
mod tests {
    // Import everything from parent module
    use super::*;

    #[test]
    fn should_create_valid_price() {
        // Arrange
        let amount = 500;
        let currency = Currency::EUR;

        // Act
        let result = Price::new(currency, amount);

        // Assert
        assert_eq!(result.amount(), 500);
        assert_eq!(result.currency(), Currency::EUR);
    }

    #[test]
    fn should_create_free_meal_price_at_zero() {
        // Arrange
        let amount = 0;
        let currency = Currency::USD;

        // Act
        let result = Price::new(currency, amount);

        // Assert
        assert_eq!(result.amount(), 0);
        assert_eq!(result.currency(), Currency::USD);
    }
}
