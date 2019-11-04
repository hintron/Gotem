mod gotem;
mod systems;

use crate::gotem::Gotem;
use amethyst::{
    core::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};


fn main() -> amethyst::Result<()> {
    use amethyst::input::{InputBundle, StringBindings};
    let app_root = application_root_dir()?;

    amethyst::start_logger(Default::default());

    let binding_path = app_root.join("config").join("bindings.ron");
    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;
    let display_config_path = app_root.join("config").join("display.ron");

    let assets_dir = app_root.join("assets");

    let game_data = GameDataBuilder::default()
        // Add the transform bundle which handles tracking entity positions
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
            .with(systems::PaddleSystem, "paddle_system", &["input_system"])
            .with(systems::MoveBallsSystem, "ball_system", &[])
            .with(
                systems::BounceSystem,
                "collision_system",
                &["paddle_system", "ball_system"],
            )
            .with(systems::WinnerSystem, "winner_system", &["ball_system"])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for
                // opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        // .with_clear([0.0, 0.0, 0.0, 1.0]),
                        // Cyan
                        .with_clear([0.00196, 0.23726, 0.21765, 1.0]),
                )
                // RenderFlat2D plugin is used to render entities with a
                // `SpriteRender` component.
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default())
        )?
        .with_bundle(UiBundle::<StringBindings>::new())?;

    let mut game = Application::new(assets_dir, Gotem::default(), game_data)?;
    game.run();

    Ok(())
}
