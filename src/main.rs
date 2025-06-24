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
    let mut item_y_pos: f32 = 0.0;
    let mut item_speed: f32 = 1.0;
    let item_gravity: f32 = 0.1;

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

        drop_item(screen_width() / 2.0, item_y_pos);
        item_y_pos += item_speed;
        item_speed += item_gravity;

        next_frame().await
    }
}

fn drop_item(x: f32, y: f32) {
    draw_circle(x, y, 25.0, RED);
}
