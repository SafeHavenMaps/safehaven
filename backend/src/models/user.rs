use crate::api::AppError;
use scrypt::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Scrypt,
};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgConnection};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct NewUser {
    pub name: String,
    pub password: String,
}

#[derive(FromRow, Deserialize, Serialize, ToSchema, Debug)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub is_admin: bool,
}

#[derive(FromRow, Deserialize, Serialize, Debug)]
pub struct AuthenticableUser {
    pub id: Uuid,
    pub name: String,
    pub password: String,
    pub is_admin: bool,
}

impl From<AuthenticableUser> for User {
    fn from(val: AuthenticableUser) -> User {
        User {
            id: val.id,
            name: val.name,
            is_admin: val.is_admin,
        }
    }
}

impl User {
    pub async fn new(user: NewUser, conn: &mut PgConnection) -> Result<User, AppError> {
        if user.password.is_empty() {
            return Err(AppError::ValidationError(
                "Password cannot be empty".to_string(),
            ));
        }

        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Scrypt
            .hash_password(user.password.as_bytes(), &salt)
            .map_err(|_| AppError::ValidationError("Error hashing password".to_string()))?
            .to_string();

        let new_user = NewUser {
            name: user.name.to_string(),
            password: password_hash,
        };

        sqlx::query_as!(
            User,
            r#"INSERT INTO users (name, password) VALUES ($1, $2) RETURNING id, name, is_admin"#,
            new_user.name,
            new_user.password
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::DatabaseError)
    }

    pub async fn change_password(
        given_id: Uuid,
        new_password: String,
        conn: &mut PgConnection,
    ) -> Result<(), AppError> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Scrypt
            .hash_password(new_password.as_bytes(), &salt)
            .map_err(|_| AppError::ValidationError("Error hashing password".to_string()))?
            .to_string();

        sqlx::query!(
            r#"UPDATE users SET password = $1 WHERE id = $2"#,
            password_hash,
            given_id
        )
        .execute(conn)
        .await
        .map_err(AppError::DatabaseError)?;

        Ok(())
    }

    pub async fn authenticate(
        given_name: String,
        given_password: String,
        conn: &mut PgConnection,
    ) -> Result<User, AppError> {
        let user: AuthenticableUser = sqlx::query_as!(
            AuthenticableUser,
            r#"SELECT id, name, password, is_admin FROM users WHERE name = $1"#,
            given_name
        )
        .fetch_one(conn)
        .await
        .map_err(|_| AppError::BadUsernameOrPasswordError)?;

        let password_hash =
            PasswordHash::new(&user.password).map_err(|_| AppError::BadUsernameOrPasswordError)?;

        Scrypt
            .verify_password(given_password.as_bytes(), &password_hash)
            .map_err(|_| AppError::BadUsernameOrPasswordError)?;

        Ok(user.into())
    }

    pub async fn list(conn: &mut PgConnection) -> Result<Vec<User>, AppError> {
        sqlx::query_as!(User, r#"SELECT id, name, is_admin FROM users"#)
            .fetch_all(conn)
            .await
            .map_err(AppError::DatabaseError)
    }

    pub async fn delete(given_id: Uuid, conn: &mut PgConnection) -> Result<(), AppError> {
        sqlx::query!(r#"DELETE FROM users WHERE id = $1"#, given_id)
            .execute(conn)
            .await
            .map_err(AppError::DatabaseError)?;
        Ok(())
    }

    pub async fn get(given_id: Uuid, conn: &mut PgConnection) -> Result<User, AppError> {
        sqlx::query_as!(
            User,
            r#"SELECT id, name, is_admin FROM users WHERE id = $1"#,
            given_id
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::DatabaseError)
    }
}
