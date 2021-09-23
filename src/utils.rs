use bigdecimal::{BigDecimal, FromPrimitive};
use pg_bigdecimal::PgNumeric;

pub fn numeric_to_string(n: PgNumeric) -> String {
    let optn: Option<BigDecimal> = n.n;
    match optn {
        Some(n) => n.normalized().to_string(),
        None => "Null".to_string(),
    }
}

pub fn i32_to_numeric(n: &i32) -> PgNumeric {
    let dec = BigDecimal::from_i32(*n).unwrap();
    PgNumeric { n: Some(dec) }
}
