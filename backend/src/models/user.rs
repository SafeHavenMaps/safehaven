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
    pub async fn get_users_count(conn: &mut PgConnection) -> Result<i64, AppError> {
        let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
            .fetch_one(conn)
            .await
            .map_err(AppError::Database)?;
        Ok(count)
    }

    pub async fn new(user: NewUser, conn: &mut PgConnection) -> Result<User, AppError> {
        if user.password.is_empty() {
            return Err(AppError::Validation("Password cannot be empty".to_string()));
        }

        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Scrypt
            .hash_password(user.password.as_bytes(), &salt)
            .map_err(|_| AppError::Validation("Error hashing password".to_string()))?
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
        .map_err(AppError::Database)
    }

    pub async fn change_password(
        given_id: Uuid,
        new_password: String,
        conn: &mut PgConnection,
    ) -> Result<(), AppError> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Scrypt
            .hash_password(new_password.as_bytes(), &salt)
            .map_err(|_| AppError::Validation("Error hashing password".to_string()))?
            .to_string();

        sqlx::query!(
            r#"UPDATE users SET password = $1 WHERE id = $2"#,
            password_hash,
            given_id
        )
        .execute(conn)
        .await
        .map_err(AppError::Database)?;

        Ok(())
    }

    // We do not want the password hash to be optimized out, so we do not use map
    #[allow(clippy::bind_instead_of_map)]
    pub async fn authenticate(
        given_name: String,
        given_password: String,
        conn: &mut PgConnection,
    ) -> Result<User, AppError> {
        let user_result: Result<AuthenticableUser, AppError> = sqlx::query_as!(
            AuthenticableUser,
            r#"SELECT id, name, password, is_admin FROM users WHERE name = $1"#,
            given_name
        )
        .fetch_one(conn)
        .await
        .map_err(|_| AppError::BadUsernameOrPassword);

        match user_result {
            Ok(user) => {
                let password_hash = PasswordHash::new(&user.password)
                    .map_err(|_| AppError::BadUsernameOrPassword)?;
                Scrypt
                    .verify_password(given_password.as_bytes(), &password_hash)
                    .map_err(|_| AppError::BadUsernameOrPassword)?;

                Ok(user.into())
            }
            Err(error) => {
                // Hashing of the password is done even though the user was not found to reduce the risk of timing attacks
                // Tracing of the hash is done to avoid compiler optimizing out the hash computation
                let salt = SaltString::generate(&mut OsRng);
                let _ = Scrypt
                    .hash_password(given_password.as_bytes(), &salt)
                    .and_then(|hash| {
                        tracing::warn!(
                            "Failed admin site authentication attempt with username {:?} and password hash starting with {:?}",
                            given_name,
                            &hash.to_string()[..5]
                        );
                        Ok(())
                    });

                Err(error)
            }
        }
    }

    pub async fn list(conn: &mut PgConnection) -> Result<Vec<User>, AppError> {
        sqlx::query_as!(User, r#"SELECT id, name, is_admin FROM users"#)
            .fetch_all(conn)
            .await
            .map_err(AppError::Database)
    }

    pub async fn delete(given_id: Uuid, conn: &mut PgConnection) -> Result<(), AppError> {
        sqlx::query!(r#"DELETE FROM users WHERE id = $1"#, given_id)
            .execute(conn)
            .await
            .map_err(AppError::Database)?;
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
        .map_err(AppError::Database)
    }
}
