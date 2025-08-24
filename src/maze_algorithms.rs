//! Maze generation algorithms for 3D labyrinths
//! Each progression level gets more complex and horrifying

use bevy::prelude::*;
use rand::prelude::*;
use std::collections::{HashSet, VecDeque};

/// A 3D labyrinth that gets more horrifying with progression
#[derive(Component)]
pub struct Labyrinth3D {
    pub layout: Vec<Vec<Cell>>,
    pub width: usize,
    pub height: usize,
    pub complexity: LabyrinthComplexity,
    pub boss_room: IVec2,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Wall,
    Path,
    BossRoom,
    Entrance,
    TrapRoom,
    TreasureRoom,
    VoidRift,  // Late game: reality breaks here
}

#[derive(Clone, Copy)]
pub enum LabyrinthComplexity {
    Simple,      // L1-20: Linear, few branches
    Branching,   // L21-40: Multiple paths
    Complex,     // L41-60: Maze-like
    Shifting,    // L61-80: Walls move
    Corrupted,   // L81-100: Geometry wrong
    Void,        // L101-120: Non-Euclidean
    Nightmare,   // L121-140: Reality unstable
    Truth,       // L141-160: See through walls
    Final,       // L161-180: You ARE the maze
}

impl Labyrinth3D {
    /// Generate a labyrinth based on progression
    pub fn generate(progression: u32) -> Self {
        let complexity = match progression {
            1..=20 => LabyrinthComplexity::Simple,
            21..=40 => LabyrinthComplexity::Branching,
            41..=60 => LabyrinthComplexity::Complex,
            61..=80 => LabyrinthComplexity::Shifting,
            81..=100 => LabyrinthComplexity::Corrupted,
            101..=120 => LabyrinthComplexity::Void,
            121..=140 => LabyrinthComplexity::Nightmare,
            141..=160 => LabyrinthComplexity::Truth,
            161..=180 => LabyrinthComplexity::Final,
            _ => LabyrinthComplexity::Simple,
        };
        
        // Size scales with progression
        let size = 10 + (progression / 10) * 5;
        let width = size.min(50);
        let height = size.min(50);
        
        let mut labyrinth = match complexity {
            LabyrinthComplexity::Simple => Self::recursive_backtracker(width, height),
            LabyrinthComplexity::Branching => Self::kruskal(width, height),
            LabyrinthComplexity::Complex => Self::wilson(width, height),
            LabyrinthComplexity::Shifting => Self::eller(width, height),
            LabyrinthComplexity::Corrupted => Self::corrupted_maze(width, height),
            LabyrinthComplexity::Void => Self::non_euclidean(width, height),
            LabyrinthComplexity::Nightmare => Self::nightmare_maze(width, height),
            LabyrinthComplexity::Truth => Self::truth_maze(width, height),
            LabyrinthComplexity::Final => Self::final_maze(width, height),
        };
        
        labyrinth.complexity = complexity;
        labyrinth.add_boss_room();
        labyrinth.add_special_rooms(progression);
        
        labyrinth
    }
    
    /// Recursive Backtracker - Creates long, winding paths (Simple)
    fn recursive_backtracker(width: usize, height: usize) -> Self {
        let mut layout = vec![vec![Cell::Wall; width]; height];
        let mut rng = thread_rng();
        let mut stack = vec![IVec2::new(1, 1)];
        let mut visited = HashSet::new();
        
        layout[1][1] = Cell::Entrance;
        visited.insert(IVec2::new(1, 1));
        
        while let Some(current) = stack.last().cloned() {
            let neighbors = Self::get_unvisited_neighbors(current, &visited, width, height);
            
            if neighbors.is_empty() {
                stack.pop();
            } else {
                let next = neighbors.choose(&mut rng).unwrap();
                
                // Carve path between current and next
                let between = IVec2::new(
                    (current.x + next.x) / 2,
                    (current.y + next.y) / 2,
                );
                
                layout[current.y as usize][current.x as usize] = Cell::Path;
                layout[between.y as usize][between.x as usize] = Cell::Path;
                layout[next.y as usize][next.x as usize] = Cell::Path;
                
                visited.insert(*next);
                stack.push(*next);
            }
        }
        
        Self {
            layout,
            width,
            height,
            complexity: LabyrinthComplexity::Simple,
            boss_room: IVec2::new(width as i32 - 2, height as i32 - 2),
        }
    }
    
    /// Kruskal's Algorithm - Creates more open mazes (Branching)
    fn kruskal(width: usize, height: usize) -> Self {
        let mut layout = vec![vec![Cell::Wall; width]; height];
        let mut rng = thread_rng();
        let mut sets: Vec<Vec<usize>> = vec![vec![0; width]; height];
        let mut set_id = 0;
        
        // Initialize cells and sets
        for y in (1..height).step_by(2) {
            for x in (1..width).step_by(2) {
                layout[y][x] = Cell::Path;
                sets[y][x] = set_id;
                set_id += 1;
            }
        }
        
        // Create list of walls
        let mut walls = Vec::new();
        for y in (1..height).step_by(2) {
            for x in (1..width).step_by(2) {
                if x + 2 < width {
                    walls.push((IVec2::new(x as i32, y as i32), IVec2::new(x as i32 + 2, y as i32)));
                }
                if y + 2 < height {
                    walls.push((IVec2::new(x as i32, y as i32), IVec2::new(x as i32, y as i32 + 2)));
                }
            }
        }
        
        walls.shuffle(&mut rng);
        
        // Remove walls to connect different sets
        for (cell1, cell2) in walls {
            let set1 = sets[cell1.y as usize][cell1.x as usize];
            let set2 = sets[cell2.y as usize][cell2.x as usize];
            
            if set1 != set2 {
                // Connect the cells
                let wall_x = (cell1.x + cell2.x) / 2;
                let wall_y = (cell1.y + cell2.y) / 2;
                layout[wall_y as usize][wall_x as usize] = Cell::Path;
                
                // Merge sets
                let old_set = sets[cell2.y as usize][cell2.x as usize];
                for row in &mut sets {
                    for cell in row {
                        if *cell == old_set {
                            *cell = set1;
                        }
                    }
                }
            }
        }
        
        layout[1][1] = Cell::Entrance;
        
        Self {
            layout,
            width,
            height,
            complexity: LabyrinthComplexity::Branching,
            boss_room: IVec2::new(width as i32 - 2, height as i32 - 2),
        }
    }
    
    /// Wilson's Algorithm - Uniform random maze (Complex)
    fn wilson(width: usize, height: usize) -> Self {
        let mut layout = vec![vec![Cell::Wall; width]; height];
        let mut rng = thread_rng();
        let mut in_maze = HashSet::new();
        
        // Start with a random cell
        let start = IVec2::new(1, 1);
        in_maze.insert(start);
        layout[1][1] = Cell::Entrance;
        
        // Get all cells
        let mut remaining = Vec::new();
        for y in (1..height).step_by(2) {
            for x in (1..width).step_by(2) {
                let pos = IVec2::new(x as i32, y as i32);
                if pos != start {
                    remaining.push(pos);
                }
            }
        }
        
        while !remaining.is_empty() {
            // Pick random cell not in maze
            let start_idx = rng.gen_range(0..remaining.len());
            let walker_start = remaining[start_idx];
            
            // Random walk until we hit the maze
            let mut path = vec![walker_start];
            let mut current = walker_start;
            
            while !in_maze.contains(&current) {
                let neighbors = Self::get_valid_neighbors(current, width, height);
                let next = *neighbors.choose(&mut rng).unwrap();
                
                // Loop-erase: remove cycles
                if let Some(pos) = path.iter().position(|&p| p == next) {
                    path.truncate(pos + 1);
                } else {
                    path.push(next);
                }
                current = next;
            }
            
            // Add path to maze
            for window in path.windows(2) {
                let from = window[0];
                let to = window[1];
                let between = IVec2::new((from.x + to.x) / 2, (from.y + to.y) / 2);
                
                layout[from.y as usize][from.x as usize] = Cell::Path;
                layout[between.y as usize][between.x as usize] = Cell::Path;
                in_maze.insert(from);
            }
            
            // Remove added cells from remaining
            remaining.retain(|p| !in_maze.contains(p));
        }
        
        Self {
            layout,
            width,
            height,
            complexity: LabyrinthComplexity::Complex,
            boss_room: IVec2::new(width as i32 - 2, height as i32 - 2),
        }
    }
    
    /// Eller's Algorithm - Row by row generation (Shifting)
    fn eller(width: usize, height: usize) -> Self {
        // Simplified Eller's for shifting mazes
        let mut layout = Self::recursive_backtracker(width, height).layout;
        
        // Add shifting walls (some walls that appear/disappear)
        let mut rng = thread_rng();
        for y in 1..height-1 {
            for x in 1..width-1 {
                if layout[y][x] == Cell::Wall && rng.gen_bool(0.1) {
                    // This wall shifts
                    if rng.gen_bool(0.5) {
                        layout[y][x] = Cell::Path;  // Currently open
                    }
                }
            }
        }
        
        Self {
            layout,
            width,
            height,
            complexity: LabyrinthComplexity::Shifting,
            boss_room: IVec2::new(width as i32 - 2, height as i32 - 2),
        }
    }
    
    /// Corrupted Maze - Geometry doesn't make sense
    fn corrupted_maze(width: usize, height: usize) -> Self {
        let mut base = Self::wilson(width, height);
        let mut rng = thread_rng();
        
        // Add impossible geometry
        for y in 0..height {
            for x in 0..width {
                if base.layout[y][x] == Cell::Path && rng.gen_bool(0.05) {
                    base.layout[y][x] = Cell::VoidRift;
                }
            }
        }
        
        base.complexity = LabyrinthComplexity::Corrupted;
        base
    }
    
    /// Non-Euclidean - Portals and wraparounds
    fn non_euclidean(width: usize, height: usize) -> Self {
        let mut base = Self::kruskal(width, height);
        
        // Paths can loop back on themselves
        // Portals connect distant parts
        // This would need special rendering
        
        base.complexity = LabyrinthComplexity::Void;
        base
    }
    
    /// Nightmare - Walls change as you move
    fn nightmare_maze(width: usize, height: usize) -> Self {
        // Start with no walls, they appear based on player position
        let mut layout = vec![vec![Cell::Path; width]; height];
        
        // Some areas are always walls
        for i in 0..width {
            layout[0][i] = Cell::Wall;
            layout[height-1][i] = Cell::Wall;
        }
        for i in 0..height {
            layout[i][0] = Cell::Wall;
            layout[i][width-1] = Cell::Wall;
        }
        
        Self {
            layout,
            width,
            height,
            complexity: LabyrinthComplexity::Nightmare,
            boss_room: IVec2::new(width as i32 / 2, height as i32 / 2),
        }
    }
    
    /// Truth - You can see through walls but not walk through them
    fn truth_maze(width: usize, height: usize) -> Self {
        let mut base = Self::recursive_backtracker(width, height);
        base.complexity = LabyrinthComplexity::Truth;
        base
    }
    
    /// Final - You ARE the maze
    fn final_maze(width: usize, height: usize) -> Self {
        // The maze is your own mind
        // Paths form based on your choices
        let mut layout = vec![vec![Cell::VoidRift; width]; height];
        
        // Only the path you take exists
        layout[height/2][width/2] = Cell::Path;
        
        Self {
            layout,
            width,
            height,
            complexity: LabyrinthComplexity::Final,
            boss_room: IVec2::new(width as i32 / 2, height as i32 / 2),
        }
    }
    
    fn get_unvisited_neighbors(pos: IVec2, visited: &HashSet<IVec2>, width: usize, height: usize) -> Vec<IVec2> {
        let mut neighbors = Vec::new();
        
        for dir in [IVec2::new(0, -2), IVec2::new(2, 0), IVec2::new(0, 2), IVec2::new(-2, 0)] {
            let next = pos + dir;
            if next.x > 0 && next.x < width as i32 - 1 &&
               next.y > 0 && next.y < height as i32 - 1 &&
               !visited.contains(&next) {
                neighbors.push(next);
            }
        }
        
        neighbors
    }
    
    fn get_valid_neighbors(pos: IVec2, width: usize, height: usize) -> Vec<IVec2> {
        let mut neighbors = Vec::new();
        
        for dir in [IVec2::new(0, -2), IVec2::new(2, 0), IVec2::new(0, 2), IVec2::new(-2, 0)] {
            let next = pos + dir;
            if next.x > 0 && next.x < width as i32 - 1 &&
               next.y > 0 && next.y < height as i32 - 1 {
                neighbors.push(next);
            }
        }
        
        neighbors
    }
    
    fn add_boss_room(&mut self) {
        // Boss room is always at the end
        let boss_x = self.width - 5;
        let boss_y = self.height - 5;
        
        // Clear 3x3 area for boss
        for y in boss_y..boss_y.min(self.height).min(boss_y + 3) {
            for x in boss_x..boss_x.min(self.width).min(boss_x + 3) {
                self.layout[y][x] = Cell::BossRoom;
            }
        }
        
        self.boss_room = IVec2::new(boss_x as i32 + 1, boss_y as i32 + 1);
    }
    
    fn add_special_rooms(&mut self, progression: u32) {
        let mut rng = thread_rng();
        let num_special = 1 + (progression / 40);
        
        for _ in 0..num_special {
            let x = rng.gen_range(3..self.width-3);
            let y = rng.gen_range(3..self.height-3);
            
            if self.layout[y][x] == Cell::Path {
                self.layout[y][x] = if rng.gen_bool(0.7) {
                    Cell::TreasureRoom
                } else {
                    Cell::TrapRoom
                };
            }
        }
    }
    
    /// Render the labyrinth in 3D
    pub fn spawn_3d_maze(
        &self,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) {
        for (y, row) in self.layout.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let pos = Vec3::new(x as f32 * 2.0, 0.0, y as f32 * 2.0);
                
                match cell {
                    Cell::Wall => {
                        commands.spawn(PbrBundle {
                            mesh: meshes.add(Cuboid::new(2.0, 4.0, 2.0)),
                            material: materials.add(StandardMaterial {
                                base_color: Color::srgb(0.3, 0.3, 0.3),
                                ..default()
                            }),
                            transform: Transform::from_translation(pos + Vec3::Y * 2.0),
                            ..default()
                        });
                    },
                    Cell::VoidRift => {
                        commands.spawn(PbrBundle {
                            mesh: meshes.add(Cuboid::new(2.0, 0.1, 2.0)),
                            material: materials.add(StandardMaterial {
                                base_color: Color::srgb(0.0, 0.0, 0.0),
                                emissive: LinearRgba::new(0.5, 0.0, 0.5, 1.0),
                                ..default()
                            }),
                            transform: Transform::from_translation(pos),
                            ..default()
                        });
                    },
                    _ => {
                        // Floor tile
                        commands.spawn(PbrBundle {
                            mesh: meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(1.0))),
                            material: materials.add(StandardMaterial {
                                base_color: match cell {
                                    Cell::BossRoom => Color::srgb(0.8, 0.2, 0.2),
                                    Cell::TreasureRoom => Color::srgb(0.8, 0.7, 0.2),
                                    Cell::TrapRoom => Color::srgb(0.5, 0.2, 0.2),
                                    Cell::Entrance => Color::srgb(0.2, 0.8, 0.2),
                                    _ => Color::srgb(0.5, 0.5, 0.5),
                                },
                                ..default()
                            }),
                            transform: Transform::from_translation(pos),
                            ..default()
                        });
                    }
                }
            }
        }
    }
}
