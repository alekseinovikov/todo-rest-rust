use refinery::config::Config;
use rocket::{Build, Rocket};
use std::str::FromStr;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!();
}

pub(crate) async fn migrate(rocket: &Rocket<Build>) {
    let figment = rocket.figment();
    let binding = figment.find_value("databases.pg_todo.url").unwrap();
    let database_url = binding.as_str().unwrap();
    let mut migration_config = Config::from_str(database_url).unwrap();
    embedded::migrations::runner().run_async(&mut migration_config).await.unwrap();
}

