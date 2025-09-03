```python
# Absolute imports only - NO wildcards
from typing import Any, List, Dict, Optional
import re
from pydantic import BaseModel, Field
from bs4 import BeautifulSoup

class MonsterStatBlock(BaseModel):
    """Represents a D&D monster stat block found in an area."""
    # Monster UUID (from block-<uuid> in monster-block div id)
    monster_uuid: str = Field(..., description="UUID of the monster (from monster-block div id)")
    name: str = Field(..., description="Monster name")
    cr: str | None = Field(None, description="Challenge Rating (CR) and XP")
    ac: str | None = Field(None, description="Armor Class (AC)")
    hp: str | None = Field(None, description="Hit Points (HP)")
    speed: str | None = Field(None, description="Speed")
    stats: Dict[str, str] = Field(default_factory=dict, description="Ability scores (STR, DEX, etc.)")
    saving_throws: str | None = Field(None, description="Saving throws")
    immunities: str | None = Field(None, description="Immunities")
    senses: str | None = Field(None, description="Senses")
    alignment: str | None = Field(None, description="Alignment")
    languages: str | None = Field(None, description="Languages")
    traits: List[str] = Field(default_factory=list, description="Special traits")
    actions: List[str] = Field(default_factory=list, description="Actions")

    @classmethod
    def extract_from_block(cls, block: BeautifulSoup, name: str) -> "MonsterStatBlock":
        monster_uuid = block.get("id", "").replace("block-", "")
        statblock = block.find("div", class_="statblock")
        cr = ac = hp = speed = None
        stats = {}
        saving_throws = immunities = senses = alignment = languages = None
        traits = []
        actions = []

        if statblock:
            # Top row
            top_row = statblock.find("div", class_="statblock-top-row")
            if top_row:
                for div in top_row.find_all("div"):
                    text = div.get_text(strip=True)
                    if text.startswith("CR:"):
                        cr = text.replace("CR:", "").strip()
                    elif text.startswith("AC:"):
                        ac = text.replace("AC:", "").strip()
                    elif text.startswith("HP:"):
                        hp = text.replace("HP:", "").strip()
                    elif text.startswith("Speed:"):
                        speed = text.replace("Speed:", "").strip()
            # Stats table
            table = statblock.find("table", class_="statblock-table")
            if table:
                headers = [th.get_text(strip=True) for th in table.find_all("th")]
                values = [td.get_text(strip=True) for td in table.find_all("td")]
                stats = dict(zip(headers, values))
            # Statblock-container
            container = statblock.find("div", class_="statblock-container")
            if container:
                for li in container.find_all("li"):
                    txt = li.get_text(" ", strip=True)
                    if txt.startswith("Saving Throws:"):
                        saving_throws = txt.replace("Saving Throws:", "").strip()
                    elif txt.startswith("Immunities:"):
                        immunities = txt.replace("Immunities:", "").strip()
                    elif txt.startswith("Senses:"):
                        senses = txt.replace("Senses:", "").strip()
                    elif txt.startswith("Alignment:"):
                        alignment = txt.replace("Alignment:", "").strip()
                    elif txt.startswith("Languages:"):
                        languages = txt.replace("Languages:", "").strip()
                # Traits
                for ul in container.find_all("ul"):
                    for li in ul.find_all("li"):
                        traits.append(li.get_text(" ", strip=True))
                # Actions
                actions_header = container.find("h6", string=re.compile("Actions", re.I))
                if actions_header:
                    actions_ul = actions_header.find_next("ul")
                    if actions_ul:
                        for li in actions_ul.find_all("li"):
                            actions.append(li.get_text(" ", strip=True))
        return cls(
            monster_uuid=monster_uuid,
            name=name,
            cr=cr,
            ac=ac,
            hp=hp,
            speed=speed,
            stats=stats,
            saving_throws=saving_throws,
            immunities=immunities,
            senses=senses,
            alignment=alignment,
            languages=languages,
            traits=traits,
            actions=actions
        )

class TreasureItem(BaseModel):
    """Represents a single treasure item or group of items."""
    description: str = Field(..., description="Description of the treasure item")
    value_gp: float | None = Field(None, description="Value in gold pieces, if applicable")
    type: str | None = Field(None, description="Type of treasure (coin, artifact, gemstone, magic item, tool, etc.)")
    details: Dict[str, Any] = Field(default_factory=dict, description="Additional details (e.g., quantity, names)")

class Trap(BaseModel):
    """Represents a trap or hazard in an area."""
    name: str = Field(..., description="Name of the trap")
    description: str = Field(..., description="Description of the trap")
    dc: int | None = Field(None, description="Difficulty Class to avoid or disarm")
    ability: str | None = Field(None, description="Relevant ability for the check")
    damage: str | None = Field(None, description="Damage dice or effect")

class DungeonArea(BaseModel):
    """Individual area within a dungeon."""
    # Entity identification
    entity_uuid: str = Field(..., description="UUID from filename (entity_{UUID}.html)")

    # Spatial coordinates and location
    area_number: int | None = Field(None, description="Area number like '1' from 'Cave area #1'")
    dungeon_name: str | None = Field(None, description="Dungeon name from doc-title")
    map_hex_id: str | None = Field(None, description="Map hex ID from map-coords anchor")
    map_x: float | None = Field(None, description="Map X coordinate")
    map_y: float | None = Field(None, description="Map Y coordinate")

    # Entity connections via UUIDs extracted from links
    dungeon_uuid: str | None = Field(None, description="UUID of parent dungeon from breadcrumbs")
    connected_area_uuids: List[str] = Field(default_factory=list, description="UUIDs of connected areas")
    monster_uuids: List[str] = Field(default_factory=list, description="UUIDs of monsters in this area")
    treasure_uuids: List[str] = Field(default_factory=list, description="UUIDs of treasure items (not present in sample, reserved for future)")
    quest_item_uuids: List[str] = Field(default_factory=list, description="UUIDs of quest-related items (not present in sample, reserved for future)")

    # Area content
    area_title: str | None = Field(None, description="Area title (from editable-title span)")
    foreshadowing: List[str] = Field(default_factory=list, description="Foreshadowing clues or hints")
    description: str | None = Field(None, description="Area description text")
    treasures: List[TreasureItem] = Field(default_factory=list, description="List of treasure items found in this area")
    traps: List[Trap] = Field(default_factory=list, description="List of traps or hazards in this area")
    monsters: List[MonsterStatBlock] = Field(default_factory=list, description="Monster stat blocks in this area")
    special_features: List[str] = Field(default_factory=list, description="Special features, environmental notes, or notable objects")

    @classmethod
    def extract_from_html(cls, html_content: str, filename: str) -> "DungeonArea":
        soup = BeautifulSoup(html_content, 'html.parser')

        # Extract entity UUID from filename
        entity_uuid = filename.replace('entity_', '').replace('.html', '')

        # Extract area context from hidden doc-title
        doc_title = soup.find('div', {'id': 'doc-title'})
        area_number = None
        dungeon_name = None
        if doc_title:
            area_match = re.search(r'area #(\d+)', doc_title.text, re.I)
            area_number = int(area_match.group(1)) if area_match else None
            if ' in ' in doc_title.text:
                dungeon_name = doc_title.text.split(' in ', 1)[1].strip()

        # Extract map coordinates from map-coords anchor
        map_coords = soup.find('a', class_='map-coords')
        map_hex_id = map_coords.get('hex') if map_coords else None
        map_x = float(map_coords.get('x', 0)) if map_coords and map_coords.get('x') else None
        map_y = float(map_coords.get('y', 0)) if map_coords and map_coords.get('y') else None

        # Extract area title
        editable_title = soup.find('span', {'id': 'editable-title'})
        area_title = editable_title.get_text(strip=True) if editable_title else None

        # Extract dungeon UUID from breadcrumbs (location link)
        dungeon_uuid = None
        breadcrumbs = soup.find('span', class_='breadcrumbs')
        if breadcrumbs:
            loc_link = breadcrumbs.find('a', href=re.compile(r'/location/'))
            if loc_link:
                match = re.search(r'/location/([A-Za-z0-9]+)', loc_link['href'])
                if match:
                    dungeon_uuid = match.group(1)

        # Extract foreshadowing
        foreshadowing = []
        for h5 in soup.find_all('h5'):
            if h5.get_text(strip=True).lower() == "foreshadowing":
                ul = h5.find_next_sibling('ul')
                if ul:
                    foreshadowing = [li.get_text(" ", strip=True) for li in ul.find_all('li')]

        # Extract description
        description = None
        for h5 in soup.find_all('h5'):
            if h5.get_text(strip=True).lower() == "description":
                blockquote = h5.find_next_sibling('blockquote')
                if blockquote:
                    description = blockquote.get_text(" ", strip=True)

        # Extract treasures, traps, monsters, and special features
        treasures = []
        traps = []
        monsters = []
        special_features = []
        monster_uuids = []

        # Find all <ul> after description and foreshadowing
        for ul in soup.find_all('ul'):
            for li in ul.find_all('li', recursive=False):
                li_text = li.get_text(" ", strip=True)
                # Trap detection
                if re.search(r'\btrap\b', li_text, re.I):
                    # Try to extract DC and ability
                    dc_match = re.search(r'DC\s*(\d+)', li_text)
                    ability_match = re.search(r'(Strength|Dexterity|Constitution|Intelligence|Wisdom|Charisma)', li_text, re.I)
                    damage_match = re.search(r'(\d+d\d+)', li_text)
                    traps.append(Trap(
                        name=re.sub(r'\s*\(.*?\)', '', li_text.split('trap')[0] + 'trap', flags=re.I).strip(),
                        description=li_text,
                        dc=int(dc_match.group(1)) if dc_match else None,
                        ability=ability_match.group(1) if ability_match else None,
                        damage=damage_match.group(1) if damage_match else None
                    ))
                # Monster stat block detection
                monster_block = li.find('div', class_='monster-block')
                if monster_block:
                    # Try to get monster name from previous <strong> or context
                    strong = li.find('strong')
                    monster_name = strong.get_text(strip=True) if strong else "Unknown"
                    monster = MonsterStatBlock.extract_from_block(monster_block, monster_name)
                    monsters.append(monster)
                    monster_uuids.append(monster.monster_uuid)
                # Treasure detection (coins, artifacts, gemstones, magic items, tools)
                if re.search(r'\bcoin\b|\bgp\b|\bartifact\b|\bgemstone\b|\bmagic item\b|\btools\b', li_text, re.I):
                    # Try to parse value and type
                    value_match = re.search(r'(\d[\d,]*)\s*gp', li_text.replace(',', ''))
                    value_gp = float(value_match.group(1)) if value_match else None
                    type_ = None
                    if 'coin' in li_text.lower() or 'gp' in li_text.lower():
                        type_ = 'coin'
                    elif 'artifact' in li_text.lower():
                        type_ = 'artifact'
                    elif 'gemstone' in li_text.lower():
                        type_ = 'gemstone'
                    elif 'magic item' in li_text.lower():
                        type_ = 'magic item'
                    elif 'tools' in li_text.lower():
                        type_ = 'tool'
                    # Try to extract details (list of items)
                    details = {}
                    # Look for <strong>Magic Items:</strong> or similar
                    magic_items = []
                    if 'Magic Items:' in li_text:
                        # Get all following <li> or text after 'Magic Items:'
                        after = li_text.split('Magic Items:')[-1]
                        magic_items = [x.strip() for x in re.split(r',|and', after) if x.strip()]
                        details['magic_items'] = magic_items
                    treasures.append(TreasureItem(
                        description=li_text,
                        value_gp=value_gp,
                        type=type_,
                        details=details
                    ))
                # Special features (corpses, jars, tools, etc.)
                if re.search(r'\bcorpse\b|\bjar\b|\bskull\b|\bremains\b|\bbody\b|\btools\b', li_text, re.I):
                    special_features.append(li_text)

        # Also check for <li> with reroll icons for random events/loot
        for li in soup.find_all('li'):
            if li.find('a', class_='btn-icon'):
                li_text = li.get_text(" ", strip=True)
                # If not already in treasures or special features, add as special feature
                if li_text not in special_features:
                    special_features.append(li_text)

        # Connected area UUIDs: not present in sample, but placeholder for future
        connected_area_uuids = []

        # Treasure and quest item UUIDs: not present in sample, but placeholder for future
        treasure_uuids = []
        quest_item_uuids = []

        return cls(
            entity_uuid=entity_uuid,
            area_number=area_number,
            dungeon_name=dungeon_name,
            map_hex_id=map_hex_id,
            map_x=map_x,
            map_y=map_y,
            dungeon_uuid=dungeon_uuid,
            connected_area_uuids=connected_area_uuids,
            monster_uuids=monster_uuids,
            treasure_uuids=treasure_uuids,
            quest_item_uuids=quest_item_uuids,
            area_title=area_title,
            foreshadowing=foreshadowing,
            description=description,
            treasures=treasures,
            traps=traps,
            monsters=monsters,
            special_features=special_features
        )

    @classmethod
    def extract_uuid_connections(cls) -> Dict[str, str]:
        """Return mapping of UUID fields to their entity types."""
        return {
            "entity_uuid": "dungeon_area",
            "dungeon_uuid": "dungeon",
            "connected_area_uuids": "dungeon_area",
            "monster_uuids": "monster",
            "treasure_uuids": "treasure",
            "quest_item_uuids": "quest_item"
        }
```

---

**UUID Connection Fields and Documentation:**
- `entity_uuid`: UUID of this dungeon area (from filename, e.g., `entity_TIg0z5A6.html`)
- `dungeon_uuid`: UUID of the parent dungeon (from breadcrumbs, e.g., `/location/2LdDOJYG`)
- `connected_area_uuids`: UUIDs of directly connected areas (not present in sample, placeholder)
- `monster_uuids`: UUIDs of monsters present in this area (from monster-block divs, e.g., `block-50kMuIKx`)
- `treasure_uuids`: UUIDs of treasure items (not present in sample, placeholder)
- `quest_item_uuids`: UUIDs of quest-related items (not present in sample, placeholder)

**Spatial and Content Extraction:**
- Area number, dungeon name, map hex, x/y coordinates, area title, foreshadowing, description, treasures, traps, monsters, and special features are all extracted and modeled.

**Entity Relationships:**
- All UUID fields are explicitly documented and mapped to their entity types.
- Monster stat blocks are parsed and modeled in detail.
- Treasure and trap systems are modeled for extensibility.

**Parsing Logic:**
- Uses BeautifulSoup and regex to extract all relevant data and UUIDs.
- Models are ready for systematic integration and further extension.