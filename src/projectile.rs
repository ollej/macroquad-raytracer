use crate::prelude::*;

pub fn generate_trajectory() -> Canvas {
    let start = point(0.0, 1.0, 0.0);
    let velocity = vector(1.0, 1.8, 0.0).normalize() * 11.25;
    let mut p = Projectile::new(start, velocity);
    let gravity = vector(0.0, -0.1, 0.0);
    let wind = vector(-0.01, 0.0, 0.0);
    let e = Environment::new(gravity, wind);
    let mut c = canvas(900, 550);
    let red = color(1.0, 0.0, 0.0);

    while !p.has_landed() {
        p = e.tick(p);
        let x = p.position.x.round() as usize;
        let y = p.position.y.round() as usize;
        let ypos = c.height - y - 1;
        //println!("x: {}, y: {} yprim: {}", x, y, yprim);
        c.write_pixel(x, ypos, &red);
    }

    c
}

#[derive(Debug)]
pub struct Projectile {
    position: Point,
    velocity: Vector,
}

impl Projectile {
    pub fn new(position: Point, velocity: Vector) -> Self {
        Self { position, velocity }
    }

    pub fn has_landed(&self) -> bool {
        self.position.y <= 0.0
    }
}

#[derive(Debug)]
pub struct Environment {
    gravity: Vector,
    wind: Vector,
}

impl Environment {
    pub fn new(gravity: Vector, wind: Vector) -> Self {
        Self { gravity, wind }
    }

    pub fn tick(&self, projectile: Projectile) -> Projectile {
        let position = projectile.position + &projectile.velocity;
        let velocity = projectile.velocity + &self.gravity + &self.wind;
        return Projectile::new(position, velocity);
    }
}

#[cfg(test)]
mod test_chapter_1_maths {
    use super::*;

    #[test]
    fn shooting_a_projectile() {
        // projectile starts one unit above the origin.
        // velocity is normalized to 1 unit/tick.
        let mut p = Projectile::new(point(0.0, 1.0, 0.0), vector(1.0, 1.0, 0.0).normalize());
        // gravity -0.1 unit/tick, and wind is -0.01 unit/tick.
        let e = Environment::new(vector(0.0, -0.1, 0.0), vector(-0.01, 0.0, 0.0));
        let mut ticks = 0;
        while !p.has_landed() {
            println!("#{p:?}");
            p = e.tick(p);
            ticks = ticks + 1;
        }
        println!("Reached ground after #{ticks} ticks.");
        assert_eq!(ticks, 17);
    }
}
