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
    let mut items: Vec<CollisionBox> = Vec::new();

    let width: f32 = 150.0;
    let height: f32 = 30.0;
    let x: f32 = screen_width() / 2.0 - width / 2.0;
    let y: f32 = screen_height() - height * 1.25;
    let mut player: CollisionBox = CollisionBox::new(x, y, width, height);

    let mut frame: i32 = 0;
    let mut score: i32 = 0;
    let mut health: i32 = 5;

    let mut ended: bool = false;

    loop {
        frame += 1;

        clear_background(WHITE);

        if !ended {
            ended = item_logic(&mut items, &mut player, &mut score, &mut health);

            spawner(&mut items, &mut frame);

            draw_player(&mut player);

            draw_texts(score, health);
        } else {
            items = Vec::new();

            restart(&mut score, &mut health);

            end_screen();

            if is_mouse_button_pressed(MouseButton::Left) {
                ended = false;
            }
        }

        next_frame().await
    }
}

fn end_screen() {
    draw_text(
        "You lost, press anywhere to restart",
        20.0,
        screen_height() / 2.0,
        30.0,
        DARKGRAY,
    );
}

fn restart(score: &mut i32, health: &mut i32) {
    *score = 0;
    *health = 5;
}

fn draw_player(player: &mut CollisionBox) {
    let speed = 10.0;
    let mut new_x_pos = mouse_position().0 - player.w / 2.0;

    if new_x_pos > screen_width() - player.w {
        new_x_pos = screen_width() - player.w;
    } else if new_x_pos < 0.0 {
        new_x_pos = 0.0;
    }

    let mut direction = 0.0;
    if new_x_pos > player.x + speed {
        direction = speed;
    } else if new_x_pos < player.x - speed {
        direction = -speed;
    }

    player.x += direction;

    draw_rectangle(player.x, player.y, player.w, player.h, GREEN);
}

fn item_logic(
    items: &mut Vec<CollisionBox>,
    player: &mut CollisionBox,
    score: &mut i32,
    health: &mut i32,
) -> bool {
    let item_speed: f32 = 5.0;
    let mut to_remove: Vec<usize> = Vec::new();

    for (index, item) in items.iter_mut().enumerate() {
        draw_circle(item.x, item.y, 25.0, RED);
        item.y += item_speed;

        if player.collides_with(item) {
            *score += 1;
            to_remove.push(index);
        } else if item.y > screen_height() + item.h {
            *health -= 1;
            to_remove.push(index);
        }
    }

    for index in to_remove.into_iter().rev() {
        items.remove(index);
    }

    if *health == 0 {
        return true;
    }
    return false;
}

fn spawner(items: &mut Vec<CollisionBox>, frame: &mut i32) {
    let random_timer: f32 = random();

    if items.len() < 5 && *frame > (750.0 * random_timer * 1.5) as i32 {
        *frame = 0;

        let random_x: f32 = random();

        let item_width = 25.0;
        let item_height = 25.0;
        let item_y_pos: f32 = 0.0;
        let item_x_pos: f32 = (screen_width() - item_width * 2.0) * random_x + item_width;
        let _item: CollisionBox =
            CollisionBox::new(item_x_pos, item_y_pos, item_width, item_height);
        items.push(_item);
    }
}

fn draw_texts(score: i32, health: i32) {
    let score_text = format!("Score: {}", score);
    draw_text(&score_text, 20.0, 20.0, 30.0, DARKGRAY);

    let health_text = format!("Health: {}", health);
    draw_text(&health_text, 20.0, 40.0, 30.0, DARKGRAY);
}
