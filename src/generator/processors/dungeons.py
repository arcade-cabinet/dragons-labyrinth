"""
DungeonsProcessor - HTML processor for dungeon entity clusters.

Processes dungeon clusters from the analysis phase, extracting encounter
information, monster stats, treasure data, and environmental descriptions
to create Rust ECS components.
"""

from __future__ import annotations

import json
import re
from pathlib import Path
from typing import Any, Dict, List, Optional

from bs4 import BeautifulSoup

from generator.processors.base import BaseProcessor
from generator.processors.models import DungeonData, AreaData, MonsterData, TreasureData, ProcessingResult


class DungeonsProcessor(BaseProcessor):
    """
    Specialized processor for dungeon entity clusters.
    
    Extracts data from:
    - HTML files containing dungeon area descriptions
    - Monster stat blocks with combat information
    - Treasure and loot data
    - Environmental descriptions and encounters
    
    Generates UUID-based Rust ECS structure:
    - processed/dungeons/{dungeon_uuid}/mod.rs  
    - processed/dungeons/{dungeon_uuid}/{area_uuid}.json
    """
    
    def __init__(self):
        super().__init__("dungeons")
    
    def process_cluster_directory(self, analysis_dir: Path, output_dir: Path, logger, console) -> ProcessingResult:
        """Process a dungeon cluster directory containing HTML area files."""
        
        dungeon_name = analysis_dir.name
        console.print(f"Processing dungeon: [bold red]{dungeon_name}[/bold red]")
        
        # Find all HTML area files
        html_files = list(analysis_dir.glob("entity_*.html"))
        
        if not html_files:
            logger.warning(f"No HTML files found in {analysis_dir}")
            return self._create_empty_result(dungeon_name)
        
        # Process each area file
        areas = []
        for html_file in html_files:
            area_uuid = html_file.stem.replace("entity_", "")
            area_data = self._process_area_file(html_file, area_uuid)
            if area_data:
                areas.append(area_data)
        
        # Extract dungeon metadata
        dungeon_data = DungeonData(
            name=dungeon_name,
            uuid=self._generate_dungeon_uuid(dungeon_name),
            total_areas=len(areas),
            monster_count=sum(len(area.monsters) for area in areas),
            treasure_count=sum(len(area.treasures) for area in areas),
            difficulty_level=self._calculate_difficulty(areas),
            areas=areas
        )
        
        # Create output structure
        dungeon_output_dir = output_dir / "dungeons" / dungeon_data.uuid
        dungeon_output_dir.mkdir(parents=True, exist_ok=True)
        
        # Generate area JSON files
        for area in areas:
            area_file = dungeon_output_dir / f"{area.uuid}.json"
            with open(area_file, 'w', encoding='utf-8') as f:
                json.dump(area.to_dict(), f, indent=2)
        
        # Generate mod.rs file
        mod_file = dungeon_output_dir / "mod.rs"
        self._generate_rust_module(mod_file, dungeon_data)
        
        logger.info(f"Processed dungeon {dungeon_name}: {len(areas)} areas, {dungeon_data.monster_count} monsters")
        
        return ProcessingResult(
            entity_type="dungeon",
            entity_name=dungeon_name,
            entity_uuid=dungeon_data.uuid,
            success=True,
            output_files=[str(f) for f in dungeon_output_dir.glob("*")],
            entity_count=len(areas),
            data=dungeon_data.to_dict()
        )
    
    def _process_area_file(self, html_file: Path, area_uuid: str) -> Optional[AreaData]:
        """Process a single area HTML file."""
        
        try:
            with open(html_file, 'r', encoding='utf-8') as f:
                content = f.read()
            
            soup = BeautifulSoup(content, 'html.parser')
            
            # Extract area title and description
            title_elem = soup.find(id='editable-title')
            title = title_elem.get_text(strip=True) if title_elem else f"Area {area_uuid}"
            
            # Extract description
            description = self._extract_description(soup)
            
            # Extract foreshadowing
            foreshadowing = self._extract_foreshadowing(soup)
            
            # Extract monsters
            monsters = self._extract_monsters(soup)
            
            # Extract treasure/loot
            treasures = self._extract_treasures(soup)
            
            # Extract environmental details
            environmental_details = self._extract_environmental_details(soup)
            
            return AreaData(
                uuid=area_uuid,
                title=title,
                description=description,
                foreshadowing=foreshadowing,
                monsters=monsters,
                treasures=treasures,
                environmental_details=environmental_details,
                area_type=self._determine_area_type(title, description)
            )
            
        except Exception as e:
            return None
    
    def _extract_description(self, soup: BeautifulSoup) -> str:
        """Extract main area description."""
        
        # Look for blockquotes containing descriptions
        descriptions = []
        for blockquote in soup.find_all('blockquote'):
            text = blockquote.get_text(strip=True)
            if text:
                descriptions.append(text)
        
        return " ".join(descriptions) if descriptions else ""
    
    def _extract_foreshadowing(self, soup: BeautifulSoup) -> str:
        """Extract foreshadowing information."""
        
        # Look for foreshadowing section
        foreshadowing_heading = soup.find('h5', string=re.compile(r'Foreshadowing', re.I))
        if foreshadowing_heading:
            # Get the next ul element
            ul = foreshadowing_heading.find_next_sibling('ul')
            if ul:
                items = [li.get_text(strip=True) for li in ul.find_all('li')]
                return " ".join(items)
        
        return ""
    
    def _extract_monsters(self, soup: BeautifulSoup) -> List[MonsterData]:
        """Extract monster data from stat blocks."""
        
        monsters = []
        
        # Find all monster blocks
        monster_blocks = soup.find_all('div', class_='monster-block')
        
        for block in monster_blocks:
            try:
                monster = self._parse_monster_block(block)
                if monster:
                    monsters.append(monster)
            except Exception:
                continue  # Skip malformed monster blocks
        
        return monsters
    
    def _parse_monster_block(self, block) -> Optional[MonsterData]:
        """Parse a single monster stat block."""
        
        # Extract monster name from preceding strong tag
        strong_elem = block.find_previous('strong')
        name = strong_elem.get_text(strip=True) if strong_elem else "Unknown Monster"
        
        # Extract basic stats
        statblock = block.find('div', class_='statblock')
        if not statblock:
            return None
        
        # Extract CR, AC, HP, Speed from top row
        top_row = statblock.find('div', class_='statblock-top-row')
        cr = self._extract_stat_value(top_row, 'CR:') if top_row else "0"
        ac = self._extract_stat_value(top_row, 'AC:') if top_row else "10"
        hp = self._extract_stat_value(top_row, 'HP:') if top_row else "1"
        speed = self._extract_stat_value(top_row, 'Speed:') if top_row else "30"
        
        # Extract ability scores
        ability_table = statblock.find('table', class_='statblock-table')
        abilities = {}
        if ability_table:
            headers = [th.get_text(strip=True) for th in ability_table.find('tr').find_all('th')]
            values_row = ability_table.find_all('tr')[1] if len(ability_table.find_all('tr')) > 1 else None
            if values_row:
                values = [td.get_text(strip=True).split()[0] for td in values_row.find_all('td')]
                for i, header in enumerate(headers):
                    if i < len(values):
                        abilities[header.lower()] = values[i]
        
        # Extract attacks
        attacks = self._extract_monster_attacks(statblock)
        
        return MonsterData(
            name=name,
            cr=cr,
            ac=ac,
            hp=hp,
            speed=speed,
            abilities=abilities,
            attacks=attacks
        )
    
    def _extract_stat_value(self, element, label: str) -> str:
        """Extract a stat value by label."""
        if not element:
            return ""
        
        text = element.get_text()
        pattern = f"{re.escape(label)}\\s*([^,\\n]+)"
        match = re.search(pattern, text, re.IGNORECASE)
        return match.group(1).strip() if match else ""
    
    def _extract_monster_attacks(self, statblock) -> List[Dict[str, str]]:
        """Extract monster attacks from actions section."""
        
        attacks = []
        
        # Find Actions section
        actions_heading = statblock.find('h6', string=re.compile(r'Actions', re.I))
        if actions_heading:
            # Get following ul
            ul = actions_heading.find_next_sibling(['ul', 'div'])
            if ul:
                for li in ul.find_all('li'):
                    attack_text = li.get_text(strip=True)
                    if attack_text:
                        attacks.append({"description": attack_text})
        
        return attacks
    
    def _extract_treasures(self, soup: BeautifulSoup) -> List[TreasureData]:
        """Extract treasure and loot information."""
        
        treasures = []
        
        # Look for treasure patterns in the text
        text_content = soup.get_text()
        
        # Extract monetary treasure
        gold_matches = re.findall(r'(\d+[,\d]*)\s*gp', text_content, re.IGNORECASE)
        for gold in gold_matches:
            amount = gold.replace(',', '')
            treasures.append(TreasureData(
                type="currency",
                description=f"{gold} gp",
                value=int(amount) if amount.isdigit() else 0
            ))
        
        # Extract artifact/item mentions
        artifact_pattern = r'(\d+)\s*(?:×|x)?\s*([^,\n]+(?:figurines?|crown|robe|necklace|ring|sword|armor|shield)[^,\n]*)'
        artifact_matches = re.findall(artifact_pattern, text_content, re.IGNORECASE)
        for count, item in artifact_matches:
            treasures.append(TreasureData(
                type="artifact",
                description=f"{count}x {item.strip()}",
                value=0  # Value extraction would need more parsing
            ))
        
        # Extract magic items
        magic_pattern = r'Magic Items?:\s*([^\n]+)'
        magic_matches = re.findall(magic_pattern, text_content, re.IGNORECASE)
        for items in magic_matches:
            treasures.append(TreasureData(
                type="magic_item",
                description=items.strip(),
                value=0
            ))
        
        return treasures
    
    def _extract_environmental_details(self, soup: BeautifulSoup) -> Dict[str, Any]:
        """Extract environmental and atmospheric details."""
        
        details = {
            "atmosphere": "",
            "hazards": [],
            "special_features": []
        }
        
        # Extract atmospheric descriptions
        text_content = soup.get_text()
        
        # Look for atmospheric keywords
        atmosphere_keywords = ["fossils", "formations", "stalactites", "movement", "fractures", "decorations"]
        atmosphere_found = []
        for keyword in atmosphere_keywords:
            if keyword in text_content.lower():
                atmosphere_found.append(keyword)
        
        if atmosphere_found:
            details["atmosphere"] = f"Contains: {', '.join(atmosphere_found)}"
        
        # Look for hazard mentions
        hazard_keywords = ["attack", "poison", "trap", "dangerous", "deadly"]
        for keyword in hazard_keywords:
            if keyword in text_content.lower():
                details["hazards"].append(keyword)
        
        return details
    
    def _determine_area_type(self, title: str, description: str) -> str:
        """Determine the type of dungeon area."""
        
        content = (title + " " + description).lower()
        
        if any(word in content for word in ["entrance", "entry", "door"]):
            return "entrance"
        elif any(word in content for word in ["boss", "final", "throne", "chamber"]):
            return "boss_room"
        elif any(word in content for word in ["treasure", "vault", "hoard"]):
            return "treasure_room"
        elif any(word in content for word in ["corridor", "hallway", "passage"]):
            return "corridor"
        else:
            return "chamber"
    
    def _calculate_difficulty(self, areas: List[AreaData]) -> int:
        """Calculate overall dungeon difficulty based on monsters."""
        
        if not areas:
            return 1
        
        # Simple difficulty calculation based on monster CR values
        total_cr = 0
        monster_count = 0
        
        for area in areas:
            for monster in area.monsters:
                try:
                    # Extract numeric CR value
                    cr_str = re.search(r'(\d+)', monster.cr)
                    if cr_str:
                        total_cr += int(cr_str.group(1))
                        monster_count += 1
                except (ValueError, AttributeError):
                    continue
        
        if monster_count == 0:
            return 1
        
        average_cr = total_cr / monster_count
        return max(1, min(20, int(average_cr)))
    
    def _generate_dungeon_uuid(self, dungeon_name: str) -> str:
        """Generate consistent UUID for dungeon name."""
        import hashlib
        return hashlib.md5(dungeon_name.encode()).hexdigest()[:8]
    
    def _generate_rust_module(self, mod_file: Path, dungeon_data: DungeonData):
        """Generate Rust module file for the dungeon."""
        
        rust_content = f'''//! {dungeon_data.name} - Generated dungeon module
//!
//! Total areas: {dungeon_data.total_areas}
//! Monsters: {dungeon_data.monster_count}
//! Difficulty: {dungeon_data.difficulty_level}

use bevy::prelude::*;
use serde::{{Deserialize, Serialize}};

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct {self._to_rust_identifier(dungeon_data.name)} {{
    pub uuid: String,
    pub name: String,
    pub total_areas: u32,
    pub monster_count: u32,
    pub treasure_count: u32,
    pub difficulty_level: u32,
}}

impl Default for {self._to_rust_identifier(dungeon_data.name)} {{
    fn default() -> Self {{
        Self {{
            uuid: "{dungeon_data.uuid}".to_string(),
            name: "{dungeon_data.name}".to_string(),
            total_areas: {dungeon_data.total_areas},
            monster_count: {dungeon_data.monster_count},
            treasure_count: {dungeon_data.treasure_count},
            difficulty_level: {dungeon_data.difficulty_level},
        }}
    }}
}}

pub fn spawn_dungeon(mut commands: Commands) {{
    commands.spawn({self._to_rust_identifier(dungeon_data.name)}::default());
}}
'''
        
        with open(mod_file, 'w', encoding='utf-8') as f:
            f.write(rust_content)
    
    def _to_rust_identifier(self, name: str) -> str:
        """Convert name to valid Rust identifier."""
        # Remove special characters and convert to PascalCase
        words = re.findall(r'\w+', name)
        return ''.join(word.capitalize() for word in words)
    
    def _create_empty_result(self, dungeon_name: str) -> ProcessingResult:
        """Create empty result for failed processing."""
        return ProcessingResult(
            entity_type="dungeon",
            entity_name=dungeon_name,
            entity_uuid=self._generate_dungeon_uuid(dungeon_name),
            success=False,
            output_files=[],
            entity_count=0,
            data={}
        )
