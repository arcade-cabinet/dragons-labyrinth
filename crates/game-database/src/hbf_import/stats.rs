//! Import statistics tracking for HBF conversion process

#[derive(Debug, Default)]
pub struct ImportStats {
    pub hex_tiles: usize,
    pub settlements: usize,
    pub dungeons: usize,
    pub dungeon_rooms: usize,
    pub dungeon_doorways: usize,
    pub npcs: usize,
    pub weather_systems: usize,
    pub encounters: usize,
    pub errors: usize,
    pub warnings: usize,
}

impl ImportStats {
    pub fn total_imported(&self) -> usize {
        self.hex_tiles + self.settlements + self.dungeons + self.npcs
    }
    
    pub fn has_errors(&self) -> bool {
        self.errors > 0
    }
    
    pub fn success_rate(&self) -> f32 {
        let total_attempts = self.total_imported() + self.errors;
        if total_attempts == 0 {
            1.0
        } else {
            self.total_imported() as f32 / total_attempts as f32
        }
    }
}
