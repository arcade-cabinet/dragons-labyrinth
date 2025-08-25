//! HBF transformation modules for progressive data extraction

pub mod empty_remover;
pub mod refs_extractor;
pub mod json_parser;
pub mod html_parser;
pub mod dungeon_parser;
pub mod progressive;
pub mod ai_progressive;

// Re-export the main transformer types  
pub use progressive::{HbfTransformer, TransformReport};
pub use ai_progressive::{AiHbfTransformer, TransformReport as AiTransformReport};

// Re-export individual parsers for direct use
pub use empty_remover::{EmptyRemover, EmptyRemovalStats};
pub use refs_extractor::{RefsExtractor, RefsExtractionResult, RefEntry, RefType};
pub use json_parser::{JsonParser, JsonParsingResult, MapData, HexTile, TerrainType};
pub use html_parser::{HtmlParser, HtmlParsingResult, HtmlContent, ContentType as HtmlContentType};
pub use dungeon_parser::{DungeonParser, DungeonParsingResult, DungeonData, DungeonTheme};
