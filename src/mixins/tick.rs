fn nightvision_tick(game: &mut crate::game::Game<'_>) {
    for (key, setting) in [
        ("gamma", nightvision::GAMMA_ON),
        ("night_vision", nightvision::NIGHT_VISION_ON),
    ] {
        if game.consume_click(nightvision::ID, key) {
            let on = game
                .stored_setting(nightvision::ID, setting)
                .is_some_and(|v| v.trim() == "true");
            game.set_setting(nightvision::ID, setting, if on { "false" } else { "true" });
        }
    }
    let get = |key| game.stored_setting(nightvision::ID, key);
    let (gamma_on, level, night_vision) = (
        get(nightvision::GAMMA_ON),
        get(nightvision::GAMMA_LEVEL),
        get(nightvision::NIGHT_VISION_ON),
    );
    nightvision::set_state(nightvision::State::read(
        gamma_on.as_deref(),
        level.as_deref(),
        night_vision.as_deref(),
    ));
}
