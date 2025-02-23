use redb::{Database, TableDefinition};

const DB_PATH: &str = "preferences.redb";
const TABLE: TableDefinition<&str, bool> = TableDefinition::new("preferences");

pub const DEACTIVE_ON_LOW_BATTERY_PREFERENCE: &str = "deactivate_on_low_battery";
pub const PREVENT_SCREEN_DIMMING_PREFERENCE: &str = "prevent_screen_diming";
pub const PREVENT_SLEEPING_PREFERENCE: &str = "prevent_sleeping";

pub struct Preferences {
    db: Database
}

impl Preferences {
    pub fn new() -> Self {
        let db = Database::create(DB_PATH).expect("Failed to create database");
        let txn = db.begin_write().expect("Failed to begin write");
        {
            let _table = txn.open_table(TABLE).expect("Failed to open table");
        }
        let _ = txn.commit();

        Self { db }
    }

    pub fn set_initial_values(&self) {
        if !self.exists(DEACTIVE_ON_LOW_BATTERY_PREFERENCE) {
            self.save_preference(DEACTIVE_ON_LOW_BATTERY_PREFERENCE, true);
        }

        if !self.exists(PREVENT_SCREEN_DIMMING_PREFERENCE) {
            self.save_preference(PREVENT_SCREEN_DIMMING_PREFERENCE, true);
        }

        if !self.exists(PREVENT_SLEEPING_PREFERENCE) {
            self.save_preference(PREVENT_SLEEPING_PREFERENCE, true);
        }
    }

    pub fn load_preference(&self, key: &str) -> bool {
        let tnx = self.db.begin_read().expect("failed to begin read transaction");
        let table = tnx.open_table(TABLE).expect("Failed to open table");

        table.get(key)
            .expect("Failed to read preference")
            .unwrap()
            .value()
    }

    pub fn save_preference(&self, key: &str, value: bool) {
        let txn = self.db.begin_write().expect("Failed to begin write");
        {
            let mut table = txn.open_table(TABLE).expect("Failed to open table");
            let _ = table.insert(key, value);
        }
        let _ = txn.commit();
    }
    
    pub fn toggle_preference(&self, key: &str) {
        let current_value = self.load_preference(key);

        match current_value {
            true => {
                self.save_preference(key, false);
            }
            false => {
                self.save_preference(key, true);
            }
        }
    }

    fn exists(&self, key: &str) -> bool {
        let txn = self.db.begin_read().expect("Failed to begin read");
        let table = txn.open_table(TABLE).expect("Failed to open table");
        {
            table.get(key).expect("Failed to read preference").is_some()
        }
    }
}