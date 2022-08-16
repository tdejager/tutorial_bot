use rand::Rng;
use serde::{Deserialize, Serialize};

pub mod debug;

const WORLD_HEIGHT: usize = 100;
const WORLD_WIDTH: usize = 100;

/// Tile in the world, can either be a robot empty or food
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum Tile {
    Robot,
    Food,
    Empty,
}

/// Dictates where the robot is in the world
#[derive(Serialize, Deserialize)]
pub struct World {
    data: Vec<Vec<Tile>>,
}

/// State of the world
#[derive(Serialize, Deserialize)]
pub enum WorldState {
    FoundFood,
    Searching,
}

/// Update of the world to be sent over the wire
#[derive(Serialize, Deserialize)]
pub struct WorldUpdate {
    world: World,
    world_state: WorldState,
}

#[derive(Serialize, Deserialize)]
pub enum Response {
    Ok,
    Error(String),
}

#[derive(Serialize, Deserialize)]
pub enum RobotMovement {
    Up,
    Left,
    Right,
    Down,
}

impl Default for World {
    fn default() -> Self {
        let mut data = vec![vec![Tile::Empty; WORLD_WIDTH]; WORLD_HEIGHT];

        let food_x = rand::thread_rng().gen_range(0..WORLD_WIDTH);
        let food_y = rand::thread_rng().gen_range(0..WORLD_HEIGHT);
        data[food_y][food_x] = Tile::Food;

        let robot_x = rand::thread_rng().gen_range(0..WORLD_WIDTH);
        let robot_y = rand::thread_rng().gen_range(0..WORLD_HEIGHT);
        data[robot_y][robot_x] = Tile::Robot;

        Self { data }
    }
}

impl World {
    /// Move the robot in the world
    fn move_robot(&mut self, direction: RobotMovement) -> anyhow::Result<()> {
        let mut coords = None;

        // Find the robot
        for (y, rows) in self.data.iter().enumerate() {
            for (x, tile) in rows.iter().enumerate() {
                if *tile == Tile::Robot {
                    coords = Some((y, x));
                }
            }
        }
        // Panic if the robot was not found in the world, which should not happen
        let (old_y, old_x) = coords.ok_or_else(|| anyhow::anyhow!("robot was not found"))?;

        // Update position if not out of bounds
        let (new_y, new_x): (usize, usize) = match direction {
            RobotMovement::Up => (old_y.wrapping_sub(1), old_x),
            RobotMovement::Left => (old_y, old_x.wrapping_sub(1)),
            RobotMovement::Right => (old_y, old_x + 1),
            RobotMovement::Down => (old_y + 1, old_x),
        };

        // Check bounds
        if new_y > WORLD_HEIGHT || new_x > WORLD_WIDTH {
            return Err(anyhow::anyhow!("out of bounds"));
        }

        // Update
        self.data[new_y][new_x] = Tile::Robot;
        self.data[old_x][old_y] = Tile::Empty;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::World;

    #[test]
    fn test_world() {
        let world = World::default();
    }
}
