use macroquad::prelude::*;
mod collision;
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
    let item_speed: f32 = 4.0;

    let width: f32 = 150.0;
    let height: f32 = 30.0;

    let mut items: Vec<CollisionBox> = Vec::new();

    let x = screen_width() / 2.0 - width / 2.0;
    let y = screen_height() - height * 1.25;
    let mut player: CollisionBox = CollisionBox::new(x, y, width, height);

    loop {
        clear_background(WHITE);

        spawner(&mut items);

        draw_rectangle(player.x, player.y, player.w, player.h, GREEN);
        player.x = mouse_position().0 - width / 2.0;

        let mut to_remove = Vec::new();
        for (index, item) in items.iter_mut().enumerate() {
            draw_circle(item.x, item.y, 25.0, RED);
            item.y += item_speed;

            if player.collides_with(item) {
                to_remove.push(index);
            }
        }

        for index in to_remove.into_iter().rev() {
            items.remove(index);
        }

        next_frame().await
    }
}

fn spawner(items: &mut Vec<CollisionBox>) {
    if items.len() < 1 {
        let item_y_pos: f32 = 0.0;
        let item_x_pos: f32 = screen_width() / 2.0;
        let _item: CollisionBox = CollisionBox::new(item_x_pos, item_y_pos, 25.0, 25.0);
        items.push(_item);
    }
}
