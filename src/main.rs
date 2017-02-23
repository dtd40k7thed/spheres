
use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::time::Duration;
use std::sync::mpsc::channel;

#[macro_use]
extern crate qmlrs;
use qmlrs::Engine;

extern crate yaml_rust;
use yaml::{ Yaml, YamlLoader };

extern crate notify;
use notify::{ RecommendedWatcher, Watcher, RecursiveMode };

let repo = Path::new(env::home_dir())
        .join(".spheres");

fn main() {

	let mut engine = Engine::new();
	engine.load_local_file("main.qml");

	engine.exec();

}

fn update() {

    
    if (!repo.exists() || !repo.is_dir()) {

        if (!repo.is_dir()) {
            fs::remove_file(repo.to_string())?;
        }

        fs::create_dir(repo.to_string())?;

    }

    let hash = repo.join("hash");

    let hash_remote = unimplemented!(); // Download from github

    if (!hash.exists() || hash != hash_remote) {

        // Download zip from github master.
        
        // Unpack zip into directory.

    }

}

fn watcher() -> notify::Result<()> {
    let (tx, rx) = channel();
    let watch_delay = Duration::from_secs(2);
    let mut watcher: RecommendedWatcher = Watcher::new(tx, watch_delay)?;
    watcher.watch(repo.to_string(), RecursiveMode::Recursive);
    loop {
        match rx.recv() {
            Err(error) => println!("watch error: {:?}", error),
            Ok(event)  => match (event) {
                DebouncedEvent::NoticeWrite(path)  => ;
                DebouncedEvent::NoticeRemove(path) => ;
                DebouncedEvent::Create(path)       => readyaml(path);
                DebouncedEvent::Write(path)        => readyaml(path);
                DebouncedEvent::Chmod(path)        => readyaml(path);
                DebouncedEvent::Remove(path)       => readyaml(path);
                DebouncedEvent::Rename(from, dest) => { readyaml(from); readyaml(dest); };
                DebouncedEvent::Rescan()           => ;
                DebouncedEvent::Error(error, path) => ;
            }
        }
    }
}

fn readyaml(path: Path) {

    if (path.exists()) {

        let mut filecontent = String::new();
        File::open(path.to_string())
            .read_to_string(&mut filecontent)?;

        let docs = YamlLoader::load_from_string(filecontent)?;
        for doc in &docs {

            // convert document into appropriate data structure.

            match doc {
                Yaml::Real     =>;
                Yaml::Integer  =>;
                Yaml::String   =>;
                Yaml::Boolean  =>;
                Yaml::Array    =>;
                Yaml::Hash     =>;
                Yaml::Alias    =>;
                Yaml::Null     =>;
                Yaml::BadValue =>;
            }

            // Create / Update structure in db.

        }

    } else {

        // Remove document from index.

    }

}

