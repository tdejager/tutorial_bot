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
#[derive(Serialize, Deserialize, Clone)]
pub struct World {
    pub data: Vec<Vec<Tile>>,
}

/// State of the world
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum WorldState {
    FoundFood,
    Searching,
}

/// Update of the world to be sent over the wire
#[derive(Serialize, Deserialize)]
pub struct WorldUpdate {
    pub world: World,
    pub world_state: WorldState,
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
    /// Create a custom non-random world
    fn custom(food: (usize, usize), robot: (usize, usize)) -> Self {
        let (food_y, food_x) = food;
        let (robot_y, robot_x) = robot;
        let mut data = vec![vec![Tile::Empty; WORLD_WIDTH]; WORLD_HEIGHT];
        data[food_y][food_x] = Tile::Food;
        data[robot_y][robot_x] = Tile::Robot;

        Self { data }
    }

    /// Move the robot in the world
    /// Following constraints are uphold
    ///     1. When moving out of bounds this is an error
    ///     2. When moving over the food the food is overwritten
    pub fn move_robot(&mut self, direction: RobotMovement) -> anyhow::Result<()> {
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
        self.data[old_y][old_x] = Tile::Empty;
        Ok(())
    }

    /// Check if we are still searching for food or it has been overwritten
    pub fn world_state(&self) -> WorldState {
        // Find the robot
        for rows in self.data.iter() {
            for tile in rows.iter() {
                if *tile == Tile::Food {
                    return WorldState::Searching;
                }
            }
        }

        WorldState::FoundFood
    }
}

// This is a way how things are tested in rust
// the cfg test makes sure it is only compiled during a `cargo test`
// the #[test] specifies the actual tests, note that these are run in parralel when possible
#[cfg(test)]
mod tests {
    use crate::WorldState;

    use super::World;

    #[test]
    fn test_world() {
        let mut world = World::custom((10, 10), (12, 12));
        world.move_robot(crate::RobotMovement::Up).unwrap();
        // TODO see if constraints uphold of move robot function
    }

    #[test]
    fn test_eat_food() {
        let mut world = World::custom((10, 10), (10, 11));
        world.move_robot(crate::RobotMovement::Up).unwrap();
        assert_eq!(world.world_state(), WorldState::Searching);
        world.move_robot(crate::RobotMovement::Left).unwrap();
        assert_eq!(world.world_state(), WorldState::FoundFood);
    }
}
