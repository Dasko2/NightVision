//! NightVision: a gamma boost and night vision, each on its own key.
//! Client side only: nothing is sent to the server, no setting of the game is changed.
use std::cell::Cell;

pub const ID: &str = "nightvision";
/// Gamma choices, in percent of the video slider's "Bright" (100%).
pub const LEVELS: [u32; 5] = [200, 500, 1000, 1500, 3000];
const DEFAULT_LEVEL: usize = 3;

pub const GAMMA_ON: &str = "gamma_on";
pub const GAMMA_LEVEL: &str = "gamma_level";
pub const NIGHT_VISION_ON: &str = "night_vision_on";

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct State {
    /// The gamma to draw with, or `None` to keep the player's own.
    pub gamma: Option<f32>,
    pub night_vision: bool,
}

impl State {
    /// From the stored settings: toggles as "true"/"false", the level as a choice index.
    pub fn read(gamma_on: Option<&str>, level: Option<&str>, night_vision: Option<&str>) -> Self {
        let on = |v: Option<&str>| v.is_some_and(|v| v.trim() == "true");
        let percent = level
            .and_then(|v| v.trim().parse::<usize>().ok())
            .and_then(|i| LEVELS.get(i).copied())
            .unwrap_or(LEVELS[DEFAULT_LEVEL]);
        Self {
            gamma: on(gamma_on).then_some(percent as f32 / 100.0),
            night_vision: on(night_vision),
        }
    }
}

thread_local! {
    static STATE: Cell<State> = const { Cell::new(State { gamma: None, night_vision: false }) };
    static GAMMA_APPLIED: Cell<bool> = const { Cell::new(false) };
}

pub fn set_state(state: State) {
    STATE.set(state);
}

/// What to write into the renderer this frame: the boost while on, the player's own
/// gamma once after it is turned off, nothing otherwise.
pub fn frame_gamma(own: f32) -> Option<f32> {
    match STATE.get().gamma {
        Some(boost) => {
            GAMMA_APPLIED.set(true);
            Some(boost)
        }
        None => GAMMA_APPLIED.replace(false).then_some(own),
    }
}

/// The night vision strength (0..1) to draw with.
pub fn night_vision(game_value: f32) -> f32 {
    if STATE.get().night_vision { 1.0 } else { game_value }
}

#[derive(Default)]
pub struct Mod;

#[mod_api::horizon_mod]
impl mod_api::Hooks for Mod {
    fn register(&self, registrar: &mut dyn mod_api::Registrar, _: &mod_api::RegistrationContext) {
        use mod_api::{KeyConflictContext, KeyRegistration, SettingSpec};
        let _ = registrar.register_setting(SettingSpec::toggle(
            GAMMA_ON,
            "nightvision.setting.gamma",
            false,
        ));
        let _ = registrar.register_setting(SettingSpec::choice(
            GAMMA_LEVEL,
            "nightvision.setting.level",
            [
                "nightvision.level.200",
                "nightvision.level.500",
                "nightvision.level.1000",
                "nightvision.level.1500",
                "nightvision.level.3000",
            ],
            DEFAULT_LEVEL as _,
        ));
        let _ = registrar.register_setting(SettingSpec::toggle(
            NIGHT_VISION_ON,
            "nightvision.setting.night_vision",
            false,
        ));
        for (id, key, label) in [
            ("gamma", "key.keyboard.h", "nightvision.key.gamma"),
            ("night_vision", "key.keyboard.n", "nightvision.key.night_vision"),
        ] {
            registrar.register_key_mapping(
                KeyRegistration::new(id, key)
                    .label(label)
                    .category("nightvision.category")
                    .context(KeyConflictContext::InGame),
            );
        }
    }

    // Empty on purpose: the game only calls the tick function the `tick` mixin
    // patches when some mod listens to this hook.
    fn client_tick_pre(&self, _: &mut dyn mod_api::ClientHost, _: &mut mod_api::ClientTickPreEvent) {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn off_by_default() {
        assert_eq!(State::read(None, None, None), State::default());
    }

    #[test]
    fn reads_levels_and_toggles() {
        let s = State::read(Some("true"), Some("0"), Some("true\n"));
        assert_eq!(s.gamma, Some(2.0));
        assert!(s.night_vision);
        assert_eq!(State::read(Some("true"), Some("99"), None).gamma, Some(15.0));
    }

    #[test]
    fn gives_the_own_gamma_back_once() {
        set_state(State { gamma: Some(15.0), night_vision: false });
        assert_eq!(frame_gamma(0.5), Some(15.0));
        set_state(State::default());
        assert_eq!(frame_gamma(0.5), Some(0.5));
        assert_eq!(frame_gamma(0.5), None);
    }

    #[test]
    fn night_vision_replaces_the_game_value_only_while_on() {
        assert_eq!(night_vision(0.2), 0.2);
        set_state(State { gamma: None, night_vision: true });
        assert_eq!(night_vision(0.2), 1.0);
    }
}
