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

pub(crate) fn insert_high_score(name: &str, score: &u64) {
    let conn = Connection::open("./scores.db").unwrap();
    conn.execute(
        "INSERT INTO scores (name, score) VALUES (?1, ?2)",
        params![name, *score as u32],
    )
    .unwrap();

    println!("{:?} got {:?}", name, score);
}

pub(crate) fn get_high_scores() -> Vec<(String, u32)> {
    let conn = Connection::open("./scores.db").unwrap();
    let mut stmt = conn
        .prepare("SELECT name, score FROM scores ORDER BY score DESC LIMIT 10")
        .unwrap();

    let score_iter = stmt
        .query_map(params![], |row| Ok((row.get(0)?, row.get(1)?)))
        .unwrap();

    let mut scores: Vec<(String, u32)> = Vec::new();
    for score in score_iter {
        scores.push(score.unwrap());
    }
    scores
}
