use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
pub struct TxContext {
    pub id: i32,
    pub level: i32,
    pub contract: String,
    pub operation_hash: String,
    pub operation_group_number: i32,
    pub operation_number: i32,
    pub content_number: i32,
    pub source: Option<String>,
    pub destination: Option<String>,
    pub entrypoint: Option<String>,
}

#[derive(Debug, Clone, GraphQLObject)]
pub struct Storage {
    pub id: i32,
    pub lambda_repository_creator: Option<String>,
    pub create_restrictions_creator_address: Option<String>,
    pub currency: Option<String>,
    pub tx_context: TxContext,
}

impl Storage {
    pub fn from_row(row: &tokio_postgres::Row) -> Storage {
        Storage {
            id: row.get(0),
            lambda_repository_creator: row.get(1),
            create_restrictions_creator_address: row.get(2),
            currency: row.get(3),
            tx_context: TxContext {
                id: row.get(4),
                level: row.get(5),
                contract: row.get(6),
                operation_hash: row.get(7),
                operation_group_number: row.get(8),
                operation_number: row.get(9),
                content_number: row.get(10),
                source: Some(row.get(11)),
                destination: Some(row.get(12)),
                entrypoint: Some(row.get(13)),
            },
        }
    }
}
