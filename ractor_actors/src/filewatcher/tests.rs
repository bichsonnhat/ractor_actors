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

        let (fwactor, fwhandle) = Actor::spawn(None, fw, config)
        .await
        .expect("Filewatcher failed to spawn");
        fwhandle.await.unwrap();
    }
}