use dict_derive::IntoPyObject;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, IntoPyObject)]
pub struct Score {
    pub(crate) name: String,
    pub(crate) score: u32,
}

pub(crate) fn create_database() {
    let conn = Connection::open("scores.db").unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS scores (
                    id          INTEGER PRIMARY KEY AUTOINCREMENT,
                    name        TEXT NOT NULL,
                    score       UNSIGNED BIG INT NOT NULL
                    )",
        params![],
    )
    .unwrap();
}

pub(crate) fn insert_high_score(score: Score) {
    let conn = Connection::open("./scores.db").unwrap();
    conn.execute(
        "INSERT INTO scores (name, score) VALUES (?1, ?2)",
        params![score.name, score.score as u32],
    )
    .unwrap();

    println!("{:?} got {:?}", score.name, score.score);
}

pub(crate) fn get_high_scores() -> Vec<Score> {
    let conn = Connection::open("./scores.db").unwrap();
    let mut stmt = conn
        .prepare("SELECT name, score FROM scores ORDER BY score DESC LIMIT 10")
        .unwrap();

    let score_iter = stmt
        .query_map(params![], |row| {
            Ok(Score {
                name: row.get(0)?,
                score: row.get(1)?,
            })
        })
        .unwrap();

    let mut scores: Vec<Score> = Vec::new();
    for score in score_iter {
        scores.push(score.unwrap());
    }
    scores
}
