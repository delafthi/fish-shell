use crate::wchar::prelude::*;
use std::sync::OnceLock;

static ATUIN_DB: OnceLock<Option<atuin_client::database::Sqlite>> = OnceLock::new();

fn get_db() -> Option<&'static atuin_client::database::Sqlite> {
    ATUIN_DB
        .get_or_init(|| {
            let settings = atuin_client::settings::Settings::new().ok()?;
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_time()
                .build()
                .ok()?;
            rt.block_on(async { atuin_client::database::Sqlite::new(&settings.db_path, 5.0).await })
                .ok()
        })
        .as_ref()
}

pub fn search(query: &wstr) -> Result<Option<String>, ()> {
    if query.is_empty() {
        return Ok(None);
    }

    let db = get_db().ok_or(())?;
    let query_str = query.to_string();

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .map_err(|_| ())?;

    let result = rt
        .block_on(async {
            use atuin_client::database::Database;

            let context = atuin_client::database::Context {
                session: std::env::var("ATUIN_SESSION").unwrap_or_default(),
                cwd: std::env::current_dir()
                    .ok()
                    .and_then(|p| p.to_str().map(String::from))
                    .unwrap_or_default(),
                hostname: String::new(),
                host_id: String::new(),
                git_root: None,
            };

            db.search(
                atuin_client::settings::SearchMode::Prefix,
                atuin_client::settings::FilterMode::Global,
                &context,
                &query_str,
                atuin_client::database::OptFilters {
                    limit: Some(1),
                    ..Default::default()
                },
            )
            .await
        })
        .map_err(|_| ())?;

    Ok(result.first().map(|h| h.command.clone()))
}
