use bigdecimal::{BigDecimal, FromPrimitive};
use pg_bigdecimal::PgNumeric;

pub fn numeric_to_string(n: Option<sqlx::types::BigDecimal>) -> Option<String> {
    let optn: Option<String> = match n {
        Some(x) => Some(x.normalized().to_string()),
        None => None,
    };
    optn
}

pub fn i32_to_numeric(n: &i32) -> PgNumeric {
    let dec = BigDecimal::from_i32(*n).unwrap();
    PgNumeric { n: Some(dec) }
}
