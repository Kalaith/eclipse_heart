use eclipse_heart::game::Game;
use eclipse_heart::ui;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Eclipse Heart".to_owned(),
        window_width: 2560,
        window_height: 1440,
        window_resizable: true,
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new().await;

    loop {
        clear_background(ui::core::BACKGROUND);
        game.update();
        game.draw();
        next_frame().await;
    }
}
