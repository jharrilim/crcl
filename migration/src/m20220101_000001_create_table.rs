use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

// CREATE DATABASE crcl WITH OWNER = postgres ENCODING = 'UTF8' LC_COLLATE = 'en_US.UTF-8' LC_CTYPE = 'en_US.UTF-8' TEMPLATE template0 TABLESPACE = pg_default CONNECTION LIMIT = -1;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let db = manager.get_connection();

        db.execute_unprepared(
            "
            CREATE TABLE IF NOT EXISTS logins (
                id serial PRIMARY KEY NOT NULL,
                email varchar NOT NULL UNIQUE CHECK (email <> ''),
                password_hash varchar NOT NULL,
                created_at timestamp NOT NULL DEFAULT now(),
                updated_at timestamp NOT NULL DEFAULT now()
            );

            CREATE TABLE IF NOT EXISTS login_oauth_info (
                id serial PRIMARY KEY NOT NULL,
                login_id int NOT NULL REFERENCES logins(id),
                provider varchar(30) NOT NULL,
                access_token varchar(40) NOT NULL,
                refresh_token varchar(40),
                expiry timestamp,
                created_at timestamp NOT NULL DEFAULT now(),
                updated_at timestamp NOT NULL DEFAULT now()
            );

            CREATE TABLE IF NOT EXISTS users (
                id serial PRIMARY KEY NOT NULL,
                name varchar NOT NULL CHECK (name <> ''),
                login_id int NOT NULL REFERENCES logins(id),
                created_at timestamp NOT NULL DEFAULT now(),
                updated_at timestamp NOT NULL DEFAULT now()
            );

            CREATE TABLE IF NOT EXISTS scopes (
                id serial PRIMARY KEY NOT NULL,
                name varchar NOT NULL CHECK (name <> ''),
                created_at timestamp NOT NULL DEFAULT now()
            );

            CREATE TABLE IF NOT EXISTS user_scopes (
                id serial PRIMARY KEY NOT NULL,
                user_id int NOT NULL REFERENCES users(id),
                scope_id int NOT NULL REFERENCES scopes(id),
                created_at timestamp NOT NULL DEFAULT now(),
                updated_at timestamp NOT NULL DEFAULT now()
            );

            CREATE TABLE IF NOT EXISTS posts (
                id serial PRIMARY KEY NOT NULL,
                title varchar NOT NULL,
                user_id int NOT NULL REFERENCES users(id)
            );

            CREATE TABLE IF NOT EXISTS media (
                id serial PRIMARY KEY NOT NULL,
                name varchar NOT NULL,
                url varchar NOT NULL,
                post_id int NOT NULL REFERENCES posts(id),
                created_at timestamp NOT NULL DEFAULT now(),
                updated_at timestamp NOT NULL DEFAULT now()
            );
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            "
            DROP TABLE IF EXISTS media;
            DROP TABLE IF EXISTS posts;
            DROP TABLE IF EXISTS user_scopes;
            DROP TABLE IF EXISTS scopes;
            DROP TABLE IF EXISTS users;
            DROP TABLE IF EXISTS login_oauth_info;
            DROP TABLE IF EXISTS logins;
        ",
        )
        .await?;
        Ok(())
    }
}
