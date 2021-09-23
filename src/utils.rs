use bigdecimal::{BigDecimal, FromPrimitive};
use pg_bigdecimal::PgNumeric;

pub fn numeric_to_string(n: Option<PgNumeric>) -> Option<String> {
    let optn: Option<BigDecimal> = match n {
        Some(x) => x.n,
        None => None,
    };
    optn.map(|n| n.normalized().to_string())
}

pub fn i32_to_numeric(n: &i32) -> PgNumeric {
    let dec = BigDecimal::from_i32(*n).unwrap();
    PgNumeric { n: Some(dec) }
}
