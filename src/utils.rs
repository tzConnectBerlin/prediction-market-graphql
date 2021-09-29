pub fn numeric_to_string(n: Option<sqlx::types::BigDecimal>) -> Option<String> {
    let optn: Option<String> = match n {
        Some(x) => Some(x.normalized().to_string()),
        None => None,
    };
    optn
}
