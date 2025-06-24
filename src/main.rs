use macroquad::prelude::*;
mod collision;
use collision::CollisionBox;

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
    let item_x_pos: f32 = screen_width() / 2.0;
    let mut item_speed: f32 = 1.0;
    let item_gravity: f32 = 0.1;

    let width: f32 = 150.0;
    let height: f32 = 30.0;

    let mut collided: bool = false;

    loop {
        clear_background(WHITE);

        let x = mouse_position().0 - width / 2.0;
        let y = screen_height() - height * 1.25;

        draw_rectangle(x, y, width, height, GREEN);

        if !collided {
            drop_item(item_x_pos, item_y_pos);
            item_y_pos += item_speed;
            item_speed += item_gravity;
        }

        let player: CollisionBox = CollisionBox::new(x, y, width, height);
        let item: CollisionBox = CollisionBox::new(item_x_pos, item_y_pos, 25.0, 25.0);

        if player.collides_with(&item) {
            collided = true;
        }

        next_frame().await
    }
}

fn drop_item(x: f32, y: f32) {
    draw_circle(x, y, 25.0, RED);
}
