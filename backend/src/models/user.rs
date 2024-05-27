use crate::api::AppError;
use chrono::NaiveDateTime;
use scrypt::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Scrypt,
};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgConnection};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct NewOrUpdatedUser {
    pub name: String,
    pub password: Option<String>,
    pub is_admin: bool,
}

#[derive(FromRow, Deserialize, Serialize, ToSchema, Debug)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub is_admin: bool,
    pub last_login: Option<NaiveDateTime>,
}

#[derive(FromRow, Deserialize, Serialize, Debug)]
pub struct AuthenticableUser {
    pub id: Uuid,
    pub name: String,
    pub password: String,
    pub is_admin: bool,
    pub last_login: Option<NaiveDateTime>,
}

impl From<AuthenticableUser> for User {
    fn from(val: AuthenticableUser) -> User {
        User {
            id: val.id,
            name: val.name,
            is_admin: val.is_admin,
            last_login: val.last_login,
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

    pub async fn new(user: NewOrUpdatedUser, conn: &mut PgConnection) -> Result<User, AppError> {
        let new_password = user.password.ok_or(AppError::Validation(
            "Password for new users is required".to_string(),
        ))?;
        if new_password.is_empty() {
            return Err(AppError::Validation("Password cannot be empty".to_string()));
        }

        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Scrypt
            .hash_password(new_password.as_bytes(), &salt)
            .map_err(|_| AppError::Validation("Error hashing password".to_string()))?
            .to_string();

        sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (name, password, is_admin) 
            VALUES ($1, $2, $3) 
            RETURNING
                id,
                name, 
                is_admin,
                last_login
            "#,
            user.name,
            password_hash,
            user.is_admin,
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::Database)
    }

    pub async fn update_user(
        given_id: Uuid,
        updated_user: NewOrUpdatedUser,
        conn: &mut PgConnection,
    ) -> Result<User, AppError> {
        match updated_user.password {
            Some(password) => {
                let salt = SaltString::generate(&mut OsRng);
                let password_hash = Scrypt
                    .hash_password(password.as_bytes(), &salt)
                    .map_err(|_| AppError::Validation("Error hashing password".to_string()))?
                    .to_string();

                sqlx::query_as!(
                    User,
                    r#"
                    UPDATE users 
                    SET name = $2, password = $3, is_admin = $4 
                    WHERE id = $1
                    RETURNING
                        id,
                        name,
                        is_admin,
                        last_login
                    "#,
                    given_id,
                    updated_user.name,
                    password_hash,
                    updated_user.is_admin,
                )
                .fetch_one(conn)
                .await
                .map_err(AppError::Database)
            }
            None => sqlx::query_as!(
                User,
                r#"
                    UPDATE users 
                    SET name = $2, is_admin = $3 
                    WHERE id = $1
                    RETURNING
                        id,
                        name,
                        is_admin,
                        last_login
                    "#,
                given_id,
                updated_user.name,
                updated_user.is_admin,
            )
            .fetch_one(conn)
            .await
            .map_err(AppError::Database),
        }
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
            r#"SELECT id, name, password, is_admin, last_login FROM users WHERE name = $1"#,
            given_name
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|_| AppError::BadUsernameOrPassword);

        match user_result {
            Ok(user) => {
                let password_hash = PasswordHash::new(&user.password)
                    .map_err(|_| AppError::BadUsernameOrPassword)?;
                Scrypt
                    .verify_password(given_password.as_bytes(), &password_hash)
                    .map_err(|_| AppError::BadUsernameOrPassword)?;

                sqlx::query!(
                    r#"UPDATE users SET last_login = NOW() WHERE id = $1"#,
                    user.id
                )
                .execute(&mut *conn)
                .await
                .map_err(AppError::Database)?;

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
        sqlx::query_as!(User, r#"SELECT id, name, is_admin, last_login FROM users"#)
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
            r#"SELECT id, name, is_admin, last_login FROM users WHERE id = $1"#,
            given_id
        )
        .fetch_one(conn)
        .await
        .map_err(AppError::Database)
    }
}
