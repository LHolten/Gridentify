use gridentify::protocol::high_score::HighScore;
use rusqlite::OpenFlags;
use rusqlite::{params, Connection};
use rusqlite_migration::{Migrations, M};

pub fn create_database() {
    let migrations = Migrations::new(vec![
        M::up(
            "CREATE TABLE IF NOT EXISTS scores (
                    id          INTEGER PRIMARY KEY AUTOINCREMENT,
                    name        TEXT NOT NULL,
                    score       UNSIGNED BIG INT NOT NULL
            );",
        ),
        M::up(
            "ALTER TABLE scores
                    ADD timestamp DATETIME;", // DEFAULT NULL
        ),
    ]);

    let mut conn = Connection::open("scores.db").unwrap();

    migrations.to_latest(&mut conn).unwrap();
}

pub(crate) fn insert_high_score(score: HighScore) {
    let conn = Connection::open_with_flags(
        "./scores.db",
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )
    .unwrap();
    conn.execute(
        "INSERT INTO scores (name, score, timestamp) 
                    VALUES (?1, ?2, CURRENT_TIMESTAMP)",
        params![score.name, score.score],
    )
    .unwrap();

    println!("{:?} got {:?}", score.name, score.score);
}

pub(crate) fn get_high_scores(daily: bool) -> Vec<HighScore> {
    let conn = Connection::open_with_flags(
        "./scores.db",
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )
    .unwrap();
    let sql = if daily {
        "SELECT name, score 
            FROM scores 
            WHERE unixepoch('now', '-1 day') < unixepoch(timestamp) 
            ORDER BY score 
            DESC LIMIT 10"
    } else {
        "SELECT name, score 
            FROM scores 
            ORDER BY score 
            DESC LIMIT 10"
    };
    let mut stmt = conn.prepare(sql).unwrap();

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

#[cfg(test)]
mod tests {
    use gridentify::protocol::high_score::HighScore;

    use super::{create_database, get_high_scores, insert_high_score};

    #[test]
    fn get_scores() {
        create_database();
        insert_high_score(HighScore {
            name: "test".to_owned(),
            score: 42,
        });
        let l = get_high_scores(true);
        assert!(!l.is_empty());
    }
}
