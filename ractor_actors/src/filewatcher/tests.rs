#[cfg(test)]
mod tests {
    use std::env::current_dir;
    use crate::filewatcher::{FileWatcher, FileWatcherConfig};
    use ractor::Actor;
    use tokio::test;

    #[test]
    async fn filewatch_watches_file() {
        let mut dir = current_dir().unwrap();
        let file_name = "../ractor_actors/src/filewatcher/data.db".to_string();
        dir.push(file_name);
        let fw = FileWatcher {};
        let config = FileWatcherConfig {
            files: vec![dir.clone()],
            ..Default::default()
        };

        let (_fwactor, fwhandle) = Actor::spawn(None, fw, config)
        .await
        .expect("Filewatcher failed to spawn");
        let conn = rusqlite::Connection::open("../ractor_actors/src/filewatcher/data.db").expect("Failed to load database");
        let _ = conn.execute("INSERT INTO foo values (18, 11)", ()).expect("Failed to execute query");
        fwhandle.await.unwrap();
    }
}