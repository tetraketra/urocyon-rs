use std::path::Path;

use anyhow::{Context, Error, Result, anyhow};
use sqlx::{SqlitePool, migrate::Migrator};

use crate::urocyon::args::Args;

pub struct Database {
    pub pool: SqlitePool,

    migr: Migrator,
}

impl Database {
    async fn create_pool(db_path: &str) -> Result<SqlitePool> {
        let db_path = std::path::Path::new(db_path);
        let db_path_parent = db_path
            .parent()
            .ok_or_else(|| anyhow!("Database file at path `{}` would have no parent.", db_path.display()))?;
        std::fs::create_dir_all(db_path_parent)
            .with_context(|| format!("Failed to create database file parent `{}`.", db_path_parent.display()))?;
        let db_uri = format!("file:{}?mode=rwc", db_path.display());

        SqlitePool::connect(&db_uri)
            .await
            .with_context(|| format!("Failed to open SqlitePool at uri `{}`.", db_uri))
    }

    async fn create_migrator(mgr_path: &str) -> Result<Migrator> {
        let migr_dir = std::path::Path::new(mgr_path);
        std::fs::create_dir_all(migr_dir)
            .with_context(|| format!("Failed to create migrations directory `{}`.", migr_dir.display()))?;

        Migrator::new(migr_dir)
            .await
            .with_context(|| format!("Failed to create migrator at directory `{}`.", migr_dir.display()))
    }

    pub async fn register(args: &Args) -> Result<Self> {
        let pool = Self::create_pool(&args.db_path).await?;
        let migr = Self::create_migrator(&args.mgr_path).await?;

        Ok(Self { pool, migr })
    }

    pub async fn migrate(&self) -> Result<(), Error> {
        self.migr
            .run(&self.pool)
            .await
            .with_context(|| format!("Failed to run migrations."))
    }

    pub async fn register_and_migrate(args: &Args) -> Result<Self> {
        let database = Self::register(args)
            .await
            .with_context(|| format!("Failed to register database."))?;
        let _ = database.migrate().await?;

        Ok(database)
    }
}
