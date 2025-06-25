use macroquad::prelude::*;
mod collision;
use ::rand::random;
use collision::CollisionBox;

fn window_conf() -> Conf {
    Conf {
        window_title: "catchy".to_owned(),
        window_width: 600,
        window_height: 800,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let item_speed: f32 = 5.0;

    let width: f32 = 150.0;
    let height: f32 = 30.0;

    let mut items: Vec<CollisionBox> = Vec::new();

    let x: f32 = screen_width() / 2.0 - width / 2.0;
    let y: f32 = screen_height() - height * 1.25;
    let mut player: CollisionBox = CollisionBox::new(x, y, width, height);

    let mut frame: i32 = 0;

    loop {
        frame += 1;

        clear_background(WHITE);

        spawner(&mut items, &mut frame);

        draw_rectangle(player.x, player.y, player.w, player.h, GREEN);
        player.x = mouse_position().0 - width / 2.0;

        let mut to_remove: Vec<usize> = Vec::new();
        for (index, item) in items.iter_mut().enumerate() {
            draw_circle(item.x, item.y, 25.0, RED);
            item.y += item_speed;

            if player.collides_with(item) || item.y > screen_height() + item.h {
                to_remove.push(index);
            }
        }

        for index in to_remove.into_iter().rev() {
            items.remove(index);
        }

        next_frame().await
    }
}

fn spawner(items: &mut Vec<CollisionBox>, frame: &mut i32) {
    let random_timer: f32 = random();

    if items.len() < 5 && *frame > (50.0 * random_timer * 1.5) as i32 {
        *frame = 0;

        let random_x: f32 = random();

        let item_y_pos: f32 = 0.0;
        let item_x_pos: f32 = screen_width() * random_x;
        let _item: CollisionBox = CollisionBox::new(item_x_pos, item_y_pos, 25.0, 25.0);
        items.push(_item);
    }
}
