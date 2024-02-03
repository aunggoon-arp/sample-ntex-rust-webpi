use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Address {
    pub id: i32,
    pub addr1: String,
    pub addr2: String,
    pub addr3: String,
    pub addr4: String,
    pub addr5: String,
    pub postcode: String,
    pub is_deleted: bool,
}

impl Address {
    pub const TABLE: &'static str = "address_info";
}
