use crate::error::{
    result::{StockTrekError, StockTrekResult},
    stats::StatsError,
};
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[non_exhaustive]
pub enum UnaryOperator {
    Abs,
    Neg,
    Floor,
    Ceil,
    RoundAwayFrom0,
    RoundTiesEven,
    Trunc,
    Frac,
    Sqrt,
    Exp,
    Exp2,
    Log2,
    LogE,
    Log10,
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Sinh,
    Cosh,
    Tanh,
    Asinh,
    Acosh,
    Atanh,
}

impl UnaryOperator {
    pub fn calculate(self, value: f64) -> StockTrekResult<f64> {
        let calculation_result = match self {
            UnaryOperator::Abs => value.abs(),
            UnaryOperator::Acos => {
                if value < -1.0 {
                    return Err(StockTrekError::Stats(StatsError::DomainError {
                        function: "Acos",
                        message: format!("value {} outside [-1, 1]", value),
                    }));
                }
                if value > 1.0 {
                    return Err(StockTrekError::Stats(StatsError::DomainError {
                        function: "Acos",
                        message: format!("value {} outside [-1, 1]", value),
                    }));
                }
                value.acos()
            }
            UnaryOperator::Acosh => {
                if value < 1.0 {
                    return Err(StockTrekError::Stats(StatsError::DomainError {
                        function: "Acosh",
                        message: format!("value {} < 1", value),
                    }));
                }
                value.acosh()
            }
            UnaryOperator::Asin => {
                if value < -1.0 {
                    return Err(StockTrekError::Stats(StatsError::DomainError {
                        function: "Asin",
                        message: format!("value {} outside [-1, 1]", value),
                    }));
                }
                if value > 1.0 {
                    return Err(StockTrekError::Stats(StatsError::DomainError {
                        function: "Asin",
                        message: format!("value {} outside [-1, 1]", value),
                    }));
                }
                value.asin()
            }
            UnaryOperator::Asinh => value.asinh(),
            UnaryOperator::Atan => value.atan(),
            UnaryOperator::Atanh => {
                if value == -1.0 {
                    return Err(StockTrekError::Stats(StatsError::DomainError {
                        function: "Atanh",
                        message: "value = -1 produces negative infinity".to_string(),
                    }));
                }
                if value == 1.0 {
                    return Err(StockTrekError::Stats(StatsError::DomainError {
                        function: "Atanh",
                        message: "value = 1 produces positive infinity".to_string(),
                    }));
                }
                if value < -1.0 {
                    return Err(StockTrekError::Stats(StatsError::DomainError {
                        function: "Atanh",
                        message: format!("value {} outside [-1, 1]", value),
                    }));
                }
                if value > 1.0 {
                    return Err(StockTrekError::Stats(StatsError::DomainError {
                        function: "Atanh",
                        message: format!("value {} outside [-1, 1]", value),
                    }));
                }
                value.atanh()
            }
            UnaryOperator::Ceil => value.ceil(),
            UnaryOperator::Cos => value.cos(),
            UnaryOperator::Cosh => value.cosh(),
            UnaryOperator::Exp => value.exp(),
            UnaryOperator::Exp2 => value.exp2(),
            UnaryOperator::Floor => value.floor(),
            UnaryOperator::Frac => value.fract(),
            UnaryOperator::Log10 => {
                if value == 0.0 {
                    return Err(StockTrekError::Stats(StatsError::DomainError {
                        function: "Log10",
                        message: "value = 0 is undefined".to_string(),
                    }));
                }
                if value < 0.0 {
                    return Err(StockTrekError::Stats(StatsError::DomainError {
                        function: "Log10",
                        message: format!("value {} < 0 would produce a complex number", value),
                    }));
                }
                value.log10()
            }
            UnaryOperator::Log2 => {
                if value == 0.0 {
                    return Err(StockTrekError::Stats(StatsError::DomainError {
                        function: "Log2",
                        message: "value = 0 is undefined".to_string(),
                    }));
                }
                if value < 0.0 {
                    return Err(StockTrekError::Stats(StatsError::DomainError {
                        function: "Log2",
                        message: format!("value {} < 0 would produce a complex number", value),
                    }));
                }
                value.log2()
            }
            UnaryOperator::LogE => {
                if value == 0.0 {
                    return Err(StockTrekError::Stats(StatsError::DomainError {
                        function: "LogE",
                        message: "value = 0 is undefined".to_string(),
                    }));
                }
                if value < 0.0 {
                    return Err(StockTrekError::Stats(StatsError::DomainError {
                        function: "LogE",
                        message: format!("value {} < 0 would produce a complex number", value),
                    }));
                }
                value.ln()
            }
            UnaryOperator::Neg => -value,
            UnaryOperator::RoundAwayFrom0 => value.round(),
            UnaryOperator::RoundTiesEven => value.round_ties_even(),
            UnaryOperator::Sin => value.sin(),
            UnaryOperator::Sinh => value.sinh(),
            UnaryOperator::Sqrt => {
                if value < 0.0 {
                    return Err(StockTrekError::Stats(StatsError::DomainError {
                        function: "Sqrt",
                        message: format!("value {} < 0 would produce a complex number", value),
                    }));
                }
                value.sqrt()
            }
            UnaryOperator::Tan => value.tan(),
            UnaryOperator::Tanh => value.tanh(),
            UnaryOperator::Trunc => value.trunc(),
        };
        Ok(calculation_result)
    }
}
