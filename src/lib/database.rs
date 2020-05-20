use crate::lib::high_score::HighScore;
use rusqlite::OpenFlags;
use rusqlite::{params, Connection};

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

pub(crate) fn insert_high_score(score: HighScore) {
    let conn = Connection::open_with_flags(
        "./scores.db",
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )
    .unwrap();
    conn.execute(
        "INSERT INTO scores (name, score) VALUES (?1, ?2)",
        params![score.name, score.score as u32],
    )
    .unwrap();

    println!("{:?} got {:?}", score.name, score.score);
}

pub(crate) fn get_high_scores() -> Vec<HighScore> {
    let conn = Connection::open_with_flags(
        "./scores.db",
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )
    .unwrap();
    let mut stmt = conn
        .prepare("SELECT name, score FROM scores ORDER BY score DESC LIMIT 10")
        .unwrap();

    let score_iter = stmt
        .query_map(params![], |row| {
            Ok(HighScore {
                name: row.get(0)?,
                score: row.get(1)?,
            })
        })
        .unwrap();

    let mut scores: Vec<HighScore> = Vec::new();
    for score in score_iter {
        scores.push(score.unwrap());
    }
    scores
}
