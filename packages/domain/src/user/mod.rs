use chrono::naive::NaiveDateTime;
use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub wallet: Option<String>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Model {
    pub fn user_info(&self) -> UserInfo {
        UserInfo {
            id: self.id,
            username: self.username.clone(),
            created_at: self.created_at,
            wallet: self.wallet.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
    pub wallet: Option<String>,
    pub created_at: NaiveDateTime,
}

pub async fn find_by_username(
    db: &DatabaseConnection,
    username: &str,
) -> anyhow::Result<Option<Model>> {
    let users = Entity::find()
        .filter(Column::Username.eq(username))
        .all(db)
        .await?;

    Ok(users.first().cloned())
}

pub fn new_user_model(
    username: String,
    password: String,
    created_at: NaiveDateTime,
    wallet: Option<String>,
) -> ActiveModel {
    ActiveModel {
        username: Set(username),
        password: Set(password),
        created_at: Set(created_at),
        wallet: Set(wallet),
        ..Default::default()
    }
}

// let user = domain::user::ActiveModel {
//     username: Set(payload.username),
//     password: Set(payload.password),
//     created_at: Set(chrono::Utc::now().naive_utc()),
//     wallet: Set(payload.wallet),
//     ..Default::default()
// };
