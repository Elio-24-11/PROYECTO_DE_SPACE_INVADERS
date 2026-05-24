use macroquad::prelude::*;
use macroquad::audio::{play_sound, PlaySoundParams, Sound};
use crate::entidades::Juego;

pub fn manejar_input(juego: &mut Juego, sonido_disparo: &Sound) {
    if is_key_down(KeyCode::Left) {
        juego.nave.mover_izquierda();
    }

    if is_key_down(KeyCode::Right) {
        juego.nave.mover_derecha(); 
    }

    if is_key_pressed(KeyCode::Space) {
        juego.agregar_disparo();
        play_sound(sonido_disparo, PlaySoundParams { looped: false, volume: 1.0 });
    }
}