use bevy::prelude::*;
use mapgen::*;
use rand::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
use super::hex::HexCoord;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MazeCoord {
    pub x: i32,
    pub y: i32,
}

impl MazeCoord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    
    pub fn neighbors(&self) -> Vec<MazeCoord> {
        vec![
            MazeCoord::new(self.x, self.y - 1), // North
            MazeCoord::new(self.x + 1, self.y), // East
            MazeCoord::new(self.x, self.y + 1), // South
            MazeCoord::new(self.x - 1, self.y), // West
        ]
    }
}

#[derive(Debug, Clone)]
pub struct Maze {
    pub width: i32,
    pub height: i32,
    pub walls: HashSet<(MazeCoord, MazeCoord)>, // Walls between cells
    pub cells: HashSet<MazeCoord>, // Valid maze cells
    pub start: MazeCoord,
    pub end: MazeCoord,
}

impl Maze {
    pub fn new(width: i32, height: i32) -> Self {
        let mut cells = HashSet::new();
        for x in 0..width {
            for y in 0..height {
                cells.insert(MazeCoord::new(x, y));
            }
        }
        
        Self {
            width,
            height,
            walls: HashSet::new(),
            cells,
            start: MazeCoord::new(0, 0),
            end: MazeCoord::new(width - 1, height - 1),
        }
    }
    
    pub fn is_valid_coord(&self, coord: MazeCoord) -> bool {
        coord.x >= 0 && coord.x < self.width && coord.y >= 0 && coord.y < self.height
    }
    
    pub fn has_wall(&self, from: MazeCoord, to: MazeCoord) -> bool {
        self.walls.contains(&(from, to)) || self.walls.contains(&(to, from))
    }
    
    pub fn add_wall(&mut self, from: MazeCoord, to: MazeCoord) {
        self.walls.insert((from, to));
    }
    
    pub fn remove_wall(&mut self, from: MazeCoord, to: MazeCoord) {
        self.walls.remove(&(from, to));
        self.walls.remove(&(to, from));
    }
    
    pub fn get_accessible_neighbors(&self, coord: MazeCoord) -> Vec<MazeCoord> {
        coord.neighbors()
            .into_iter()
            .filter(|&neighbor| {
                self.is_valid_coord(neighbor) && !self.has_wall(coord, neighbor)
            })
            .collect()
    }
    
    pub fn find_path(&self, start: MazeCoord, end: MazeCoord) -> Option<Vec<MazeCoord>> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut parent = HashMap::new();
        
        queue.push_back(start);
        visited.insert(start);
        
        while let Some(current) = queue.pop_front() {
            if current == end {
                // Reconstruct path
                let mut path = Vec::new();
                let mut node = end;
                path.push(node);
                
                while let Some(&p) = parent.get(&node) {
                    path.push(p);
                    node = p;
                }
                
                path.reverse();
                return Some(path);
            }
            
            for neighbor in self.get_accessible_neighbors(current) {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    parent.insert(neighbor, current);
                    queue.push_back(neighbor);
                }
            }
        }
        
        None // No path found
    }
}

pub trait MazeGenerator {
    fn generate(&self, maze: &mut Maze, rng: &mut impl Rng);
}

pub struct RecursiveBacktracker;

impl MazeGenerator for RecursiveBacktracker {
    fn generate(&self, maze: &mut Maze, rng: &mut impl Rng) {
        // Start with all walls
        for x in 0..maze.width {
            for y in 0..maze.height {
                let current = MazeCoord::new(x, y);
                for neighbor in current.neighbors() {
                    if maze.is_valid_coord(neighbor) {
                        maze.add_wall(current, neighbor);
                    }
                }
            }
        }
        
        let mut stack = Vec::new();
        let mut visited = HashSet::new();
        
        let start = maze.start;
        stack.push(start);
        visited.insert(start);
        
        while let Some(current) = stack.last().copied() {
            let unvisited_neighbors: Vec<_> = current.neighbors()
                .into_iter()
                .filter(|&n| maze.is_valid_coord(n) && !visited.contains(&n))
                .collect();
            
            if !unvisited_neighbors.is_empty() {
                let next = unvisited_neighbors[rng.gen_range(0..unvisited_neighbors.len())];
                maze.remove_wall(current, next);
                visited.insert(next);
                stack.push(next);
            } else {
                stack.pop();
            }
        }
    }
}

pub struct Kruskals;

impl MazeGenerator for Kruskals {
    fn generate(&self, maze: &mut Maze, rng: &mut impl Rng) {
        // Start with all walls
        let mut edges = Vec::new();
        for x in 0..maze.width {
            for y in 0..maze.height {
                let current = MazeCoord::new(x, y);
                for neighbor in current.neighbors() {
                    if maze.is_valid_coord(neighbor) && current.x <= neighbor.x && current.y <= neighbor.y {
                        edges.push((current, neighbor));
                        maze.add_wall(current, neighbor);
                    }
                }
            }
        }
        
        edges.shuffle(rng);
        
        // Union-Find for connected components
        let mut parent = HashMap::new();
        for x in 0..maze.width {
            for y in 0..maze.height {
                let coord = MazeCoord::new(x, y);
                parent.insert(coord, coord);
            }
        }
        
        fn find(parent: &mut HashMap<MazeCoord, MazeCoord>, mut x: MazeCoord) -> MazeCoord {
            while parent[&x] != x {
                let next = parent[&x];
                parent.insert(x, parent[&next]);
                x = next;
            }
            x
        }
        
        for (a, b) in edges {
            let root_a = find(&mut parent, a);
            let root_b = find(&mut parent, b);
            
            if root_a != root_b {
                parent.insert(root_a, root_b);
                maze.remove_wall(a, b);
            }
        }
    }
}

pub struct Wilsons;

impl MazeGenerator for Wilsons {
    fn generate(&self, maze: &mut Maze, rng: &mut impl Rng) {
        // Start with all walls
        for x in 0..maze.width {
            for y in 0..maze.height {
                let current = MazeCoord::new(x, y);
                for neighbor in current.neighbors() {
                    if maze.is_valid_coord(neighbor) {
                        maze.add_wall(current, neighbor);
                    }
                }
            }
        }
        
        let mut in_maze = HashSet::new();
        let mut remaining: Vec<_> = maze.cells.iter().copied().collect();
        
        // Start with a random cell in the maze
        let initial = remaining[rng.gen_range(0..remaining.len())];
        in_maze.insert(initial);
        remaining.retain(|&x| x != initial);
        
        while !remaining.is_empty() {
            // Pick a random cell not in maze
            let start = remaining[rng.gen_range(0..remaining.len())];
            let mut path = vec![start];
            let mut current = start;
            
            // Random walk until we hit the maze
            while !in_maze.contains(&current) {
                let neighbors: Vec<_> = current.neighbors()
                    .into_iter()
                    .filter(|&n| maze.is_valid_coord(n))
                    .collect();
                
                if !neighbors.is_empty() {
                    let next = neighbors[rng.gen_range(0..neighbors.len())];
                    
                    // If we've visited this cell before in this walk, loop-erase
                    if let Some(pos) = path.iter().position(|&x| x == next) {
                        path.truncate(pos + 1);
                    } else {
                        path.push(next);
                    }
                    current = next;
                }
            }
            
            // Add the path to the maze
            for window in path.windows(2) {
                maze.remove_wall(window[0], window[1]);
            }
            
            // Mark all cells in path as part of maze
            for &cell in &path {
                if !in_maze.contains(&cell) {
                    in_maze.insert(cell);
                    remaining.retain(|&x| x != cell);
                }
            }
        }
    }
}

pub struct NonEuclidean;

impl MazeGenerator for NonEuclidean {
    fn generate(&self, maze: &mut Maze, rng: &mut impl Rng) {
        // Generate a base maze using recursive backtracker
        RecursiveBacktracker.generate(maze, rng);
        
        // Add non-euclidean connections (portals/wrapping)
        let portal_count = (maze.width * maze.height) / 20; // 5% of cells get portals
        
        for _ in 0..portal_count {
            let x1 = rng.gen_range(0..maze.width);
            let y1 = rng.gen_range(0..maze.height);
            let x2 = rng.gen_range(0..maze.width);
            let y2 = rng.gen_range(0..maze.height);
            
            let cell1 = MazeCoord::new(x1, y1);
            let cell2 = MazeCoord::new(x2, y2);
            
            // Create a "portal" connection by removing the conceptual wall
            // This would be handled differently in actual 3D rendering
            if cell1 != cell2 {
                maze.remove_wall(cell1, cell2);
            }
        }
        
        // Add some impossible geometry (stairs that loop, etc.)
        // This would be implemented in the 3D rendering system
    }
}

pub fn generate_maze_for_dread_level(
    width: i32,
    height: i32,
    dread_level: f32,
    rng: &mut StdRng,
) -> Maze {
    let mut maze = Maze::new(width, height);
    
    // Use mapgen algorithms based on dread level
    let map = match dread_level as i32 {
        0..=20 => {
            // Simple, peaceful mazes
            MapBuilder::new(width as usize, height as usize)
                .with(SimpleRooms::new())
                .with(NearestCorridors::new())
                .with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER))
                .build_map(rng)
        }
        21..=40 => {
            // BSP dungeon with increasing complexity
            MapBuilder::new(width as usize, height as usize)
                .with(BspDungeonBuilder::new())
                .with(AreaStartingPosition::new(XStart::LEFT, YStart::CENTER))
                .with(CullUnreachable::new())
                .build_map(rng)
        }
        41..=60 => {
            // Cellular automata for organic, unsettling layouts
            MapBuilder::new(width as usize, height as usize)
                .with(CellularAutomataBuilder::new())
                .with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER))
                .with(CullUnreachable::new())
                .with(VoronoiSpawning::new())
                .build_map(rng)
        }
        61..=80 => {
            // Diffusion limited aggregation for chaotic structures
            MapBuilder::new(width as usize, height as usize)
                .with(DiffusionLimitedAggregationBuilder::new())
                .with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER))
                .with(CullUnreachable::new())
                .with(VoronoiSpawning::new())
                .build_map(rng)
        }
        _ => {
            // Maze with prefab nightmare rooms
            MapBuilder::new(width as usize, height as usize)
                .with(MazeBuilder::new())
                .with(PrefabBuilder::sectional())
                .with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER))
                .with(CullUnreachable::new())
                .with(VoronoiSpawning::new())
                .build_map(rng)
        }
    };
    
    // Convert mapgen result to our maze format
    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) as usize;
            if idx < map.tiles.len() && map.tiles[idx] == TileType::Wall {
                let current = MazeCoord::new(x, y);
                for neighbor in current.neighbors() {
                    if maze.is_valid_coord(neighbor) {
                        maze.add_wall(current, neighbor);
                    }
                }
            }
        }
    }
    
    maze
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maze_creation() {
        let maze = Maze::new(10, 10);
        assert_eq!(maze.width, 10);
        assert_eq!(maze.height, 10);
        assert_eq!(maze.cells.len(), 100);
    }

    #[test]
    fn test_recursive_backtracker() {
        let mut maze = Maze::new(5, 5);
        let mut rng = thread_rng();
        RecursiveBacktracker.generate(&mut maze, &mut rng);
        
        // Should have a path from start to end
        assert!(maze.find_path(maze.start, maze.end).is_some());
    }
}
