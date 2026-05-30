use eclipse_heart::game::Game;
use eclipse_heart::ui;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    let capture_mode = env_string("ECLIPSE_HEART_CAPTURE_SCREEN").is_some();
    Conf {
        window_title: "Eclipse Heart".to_owned(),
        window_width: env_i32("ECLIPSE_HEART_WINDOW_WIDTH", 2560),
        window_height: env_i32("ECLIPSE_HEART_WINDOW_HEIGHT", 1440),
        window_resizable: true,
        fullscreen: env_bool("ECLIPSE_HEART_FULLSCREEN", !capture_mode),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new().await;
    let capture_screen = env_string("ECLIPSE_HEART_CAPTURE_SCREEN");
    let capture_path = env_string("ECLIPSE_HEART_CAPTURE_PATH");
    let capture_frames = env_u32("ECLIPSE_HEART_CAPTURE_FRAMES", 8).max(1);
    let mut rendered_frames = 0;

    if let Some(screen_name) = capture_screen.as_deref() {
        assert!(
            game.prepare_capture_screen(screen_name),
            "unknown capture screen: {screen_name}"
        );
    }

    loop {
        show_mouse(true);
        clear_background(ui::core::BACKGROUND);
        if capture_path.is_some() {
            game.update_for_capture();
        } else {
            game.update();
        }
        game.draw();
        rendered_frames += 1;
        if let Some(path) = capture_path.as_ref() {
            if rendered_frames >= capture_frames {
                get_screen_data().export_png(path);
                break;
            }
        }
        next_frame().await;
    }
}

fn env_i32(name: &str, fallback: i32) -> i32 {
    env_string(name)
        .and_then(|value| value.parse::<i32>().ok())
        .unwrap_or(fallback)
}

fn env_u32(name: &str, fallback: u32) -> u32 {
    env_string(name)
        .and_then(|value| value.parse::<u32>().ok())
        .unwrap_or(fallback)
}

fn env_bool(name: &str, fallback: bool) -> bool {
    env_string(name)
        .map(|value| value != "0" && !value.eq_ignore_ascii_case("false"))
        .unwrap_or(fallback)
}

fn env_string(name: &str) -> Option<String> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::env::var(name).ok()
    }

    #[cfg(target_arch = "wasm32")]
    {
        let _ = name;
        None
    }
}
