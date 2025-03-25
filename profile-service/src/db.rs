use sqlx::types::Uuid;
use sqlx::{Pool, Postgres, Result};

use crate::models::Profile;

pub struct DB {
  pub pool: Pool<Postgres>,
}

impl DB {
  pub async fn get_profile_by_user_id(&self, user_id: Uuid) -> Result<Option<Profile>> {
    let result: Option<Profile> = sqlx::query_as::<_, Profile>(
      r#"
        SELECT * FROM profiles
        WHERE user_id = $1
        LIMIT 1
      "#,
    )
    .bind(user_id)
    .fetch_optional(&self.pool)
    .await?;

    Ok(result)
  }

  pub async fn get_profiles(&self) -> Result<Vec<Profile>> {
    let result: Vec<Profile> = sqlx::query_as::<_, Profile>(r#"SELECT * FROM profiles"#)
      .fetch_all(&self.pool)
      .await?;

    Ok(result)
  }
}
