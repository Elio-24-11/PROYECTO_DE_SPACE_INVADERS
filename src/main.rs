use macroquad::prelude::*;
use macroquad::audio::{load_sound, play_sound, PlaySoundParams};

mod entidades;
mod input;

use entidades::Juego;

fn conf() -> Conf {
    Conf {
        window_title: "Space Invaders".to_string(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    // cargar assets
    let textura_jugador  = load_texture("assets/jugador.png").await.unwrap();
    let textura_enemigo  = load_texture("assets/enemigos.png").await.unwrap();
    let textura_fondo    = load_texture("assets/fondo.png").await.unwrap();
    let sonido_disparo   = load_sound("assets/sonidos_de_disparos.wav").await.unwrap();
    let sonido_explosion = load_sound("assets/sonido_de_explosion.wav").await.unwrap();
    let musica = load_sound("assets/Raining_Bits.ogg").await.unwrap();

    // musica de fondo en loop
    play_sound(&musica, PlaySoundParams { looped: true, volume: 0.5 });

    let mut juego = Juego::new();

    // crear aliens en filas
    for fila in 0..4 {
        for col in 0..10 {
            let x = 80.0 + col as f32 * 65.0;
            let y = 60.0 + fila as f32 * 50.0;
            juego.agregar_alien(x, y);
        }
    }

    // crear bunkers
    juego.crear_bunkers();

    let mut timer_disparo_alien: f32 = 0.0;
    let mut intervalo_disparo_alien: f32 = 1.5;

    loop {
        // fondo
        draw_texture_ex(
            &textura_fondo,
            0.0, 0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(800.0, 600.0)),
                ..Default::default()
            },
        );

        // input
        input::manejar_input(&mut juego, &sonido_disparo);

        // mover disparos jugador
        for i in 0..juego.num_disparos {
            if juego.disparos[i].activo {
                juego.disparos[i].mover();
                if juego.disparos[i].y < 0.0 {
                    juego.disparos[i].activo = false;
                }
            }
        }

        // mover aliens
        juego.mover_aliens();

        // disparo alien automatico
        timer_disparo_alien += get_frame_time();
        if timer_disparo_alien >= intervalo_disparo_alien {
            timer_disparo_alien = 0.0;

            let mut vivos: [usize; 50] = [0; 50];
            let mut num_vivos = 0;
            for i in 0..juego.num_aliens {
                if juego.aliens[i].vivo {
                    vivos[num_vivos] = i;
                    num_vivos += 1;
                }
            }
            if num_vivos > 0 {
                let idx = (macroquad::time::get_time() as usize) % num_vivos;
                let alien = juego.aliens[vivos[idx]];
                juego.agregar_disparo_alien(alien.x, alien.y);
            }
        }

        // mover disparos alien
        for i in 0..juego.num_disparos_alien {
            if juego.disparos_alien[i].activo {
                juego.disparos_alien[i].mover();
                if juego.disparos_alien[i].y > 600.0 {
                    juego.disparos_alien[i].activo = false;
                }
            }
        }

        // colision disparo jugador vs alien
        for i in 0..juego.num_disparos {
            if juego.disparos[i].activo {
                for j in 0..juego.num_aliens {
                    if juego.aliens[j].vivo {
                        let dx = (juego.disparos[i].x - juego.aliens[j].x).abs();
                        let dy = (juego.disparos[i].y - juego.aliens[j].y).abs();
                        if dx < 20.0 && dy < 20.0 {
                            juego.aliens[j].vivo = false;
                            juego.disparos[i].activo = false;
                            juego.puntuacion += 10;
                            play_sound(&sonido_explosion, PlaySoundParams { looped: false, volume: 1.0 });
                        }
                    }
                }
            }
        }

        // colision disparo alien vs nave
        for i in 0..juego.num_disparos_alien {
            if juego.disparos_alien[i].activo {
                let dx = (juego.disparos_alien[i].x - juego.nave.x).abs();
                let dy = (juego.disparos_alien[i].y - juego.nave.y).abs();
                if dx < 20.0 && dy < 20.0 {
                    juego.disparos_alien[i].activo = false;
                    if juego.nave.vidas > 0 {
                        juego.nave.vidas -= 1;
                        play_sound(&sonido_explosion, PlaySoundParams { looped: false, volume: 1.0 });
                    }
                }
            }
        }

        // colision disparo jugador vs bunker
        for i in 0..juego.num_disparos {
            if juego.disparos[i].activo {
                for b in 0..4 {
                    if juego.bunkers[b].activo {
                        let dx = (juego.disparos[i].x - juego.bunkers[b].x).abs();
                        let dy = (juego.disparos[i].y - juego.bunkers[b].y).abs();
                        if dx < 30.0 && dy < 20.0 {
                            juego.disparos[i].activo = false;
                            if juego.bunkers[b].vida > 0 {
                                juego.bunkers[b].vida -= 1;
                            }
                            if juego.bunkers[b].vida == 0 {
                                juego.bunkers[b].activo = false;
                            }
                        }
                    }
                }
            }
        }

        // colision disparo alien vs bunker
        for i in 0..juego.num_disparos_alien {
            if juego.disparos_alien[i].activo {
                for b in 0..4 {
                    if juego.bunkers[b].activo {
                        let dx = (juego.disparos_alien[i].x - juego.bunkers[b].x).abs();
                        let dy = (juego.disparos_alien[i].y - juego.bunkers[b].y).abs();
                        if dx < 30.0 && dy < 20.0 {
                            juego.disparos_alien[i].activo = false;
                            if juego.bunkers[b].vida > 0 {
                                juego.bunkers[b].vida -= 1;
                            }
                            if juego.bunkers[b].vida == 0 {
                                juego.bunkers[b].activo = false;
                            }
                        }
                    }
                }
            }
        }

        // verificar si todos los aliens murieron -> siguiente nivel
        let mut aliens_vivos = 0;
        for i in 0..juego.num_aliens {
            if juego.aliens[i].vivo {
                aliens_vivos += 1;
            }
        }
        if aliens_vivos == 0 && juego.nivel < 3 {
            juego.nivel += 1;
            juego.velocidad_alien += 0.5;
            juego.num_aliens = 0;

            // nivel 1 = 4 filas, nivel 2 = 5 filas, nivel 3 = 6 filas
            let filas = 3 + juego.nivel as usize;
            for fila in 0..filas {
                for col in 0..10 {
                    let x = 80.0 + col as f32 * 65.0;
                    let y = 60.0 + fila as f32 * 50.0;
                    juego.agregar_alien(x, y);
                }
            }

            // aliens disparan mas rapido cada nivel
            intervalo_disparo_alien = 1.5 - (juego.nivel as f32 - 1.0) * 0.4;

            juego.limpiar_disparos_alien();
        }

        // --- DIBUJAR ---

        // dibujar jugador
        draw_texture_ex(
            &textura_jugador,
            juego.nave.x - 25.0,
            juego.nave.y - 25.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(50.0, 50.0)),
                ..Default::default()
            },
        );

        // dibujar aliens
        for i in 0..juego.num_aliens {
            if juego.aliens[i].vivo {
                draw_texture_ex(
                    &textura_enemigo,
                    juego.aliens[i].x - 20.0,
                    juego.aliens[i].y - 20.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(40.0, 40.0)),
                        ..Default::default()
                    },
                );
            }
        }

        // disparos jugador
        for i in 0..juego.num_disparos {
            if juego.disparos[i].activo {
                draw_rectangle(juego.disparos[i].x - 2.0, juego.disparos[i].y - 8.0, 4.0, 16.0, YELLOW);
            }
        }

        // disparos alien
        for i in 0..juego.num_disparos_alien {
            if juego.disparos_alien[i].activo {
                draw_rectangle(juego.disparos_alien[i].x - 2.0, juego.disparos_alien[i].y - 8.0, 4.0, 16.0, RED);
            }
        }

        // dibujar bunkers
        for b in 0..4 {
            if juego.bunkers[b].activo {
                let color = match juego.bunkers[b].vida {
                    20..=25 => GREEN,
                    10..=19 => YELLOW,
                    1..=9   => RED,
                    _       => WHITE,
                };
                let bx = juego.bunkers[b].x;
                let by = juego.bunkers[b].y;

                // fila superior completa
                draw_rectangle(bx - 30.0, by - 20.0, 60.0, 12.0, color);
                // fila del medio completa
                draw_rectangle(bx - 30.0, by - 8.0,  60.0, 12.0, color);
                // fila inferior: dos columnas laterales (hueco en el medio)
                draw_rectangle(bx - 30.0, by + 4.0,  20.0, 12.0, color);
                draw_rectangle(bx + 10.0, by + 4.0,  20.0, 12.0, color);
            }
        }

        // HUD
        draw_text(&format!("PUNTUACION: {}", juego.puntuacion), 10.0, 25.0, 24.0, WHITE);
        draw_text(&format!("VIDAS: {}", juego.nave.vidas), 650.0, 25.0, 24.0, WHITE);
        draw_text(&format!("NIVEL: {}", juego.nivel), 360.0, 25.0, 24.0, WHITE);

        // you win
        if juego.nivel >= 3 && aliens_vivos == 0 {
            draw_text("YOU WIN!", 280.0, 300.0, 60.0, GREEN);
            draw_text("presiona R para reiniciar", 240.0, 360.0, 24.0, WHITE);
            if is_key_pressed(KeyCode::R) {
                juego = Juego::new();
                intervalo_disparo_alien = 1.5;
                for fila in 0..4 {
                    for col in 0..10 {
                        let x = 80.0 + col as f32 * 65.0;
                        let y = 60.0 + fila as f32 * 50.0;
                        juego.agregar_alien(x, y);
                    }
                }
                juego.crear_bunkers();
            }
        }

        // game over
        if juego.nave.vidas == 0 {
            draw_text("GAME OVER", 280.0, 300.0, 60.0, RED);
            draw_text("presiona R para reiniciar", 240.0, 360.0, 24.0, WHITE);
            if is_key_pressed(KeyCode::R) {
                juego = Juego::new();
                intervalo_disparo_alien = 1.5;
                for fila in 0..4 {
                    for col in 0..10 {
                        let x = 80.0 + col as f32 * 65.0;
                        let y = 60.0 + fila as f32 * 50.0;
                        juego.agregar_alien(x, y);
                    }
                }
                juego.crear_bunkers();
            }
        }

        next_frame().await;
    }
}