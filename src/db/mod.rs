use std::path::Path;

use rusqlite::Connection;

use crate::app::Stats;

pub struct DB {
    conn: Connection,
}

const TABLE_NAME: &str = "typing_tests";

impl DB {
    pub fn new(path: &Path) -> Self {
        let conn = Connection::open(path.join("stats.db")).expect("error: connecting to local db");

        conn.execute(
            format!(
                "CREATE TABLE IF NOT EXISTS {} (
            id INTEGER PRIMARY KEY,
            wpm INTEGER NOT NULL,
            accuracy REAL NOT NULL,
            typed INTEGER NOT NULL,
            misstyped INTEGER NOT NULL,
            extra INTEGER NOT NULL,
            duration INTEGER NOT NULL,
            correct INTEGER NOT NULL,
            incorrect INTEGER NOT NULL,
            created_at TEXT NOT NULL
        );",
                TABLE_NAME
            )
            .as_str(),
            (),
        )
        .expect("error: creating table");

        Self { conn }
    }

    pub fn insert_results(&self, results: Stats) -> rusqlite::Result<()> {
        self.conn
            .execute(
                format!(
                    "INSERT INTO {} (
                wpm,
                accuracy,
                typed,
                misstyped,
                extra,
                duration,
                correct,
                incorrect,
                created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, datetime('now'));",
                    TABLE_NAME
                )
                .as_str(),
                (
                    results.wpm,
                    results.accuracy,
                    results.typed,
                    results.misstyped,
                    results.extra,
                    results.time_in_seconds,
                    results.correct,
                    results.incorrect,
                ),
            )
            .expect("error: inserting results");

        Ok(())
    }

    pub fn reset_results(&self) -> rusqlite::Result<()> {
        self.conn
            .execute(format!("DELETE FROM {};", TABLE_NAME).as_str(), ())
            .expect("error: clearing stats");

        Ok(())
    }

    pub fn get_results(&self) -> Vec<Stats> {
        let mut statement = self
            .conn
            .prepare(
                format!(
                    "
                SELECT 
                    wpm, 
                    accuracy, 
                    typed, 
                    misstyped, 
                    extra, 
                    duration, 
                    correct, 
                    incorrect 
                FROM {};",
                    TABLE_NAME
                )
                .as_str(),
            )
            .expect("error: getting historic results");

        let results = statement
            .query_map([], |row| {
                Ok(Stats {
                    wpm: row.get(0)?,
                    accuracy: row.get(1)?,
                    typed: row.get(2)?,
                    misstyped: row.get(3)?,
                    extra: row.get(4)?,
                    time_in_seconds: row.get(5)?,
                    correct: row.get(6)?,
                    incorrect: row.get(7)?,
                })
            })
            .expect("error: executing GET query")
            .collect::<rusqlite::Result<Vec<Stats>>>()
            .expect("error: collecting results data");

        results
    }
}
