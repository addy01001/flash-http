use std::fs;
use std::path::Path;

use diesel::sqlite::SqliteConnection;
use diesel::Connection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dirs::home_dir;
const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

// Check if a database file exists, and create one if it does not.
pub fn init() {
    if !db_file_exists() {
        create_db_file();
    }
    run_migrations();
}

// Create the database file.
fn create_db_file() {
    let db_path = get_db_path();
    let db_dir = Path::new(&db_path).parent().unwrap();

    // If the parent directory does not exist, create it.
    if !db_dir.exists() {
        fs::create_dir_all(db_dir).unwrap();
    }

    // Create the database file.
    fs::File::create(db_path).unwrap();
}

pub fn estabilish_connection()-> SqliteConnection {
    let db_path = get_db_path().clone();

    SqliteConnection::establish(db_path.as_str())
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_path))
}

fn run_migrations() {
    let mut connection = estabilish_connection();
    connection.run_pending_migrations(MIGRATIONS).unwrap();
}

// Check whether the database file exists.
fn db_file_exists() -> bool {
    let db_path = get_db_path();
    Path::new(&db_path).exists()
}

// Get the path where the database file should be located.
fn get_db_path() -> String {
    // let home_dir = home_dir().unwrap().to_str().unwrap();

    "C://Users/Andrew/.config/flash-http/database.sqlite".to_string()
}