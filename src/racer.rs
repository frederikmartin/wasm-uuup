#[derive(Debug, Clone, PartialEq)]
pub struct Position(pub i32, pub i32);

#[derive(Debug, Clone)]
pub struct Ship {
    pub position: Position
}

impl Ship {
    pub fn new(x: i32, y: i32) -> Self {
        Self { position: Position(x, y) }
    }

    pub fn jump(position: Position) -> Self {
        Self { position }
    }

    pub fn jump_random() -> Self {
        let x_random = (rand::random::<f64>() * 10.0).floor() as i32;
        let y_random = (rand::random::<f64>() * 10.0).floor() as i32;
        Self::jump(Position(x_random, y_random))
    }
}

#[derive(Debug, Clone)]
pub struct Racer {
    pub width: u32,
    pub height: u32,
    pub ship: Ship
}

impl Racer {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height, ship: Ship::new((width as i32) / 2, (height as i32) / 2) }
    }

    pub fn tick(&mut self) {
        self.ship = Ship::jump_random()
    }
}

#[cfg(test)]
mod tests {
    use super::Racer;

    #[test]
    fn test() {
        let racer = Racer::new(10, 10);
        println!("{:#?}", racer)
    }
}

