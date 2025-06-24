use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "My Game".to_owned(),
        window_width: 600,
        window_height: 800,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(WHITE);

        let width: f32 = 150.0;
        let height: f32 = 30.0;

        draw_rectangle(
            mouse_position().0 - width / 2.0,
            screen_height() - height * 1.25,
            width,
            height,
            GREEN,
        );

        next_frame().await
    }
}
