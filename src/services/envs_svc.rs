use crate::{
    db::db::ConfigMonkeyDb,
    models::{app::App, env::Env, list::List},
    repos::envs_repo,
};
use rocket_db_pools::Connection;

pub enum EnvsServiceError {
    Unknown,
}

impl EnvsServiceError {
    pub fn code(&self) -> &str {
        match *self {
            EnvsServiceError::Unknown => "unknown",
        }
    }
    pub fn message(&self) -> &str {
        match *self {
            EnvsServiceError::Unknown => "Unknown error",
        }
    }
}

const DEFAULT_LIMIT: i32 = 10;
const DEFAULT_OFFSET: i32 = 0;

pub async fn get_envs(
    db: Connection<ConfigMonkeyDb>,
    app_slug: &str,
    limit_opt: Option<i32>,
    offset_opt: Option<i32>,
) -> Result<List<Env>, EnvsServiceError> {
    let limit = limit_opt.unwrap_or(DEFAULT_LIMIT);
    let offset = offset_opt.unwrap_or(DEFAULT_OFFSET);

    let result = envs_repo::get_envs(db, app_slug, limit, offset).await;
    match result {
        Ok(apps) => {
            let count = apps.len() as i32;
            Ok(List {
                items: apps,
                count,
                limit,
                offset,
                next_offset: if count == limit {
                    Some(offset + limit)
                } else {
                    None
                },
                prev_offset: if offset == 0 {
                    None
                } else if offset - limit >= 0 {
                    Some(offset - limit)
                } else {
                    Some(0)
                },
            })
        }
        Err(apps_repo_err) => match apps_repo_err {
            _ => Err(EnvsServiceError::Unknown),
        },
    }
}
