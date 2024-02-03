use chrono::Local;
use futures::executor;
use sqlx::MySqlPool;

use crate::{
    error::ApiResult,
    entity::role::Role, dto::role::{CreateRoleData, UpdateRoleData}
};

impl Role {
    pub async fn find_role_by_id(id: i32, pool: &MySqlPool) -> ApiResult<Role> {
        let sql = format!("SELECT * FROM {} WHERE id = ? LIMIT 1", Role::TABLE);
        Ok(sqlx::query_as(&sql).bind(id).fetch_one(pool).await?)
    }

    pub async fn find_role_all(pool: &MySqlPool) -> ApiResult<Vec<Role>> {
        let sql = format!("SELECT * FROM {}", Role::TABLE);
        Ok(sqlx::query_as(&sql).fetch_all(pool).await?)
    }

    pub async fn create_role(data: CreateRoleData, pool: &MySqlPool) -> ApiResult<u64> {
        let sql = format!(
            "
            INSERT INTO {} (name_th, name_en, role_code, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?)
            ",
            Role::TABLE
        );
        let excutor = sqlx::query(&sql)
            .bind(data.name_th)
            .bind(data.name_en)
            .bind(data.role_code)
            .bind(Local::now())
            .bind(Local::now())
            .execute(pool)
            .await?;
        Ok(excutor.rows_affected())
    }

    pub async fn update_role(data: UpdateRoleData, pool: &MySqlPool) -> ApiResult<u64> {
        let sql = format!(
            "
            UPDATE {}
            SET name_th = ?, name_en = ?, role_code = ?)
            ",
            Role::TABLE
        );
        let excutor = sqlx::query(&sql)
            .bind(data.name_th)
            .bind(data.name_en)
            .bind(data.role_code)
            .execute(pool)
            .await?;
        Ok(excutor.rows_affected())
    }
}
