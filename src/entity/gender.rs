use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Gender {
    pub id: i32,
    pub name_th: String,
    pub name_en: String,
    pub is_deleted: bool,
}

impl Gender {
    pub const TABLE: &'static str = "gender_info";
}
