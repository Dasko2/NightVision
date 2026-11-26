fn nightvision_frame(game: &mut crate::game::Game<'_>) {
    let own = game.settings.gamma;
    if let Some(renderer) = game.renderer.as_mut()
        && let Some(gamma) = nightvision::frame_gamma(own)
    {
        renderer.gamma = gamma;
    }
}
