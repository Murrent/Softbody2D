use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    request_new_screen_size(1920.0, 1080.0);
    rand::srand(get_time() as u64);

    // Refresh window
    clear_background(RED);
    next_frame().await;
    loop {
        clear_background(BLACK);

        next_frame().await
    }
}
