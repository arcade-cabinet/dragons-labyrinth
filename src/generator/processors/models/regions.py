```python
from typing import Any, Optional, List, Dict
import re
from pydantic import BaseModel, Field
from bs4 import BeautifulSoup

class InnDrinkModel(BaseModel):
    """Drink menu item."""
    name: str
    price: str

class InnFoodMenuModel(BaseModel):
    """Food menu for a d4 roll."""
    roll: str
    menu: List[str]

class InnBulletinModel(BaseModel):
    """Bulletin board entry."""
    text: str
    referenced_location_uuids: List[str] = Field(default_factory=list, description="UUIDs of referenced locations in the bulletin")

class InnRumorModel(BaseModel):
    """Rumor entry."""
    roll: str
    text: str
    referenced_location_uuids: List[str] = Field(default_factory=list, description="UUIDs of referenced locations in the rumor")

class InnNPCModel(BaseModel):
    """NPC present in the inn."""
    npc_uuid: str = Field(..., description="UUID of the NPC (from npc-anchor id)")
    name: Optional[str] = None
    description: Optional[str] = None

class InnPatronModel(BaseModel):
    """Patron or visitor referenced by location link."""
    location_uuid: str = Field(..., description="UUID of the location entity referenced")
    name: Optional[str] = None
    role: Optional[str] = None

class RegionInnModel(BaseModel):
    """Model for an inn or tavern entity, with all UUID connections and spatial data."""
    # Entity identification
    entity_uuid: str = Field(..., description="UUID from filename (entity_{UUID}.html)")

    # Spatial coordinates
    map_hex_id: Optional[str] = Field(None, description="Map hex ID from map-coords anchor")
    map_x: Optional[float] = Field(None, description="Map X coordinate")
    map_y: Optional[float] = Field(None, description="Map Y coordinate")

    # Title and location context
    title: Optional[str] = Field(None, description="Inn name/title from editable-title")
    settlement_uuid: Optional[str] = Field(None, description="UUID of the parent settlement (from breadcrumbs/location link)")
    settlement_name: Optional[str] = Field(None, description="Name of the parent settlement")
    region_uuid: Optional[str] = Field(None, description="UUID of the parent region/realm (from breadcrumbs/realm link)")
    region_name: Optional[str] = Field(None, description="Name of the parent region/realm")

    # NPCs (keeper, staff, visitors, etc.)
    keeper_npc_uuid: Optional[str] = Field(None, description="UUID of the innkeeper NPC")
    staff_npc_uuids: List[str] = Field(default_factory=list, description="UUIDs of staff NPCs")
    patron_npc_uuids: List[str] = Field(default_factory=list, description="UUIDs of patron NPCs (with statblocks)")
    all_npc_uuids: List[str] = Field(default_factory=list, description="All NPC UUIDs found in the inn (keeper, staff, patrons)")

    # Linked locations (patrons/visitors, bulletin, rumors)
    referenced_location_uuids: List[str] = Field(default_factory=list, description="UUIDs of referenced locations (patrons, bulletin, rumors)")
    referenced_hex_uuids: List[str] = Field(default_factory=list, description="UUIDs of referenced hexes (rumors)")

    # Menu
    drinks: List[InnDrinkModel] = Field(default_factory=list)
    food_menus: List[InnFoodMenuModel] = Field(default_factory=list)

    # Bulletin and rumors
    bulletins: List[InnBulletinModel] = Field(default_factory=list)
    rumors: List[InnRumorModel] = Field(default_factory=list)

    @classmethod
    def extract_from_html(cls, html_content: str, filename: str) -> "RegionInnModel":
        soup = BeautifulSoup(html_content, 'html.parser')

        # Entity UUID from filename
        entity_uuid = filename.replace('entity_', '').replace('.html', '')

        # Map coordinates
        map_coords = soup.find('a', class_='map-coords')
        map_hex_id = map_coords.get('hex') if map_coords else None
        map_x = float(map_coords.get('x', 0)) if map_coords and map_coords.get('x') else None
        map_y = float(map_coords.get('y', 0)) if map_coords and map_coords.get('y') else None

        # Title
        title_span = soup.find('span', id='editable-title')
        title = title_span.text.strip('"') if title_span else None

        # Breadcrumbs: extract settlement and region/realm UUIDs and names
        breadcrumbs = soup.find('span', class_='breadcrumbs')
        settlement_uuid = None
        settlement_name = None
        region_uuid = None
        region_name = None
        if breadcrumbs:
            for a in breadcrumbs.find_all('a'):
                href = a.get('href', '')
                if '/realm/' in href:
                    match = re.search(r'/realm/([^/]+)', href)
                    if match:
                        region_uuid = match.group(1)
                        region_name = a.text.strip()
                elif '/location/' in href:
                    match = re.search(r'/location/([^/]+)', href)
                    if match:
                        settlement_uuid = match.group(1)
                        settlement_name = a.text.strip()

        # NPCs: keeper, staff, patrons (with statblocks), all
        # Keeper: first <a class="npc-anchor"> after <h5>Keeper</h5>
        keeper_npc_uuid = None
        staff_npc_uuids = []
        patron_npc_uuids = []
        all_npc_uuids = []
        # Find all npc-anchor tags
        npc_anchors = soup.find_all('a', class_='npc-anchor')
        for anchor in npc_anchors:
            npc_id = anchor.get('id')
            if npc_id:
                all_npc_uuids.append(npc_id)
        # Keeper
        keeper_h5 = soup.find('h5', string=re.compile(r'Keeper', re.I))
        if keeper_h5:
            next_anchor = keeper_h5.find_next('a', class_='npc-anchor')
            if next_anchor and next_anchor.get('id'):
                keeper_npc_uuid = next_anchor.get('id')
        # Staff: all <a class="npc-anchor"> between <h5>Staff</h5> and next <h5>
        staff_h5 = soup.find('h5', string=re.compile(r'Staff', re.I))
        if staff_h5:
            next_h5 = staff_h5.find_next(lambda tag: tag.name == 'h5' and tag != staff_h5)
            current = staff_h5
            while True:
                current = current.find_next(['a', 'h5'])
                if current is None or (current.name == 'h5' and current != staff_h5):
                    break
                if current.name == 'a' and 'npc-anchor' in current.get('class', []):
                    npc_id = current.get('id')
                    if npc_id:
                        staff_npc_uuids.append(npc_id)
        # Patrons: all <a class="npc-anchor"> after <h5>Patrons & Visitors</h5>
        patrons_h5 = soup.find('h5', string=re.compile(r'Patrons', re.I))
        if patrons_h5:
            # Find all <a class="npc-anchor"> after this h5 until next h5 or end
            current = patrons_h5
            while True:
                current = current.find_next(['a', 'h5'])
                if current is None or (current.name == 'h5' and current != patrons_h5):
                    break
                if current.name == 'a' and 'npc-anchor' in current.get('class', []):
                    npc_id = current.get('id')
                    if npc_id:
                        patron_npc_uuids.append(npc_id)

        # Linked locations: patrons/visitors (location links after "Patrons & Visitors")
        referenced_location_uuids = []
        referenced_hex_uuids = []
        # All <a href="/sandbox/nTR8nJOW/location/UUID">
        for a in soup.find_all('a', href=re.compile(r'/location/')):
            match = re.search(r'/location/([^/]+)', a['href'])
            if match:
                referenced_location_uuids.append(match.group(1))
        # All <a href="/sandbox/nTR8nJOW/hex/UUID">
        for a in soup.find_all('a', href=re.compile(r'/hex/')):
            match = re.search(r'/hex/([^/]+)', a['href'])
            if match:
                referenced_hex_uuids.append(match.group(1))
        referenced_location_uuids = list(set(referenced_location_uuids))
        referenced_hex_uuids = list(set(referenced_hex_uuids))

        # Drinks menu: <h5>Drinks</h5> followed by <table>
        drinks = []
        drinks_h5 = soup.find('h5', string=re.compile(r'Drinks', re.I))
        if drinks_h5:
            drinks_table = drinks_h5.find_next('table')
            if drinks_table:
                for tr in drinks_table.find_all('tr'):
                    tds = tr.find_all('td')
                    if len(tds) == 2:
                        drinks.append(InnDrinkModel(name=tds[0].text.strip(), price=tds[1].text.strip()))

        # Food menu: <h5>Food</h5> followed by <table>
        food_menus = []
        food_h5 = soup.find('h5', string=re.compile(r'Food', re.I))
        if food_h5:
            food_table = food_h5.find_next('table')
            if food_table:
                for tr in food_table.find_all('tr'):
                    tds = tr.find_all('td')
                    if len(tds) == 2:
                        roll = tds[0].text.strip()
                        # Split menu items by unicode black diamond (&#11037;)
                        menu_items = [item.strip() for item in tds[1].decode_contents().split('&#11037;') if item.strip()]
                        food_menus.append(InnFoodMenuModel(roll=roll, menu=menu_items))

        # Bulletin: <table> with <th> Bulletin </th>
        bulletins = []
        for table in soup.find_all('table'):
            th = table.find('th')
            if th and 'Bulletin' in th.text:
                for tr in table.find_all('tr'):
                    tds = tr.find_all('td')
                    if tds:
                        text = tds[0].get_text(separator=' ', strip=True)
                        # Find referenced location UUIDs in <a href="/location/UUID">
                        loc_uuids = []
                        for a in tds[0].find_all('a', href=re.compile(r'/location/')):
                            match = re.search(r'/location/([^/]+)', a['href'])
                            if match:
                                loc_uuids.append(match.group(1))
                        bulletins.append(InnBulletinModel(text=text, referenced_location_uuids=loc_uuids))

        # Rumors: <table> with <th> d6 </th> and <th> Rumor </th>
        rumors = []
        for table in soup.find_all('table'):
            ths = table.find_all('th')
            if len(ths) >= 2 and 'Rumor' in ths[1].text:
                for tr in table.find_all('tr'):
                    tds = tr.find_all('td')
                    if len(tds) == 2:
                        roll = tds[0].text.strip()
                        text = tds[1].get_text(separator=' ', strip=True)
                        # Find referenced location and hex UUIDs in <a href="/location/UUID"> and <a href="/hex/UUID">
                        loc_uuids = []
                        for a in tds[1].find_all('a', href=re.compile(r'/location/')):
                            match = re.search(r'/location/([^/]+)', a['href'])
                            if match:
                                loc_uuids.append(match.group(1))
                        rumors.append(InnRumorModel(roll=roll, text=text, referenced_location_uuids=loc_uuids))

        return cls(
            entity_uuid=entity_uuid,
            map_hex_id=map_hex_id,
            map_x=map_x,
            map_y=map_y,
            title=title,
            settlement_uuid=settlement_uuid,
            settlement_name=settlement_name,
            region_uuid=region_uuid,
            region_name=region_name,
            keeper_npc_uuid=keeper_npc_uuid,
            staff_npc_uuids=staff_npc_uuids,
            patron_npc_uuids=patron_npc_uuids,
            all_npc_uuids=all_npc_uuids,
            referenced_location_uuids=referenced_location_uuids,
            referenced_hex_uuids=referenced_hex_uuids,
            drinks=drinks,
            food_menus=food_menus,
            bulletins=bulletins,
            rumors=rumors
        )

    @classmethod
    def extract_uuid_connections(cls) -> Dict[str, str]:
        """Return mapping of UUID fields to their entity types."""
        return {
            "entity_uuid": "inn_entity",
            "settlement_uuid": "settlement",
            "region_uuid": "region",
            "keeper_npc_uuid": "npc",
            "staff_npc_uuids": "npc",
            "patron_npc_uuids": "npc",
            "all_npc_uuids": "npc",
            "referenced_location_uuids": "location",
            "referenced_hex_uuids": "hex"
        }
```
**UUID Connection Fields:**
- `entity_uuid`: this inn/tavern entity
- `settlement_uuid`: parent settlement (from breadcrumbs)
- `region_uuid`: parent region/realm (from breadcrumbs)
- `keeper_npc_uuid`: innkeeper NPC
- `staff_npc_uuids`: staff NPCs
- `patron_npc_uuids`: patrons/visitors with statblocks
- `all_npc_uuids`: all NPCs found (keeper, staff, patrons)
- `referenced_location_uuids`: all referenced locations (patrons, bulletin, rumors)
- `referenced_hex_uuids`: all referenced hexes (rumors)

**Extraction covers:**
- All spatial coordinates
- All UUID connections (settlement, region, NPCs, locations, hexes)
- Menu, bulletin, rumors, and all cross-references

**All imports are absolute and explicit.**