use sqlx::mysql::MySqlPool;
use rand::Rng;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

pub async fn create_short(pool: &MySqlPool, short: String, url: String) -> anyhow::Result<()> {
    sqlx::query!(
        "INSERT INTO shorten (shortUrl, url) VALUES (?, ?)",
        short, url
    ).execute(pool).await?;
    Ok(())
}

pub async fn get_url(pool: &MySqlPool, short: String) -> anyhow::Result<String> {
    let result = sqlx::query!(
        "SELECT url FROM shorten WHERE shortUrl = ?",
        short
    ).fetch_one(pool).await?;
    Ok(result.url)
}

pub async fn get_existed(pool: &MySqlPool, url: String) -> Option<String> {
    let result = sqlx::query!(
        "SELECT shortUrl FROM shorten WHERE url = ?",
        url
    ).fetch_one(pool).await;
    match result {
        Ok(result) => Some(result.shortUrl),
        Err(_) => None,
    }
}

pub fn create_random() -> String {
    let mut rng = rand::thread_rng();
    let mut short = String::new();
    for _ in 0..6 {
        let idx = rng.gen_range(0..ALPHABET.len());
        short.push(ALPHABET.chars().nth(idx).unwrap());
    }
    short
}
