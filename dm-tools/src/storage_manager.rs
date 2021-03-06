use crate::commands::Commands;
use crate::mru_files::MruFiles;
use crate::storage::EntriesCollection;
use crate::storage::Storage;
use crate::working_dirs::WorkingDirs;

use app_dirs::{AppDataType, AppInfo};

const APP_INFO: AppInfo = AppInfo {
    name: "dm-tools",
    author: "Dimitri Merejkowsky",
};

pub enum StorageType {
    CwdHistory,
    CommandsHistory,
    FilesHistory,
}

pub struct StorageManager {
    storage: Storage,
}

impl StorageManager {
    pub fn new(storage_type: StorageType) -> StorageManager {
        let app_dir = app_dirs::app_dir(AppDataType::UserData, &APP_INFO, "")
            .expect("could not create app dir");
        let entries: Box<dyn EntriesCollection> = match storage_type {
            StorageType::CwdHistory => Box::new(WorkingDirs::new()),
            StorageType::CommandsHistory => Box::new(Commands::new()),
            StorageType::FilesHistory => Box::new(MruFiles::new()),
        };
        let storage = Storage::new(entries, &app_dir);
        StorageManager { storage }
    }

    pub fn add(&mut self, entry: &str) {
        self.storage.add(entry)
    }

    pub fn remove(&mut self, entry: &str) {
        self.storage.remove(entry)
    }

    pub fn clean(&mut self) {
        let before = &self.storage.list().len();
        self.storage.clean();
        let after = &self.storage.list().len();
        println!("Cleaned {} entries over {}", before - after, before);
    }

    pub fn edit(&self) {
        let editor = std::env::var("EDITOR").expect("EDITOR not set");
        let status = std::process::Command::new(editor)
            .args(&[self.storage.db_path()])
            .status()
            .expect("Failed to execute editor");
        if !status.success() {
            panic!("Editing the db failed");
        }
    }

    pub fn list(self) {
        for entry in self.storage.list() {
            println!("{}", entry)
        }
    }

    pub fn list_reversed(self) {
        for entry in self.storage.list().iter().rev() {
            println!("{}", entry)
        }
    }

    pub fn init_kakoune(&self) {
        self.storage.init_kakoune()
    }
}
