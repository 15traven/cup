use crate::preferences::{Preferences, DEACTIVE_ON_LOW_BATTERY_PREFERENCE, PREVENT_SCREEN_DIMMING_PREFERENCE, PREVENT_SLEEPING_PREFERENCE};

#[derive(Clone, Copy)]
pub struct Options {
    pub display: bool,
    pub idle: bool,
    pub deactivate_on_low_battery: bool
}

impl Options {
    pub fn from_preferences(preferences: &Preferences) -> Self {
        let display = preferences.load_preference(PREVENT_SCREEN_DIMMING_PREFERENCE);
        let idle = preferences.load_preference(PREVENT_SLEEPING_PREFERENCE);
        let deactivate_on_low_battery = preferences.load_preference(DEACTIVE_ON_LOW_BATTERY_PREFERENCE);

        Options {
            display,
            idle,
            deactivate_on_low_battery
        }
    }
}