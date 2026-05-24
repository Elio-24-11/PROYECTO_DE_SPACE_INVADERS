const MAX_ALIENS: usize = 50;
const MAX_DISPAROS: usize = 1024;

#[derive(Copy, Clone)]
pub struct Nave {
    pub x: f32,
    pub y: f32,
    pub vidas: u32,
    pub velocidad: f32,
}

#[derive(Copy, Clone)]
pub struct Alien {
    pub x: f32,
    pub y: f32,
    pub vivo: bool,
}

#[derive(Copy, Clone)]
pub struct Disparo {
    pub x: f32,
    pub y: f32,
    pub activo: bool,
    pub velocidad: f32,
}

#[derive(Copy, Clone)]
pub struct DisparoAlien {
    pub x: f32,
    pub y: f32,
    pub activo: bool,
    pub velocidad: f32,
}

#[derive(Copy, Clone)]
pub struct Bunker {
    pub x: f32,
    pub y: f32,
    pub vida: u32,
    pub activo: bool,
}

pub struct Juego {
    pub nave: Nave,
    pub aliens: [Alien; MAX_ALIENS],
    pub num_aliens: usize,
    pub disparos: [Disparo; MAX_DISPAROS],
    pub num_disparos: usize,
    pub disparos_alien: [DisparoAlien; MAX_DISPAROS],
    pub num_disparos_alien: usize,
    pub bunkers: [Bunker; 4],
    pub puntuacion: u32,
    pub direccion_alien: f32,
    pub velocidad_alien: f32,
    pub nivel: u32,
}

impl Nave {
    pub fn new() -> Nave {
        Nave {
            x: 400.0,
            y: 550.0,
            vidas: 3,
            velocidad: 5.0,
        }
    }

    pub fn mover_izquierda(&mut self) {
        if self.x > 20.0 {
            self.x -= self.velocidad;
        }
    }

    pub fn mover_derecha(&mut self) {
        if self.x < 780.0 {
            self.x += self.velocidad;
        }
    }
}

impl Alien {
    pub fn new(x: f32, y: f32) -> Alien {
        Alien { x, y, vivo: true }
    }
}

impl Disparo {
    pub fn new(x: f32, y: f32) -> Disparo {
        Disparo {
            x,
            y,
            activo: true,
            velocidad: 8.0,
        }
    }

    pub fn mover(&mut self) {
        self.y -= self.velocidad;
    }
}

impl DisparoAlien {
    pub fn new(x: f32, y: f32) -> DisparoAlien {
        DisparoAlien {
            x,
            y,
            activo: true,
            velocidad: 4.0,
        }
    }

    pub fn mover(&mut self) {
        self.y += self.velocidad;
    }
}

impl Bunker {
    pub fn new(x: f32, y: f32) -> Bunker {
        Bunker {
            x,
            y,
            vida: 25,
            activo: true,
        }
    }
}

impl Juego {
    pub fn new() -> Juego {
        Juego {
            nave: Nave::new(),
            aliens: [Alien {
                x: 0.0,
                y: 0.0,
                vivo: false,
            }; MAX_ALIENS],
            num_aliens: 0,
            disparos: [Disparo {
                x: 0.0,
                y: 0.0,
                activo: false,
                velocidad: 8.0,
            }; MAX_DISPAROS],
            num_disparos: 0,
            disparos_alien: [DisparoAlien {
                x: 0.0,
                y: 0.0,
                activo: false,
                velocidad: 4.0,
            }; MAX_DISPAROS],
            num_disparos_alien: 0,
            bunkers: [Bunker {
                x: 0.0,
                y: 0.0,
                vida: 3,
                activo: true,
            }; 4],
            puntuacion: 0,
            direccion_alien: 1.0,
            velocidad_alien: 1.0,
            nivel: 1,
        }
    }

    pub fn agregar_alien(&mut self, x: f32, y: f32) {
        if self.num_aliens < MAX_ALIENS {
            self.aliens[self.num_aliens] = Alien::new(x, y);
            self.num_aliens += 1;
        }
    }

    pub fn agregar_disparo(&mut self) {
        for i in 0..MAX_DISPAROS {
            if !self.disparos[i].activo {
                self.disparos[i] = Disparo::new(self.nave.x, self.nave.y);
                if i >= self.num_disparos {
                    self.num_disparos = i + 1;
                }
                return;
            }
        }
    }

    pub fn crear_bunkers(&mut self) {
        self.bunkers[0] = Bunker::new(150.0, 450.0);
        self.bunkers[1] = Bunker::new(300.0, 450.0);
        self.bunkers[2] = Bunker::new(500.0, 450.0);
        self.bunkers[3] = Bunker::new(650.0, 450.0);
    }

    pub fn agregar_disparo_alien(&mut self, x: f32, y: f32) {
        for i in 0..MAX_DISPAROS {
            if !self.disparos_alien[i].activo {
                self.disparos_alien[i] = DisparoAlien::new(x, y);
                if i >= self.num_disparos_alien {
                    self.num_disparos_alien = i + 1;
                }
                return;
            }
        }
    }

    pub fn limpiar_disparos_alien(&mut self) {
        for i in 0..MAX_DISPAROS {
            self.disparos_alien[i].activo = false;
        }
        self.num_disparos_alien = 0;
    }

    pub fn mover_aliens(&mut self) {
        let mut toca_borde = false;

        for i in 0..self.num_aliens {
            if self.aliens[i].vivo {
                if self.aliens[i].x > 780.0 || self.aliens[i].x < 20.0 {
                    toca_borde = true;
                }
            }
        }

        if toca_borde {
            self.direccion_alien *= -1.0;
            for i in 0..self.num_aliens {
                if self.aliens[i].vivo {
                    self.aliens[i].y += 20.0;
                }
            }
        }

        for i in 0..self.num_aliens {
            if self.aliens[i].vivo {
                self.aliens[i].x += self.velocidad_alien * self.direccion_alien;
            }
        }
    }
}