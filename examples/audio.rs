use macroquad::{audio, prelude::*};

#[macroquad::main("Audio")]
async fn main() {
    set_pc_assets_folder("examples");

    let sound1 = audio::load_sound("sound.wav").await.unwrap();
    let sound2 = audio::load_sound("sound2.wav").await.unwrap();

    loop {
        clear_background(LIGHTGRAY);

        if is_key_pressed(KeyCode::A) {
            warn!("play 1!");
            audio::play_sound_once(&sound1);
        }
        if is_key_pressed(KeyCode::B) {
            warn!("play 2!");
            audio::play_sound_once(&sound2);
        }
        next_frame().await
    }
}
