```python
# Absolute imports only - NO wildcards
from typing import Any, ClassVar
import re
from pydantic import BaseModel, Field
from bs4 import BeautifulSoup

class EstablishmentInventoryItem(BaseModel):
    """Single inventory item for an establishment."""
    item_name: str = Field(..., description="Name of the item")
    price: str = Field(..., description="Price string as listed (e.g., '10 gp', '9 sp', '0 gp')")

class EstablishmentQuest(BaseModel):
    """Quest or job posting associated with an establishment."""
    quest_type: str = Field(..., description="Type or summary of the quest (e.g., 'Delivery', 'Help', etc.)")
    description: str = Field(..., description="Full text description of the quest")
    reward: str | None = Field(None, description="Reward for the quest, if specified")
    target_location_uuid: str | None = Field(None, description="UUID of the target location (if delivery or similar)")
    target_location_name: str | None = Field(None, description="Name of the target location (if present)")

class EstablishmentNPC(BaseModel):
    """NPC associated with the establishment (owner, clerk, etc.)."""
    role: str = Field(..., description="Role or job title (e.g., 'Physician', 'Merchant', 'Clerk', etc.)")
    name: str = Field(..., description="NPC's name")
    description: str = Field(..., description="Physical and personality description")
    mood: str | None = Field(None, description="Mood or disposition (e.g., 'Joyful', 'Nervous')")
    pocket_money: dict[str, int] = Field(default_factory=dict, description="Money in NPC's pocket (cp, sp, gp, etc.)")
    pocket_items: list[str] = Field(default_factory=list, description="Other items in NPC's pocket")

class EstablishmentFactionMembership(BaseModel):
    """Faction membership revealed in spoilers."""
    faction_uuid: str = Field(..., description="UUID of the faction")
    faction_name: str = Field(..., description="Name of the faction")

class SettlementEstablishment(BaseModel):
    """Comprehensive model for a settlement establishment with UUID and spatial tracking."""
    # Entity identification
    entity_uuid: str = Field(..., description="UUID from filename (entity_{UUID}.html)")

    # Spatial coordinates and location
    settlement_name: str | None = Field(None, description="Settlement name from doc-title")
    map_hex_id: str | None = Field(None, description="Map hex ID from map-coords anchor")
    map_x: float | None = Field(None, description="Map X coordinate")
    map_y: float | None = Field(None, description="Map Y coordinate")

    # Entity connections via UUIDs extracted from links
    settlement_uuid: str | None = Field(None, description="UUID of parent settlement from breadcrumbs")
    district_uuid: str | None = Field(None, description="UUID of district/street from breadcrumbs")
    realm_uuid: str | None = Field(None, description="UUID of realm from breadcrumbs")
    faction_uuids: list[str] = Field(default_factory=list, description="UUIDs of factions referenced (spoilers or links)")
    npc_uuids: list[str] = Field(default_factory=list, description="UUIDs of NPCs with anchors (not present in these samples)")
    location_uuids: list[str] = Field(default_factory=list, description="UUIDs of other locations referenced (e.g., delivery targets, taverns)")

    # Social and economic data
    establishment_type: str | None = Field(None, description="Type of establishment (e.g., 'Physician', 'Game Shop')")
    establishment_name: str | None = Field(None, description="Name of the establishment")
    npcs: list[EstablishmentNPC] = Field(default_factory=list, description="NPCs associated with the establishment")
    quests: list[EstablishmentQuest] = Field(default_factory=list, description="Quests or job postings")
    inventory: list[EstablishmentInventoryItem] = Field(default_factory=list, description="Inventory or menu items")
    faction_memberships: list[EstablishmentFactionMembership] = Field(default_factory=list, description="Faction memberships revealed in spoilers")

    # --- Extraction Methods ---

    @classmethod
    def extract_from_html(cls, html_content: str, filename: str) -> "SettlementEstablishment":
        """Extract all coordinate, UUID, social, and economic data from HTML using BeautifulSoup."""
        soup = BeautifulSoup(html_content, 'html.parser')

        # Entity UUID from filename
        entity_uuid = filename.replace('entity_', '').replace('.html', '')

        # Settlement context from hidden doc-title
        doc_title = soup.find('div', {'id': 'doc-title'})
        settlement_name = doc_title.text.strip() if doc_title else None

        # Map coordinates from map-coords anchor
        map_coords = soup.find('a', class_='map-coords')
        map_hex_id = map_coords.get('hex') if map_coords else None
        map_x = float(map_coords.get('x')) if map_coords and map_coords.get('x') else None
        map_y = float(map_coords.get('y')) if map_coords and map_coords.get('y') else None

        # Breadcrumbs: extract realm, settlement, district UUIDs
        breadcrumbs = soup.find('span', class_='breadcrumbs')
        realm_uuid = None
        settlement_uuid = None
        district_uuid = None
        if breadcrumbs:
            links = breadcrumbs.find_all('a')
            for link in links:
                href = link.get('href', '')
                # /realm/{uuid}
                m = re.search(r'/realm/([A-Za-z0-9]+)', href)
                if m:
                    realm_uuid = m.group(1)
                # /location/{uuid}
                m = re.search(r'/location/([A-Za-z0-9]+)', href)
                if m:
                    if not settlement_uuid:
                        settlement_uuid = m.group(1)
                    else:
                        district_uuid = m.group(1)

        # Faction UUIDs (from spoilers or links)
        faction_uuids = []
        faction_memberships = []
        for spoiler in soup.find_all('span', class_='spoiler'):
            faction_link = spoiler.find('a', href=re.compile(r'/faction/'))
            if faction_link:
                m = re.search(r'/faction/([A-Za-z0-9]+)', faction_link['href'])
                if m:
                    faction_uuids.append(m.group(1))
                    faction_memberships.append(EstablishmentFactionMembership(
                        faction_uuid=m.group(1),
                        faction_name=faction_link.text.strip()
                    ))

        # Location UUIDs (other referenced locations, e.g., delivery targets, taverns)
        location_uuids = []
        for a in soup.find_all('a', href=re.compile(r'/location/')):
            m = re.search(r'/location/([A-Za-z0-9]+)', a['href'])
            if m:
                uuid = m.group(1)
                # Avoid adding the main settlement/district UUIDs again
                if uuid not in (settlement_uuid, district_uuid):
                    location_uuids.append(uuid)

        # NPC UUIDs (not present in these samples, but included for completeness)
        npc_anchors = soup.find_all('a', class_='npc-anchor')
        npc_uuids = [a.get('name') for a in npc_anchors if a.get('name')]

        # Establishment name and type
        editable_title = soup.find('span', id='editable-title')
        establishment_name = editable_title.text.strip() if editable_title else None
        establishment_type = None
        editable_title_container = soup.find('div', id='editable-title-container')
        if editable_title_container:
            em = editable_title_container.find('em')
            if em:
                establishment_type = em.text.strip()

        # NPC extraction (owner, clerk, etc.)
        npcs = []
        # Look for <p> blocks with "Physician:", "Merchant:", "Clerk:", etc.
        for p in soup.find_all('p'):
            text = p.get_text()
            # Try to match "Role: Name. Description (Mood)."
            m = re.match(r'\s*(\w+):\s+([^.]+)\.\s+(.+)\((\w+)\)\.', text)
            if m:
                role, name, desc, mood = m.groups()
                # Pocket items/money
                pocket_money, pocket_items = cls._extract_pocket_info(p)
                npcs.append(EstablishmentNPC(
                    role=role,
                    name=name.strip(),
                    description=desc.strip(),
                    mood=mood.strip(),
                    pocket_money=pocket_money,
                    pocket_items=pocket_items
                ))
            else:
                # Try to match "Role: Name. Description (<em>Mood</em>)."
                strong = p.find('strong')
                em = p.find('em')
                if strong and em:
                    # Try to extract role from text before strong
                    role_match = re.match(r'\s*(\w+):', p.text)
                    role = role_match.group(1) if role_match else "Unknown"
                    name = strong.text.strip()
                    # Description is text between strong and em
                    desc = p.text.split(strong.text)[-1].split('(')[0].strip('. ')
                    mood = em.text.strip()
                    pocket_money, pocket_items = cls._extract_pocket_info(p)
                    npcs.append(EstablishmentNPC(
                        role=role,
                        name=name,
                        description=desc,
                        mood=mood,
                        pocket_money=pocket_money,
                        pocket_items=pocket_items
                    ))

        # Quest extraction (from <ul> in <p>)
        quests = []
        for ul in soup.find_all('ul'):
            for li in ul.find_all('li'):
                # Delivery quest pattern
                delivery_match = re.search(
                    r'deliver (.+?) to\s+<a href="[^"]+/location/([A-Za-z0-9]+)"><strong>([^<]+)</strong></a> in <strong>([^<]+)</strong>.*?Reward is <strong>([^<]+)</strong>',
                    str(li), re.IGNORECASE)
                if delivery_match:
                    item, target_uuid, target_name, target_location, reward = delivery_match.groups()
                    quests.append(EstablishmentQuest(
                        quest_type="Delivery",
                        description=f"Deliver {item} to {target_name} in {target_location}",
                        reward=reward,
                        target_location_uuid=target_uuid,
                        target_location_name=target_location
                    ))
                else:
                    # Other quest patterns (e.g., "Bothered by an unwanted guest.")
                    text = li.get_text().strip()
                    if text and not text.lower().startswith("unlock this page"):
                        quests.append(EstablishmentQuest(
                            quest_type="Other",
                            description=text,
                            reward=None,
                            target_location_uuid=None,
                            target_location_name=None
                        ))

        # Inventory extraction (from <table> after <h5> Inventory)
        inventory = []
        for h5 in soup.find_all('h5'):
            if 'Inventory' in h5.text:
                table = h5.find_next('table')
                if table:
                    for tr in table.find_all('tr'):
                        tds = tr.find_all('td')
                        if len(tds) == 2:
                            item_name = tds[0].text.strip()
                            price = tds[1].text.strip()
                            inventory.append(EstablishmentInventoryItem(
                                item_name=item_name,
                                price=price
                            ))

        return cls(
            entity_uuid=entity_uuid,
            settlement_name=settlement_name,
            map_hex_id=map_hex_id,
            map_x=map_x,
            map_y=map_y,
            settlement_uuid=settlement_uuid,
            district_uuid=district_uuid,
            realm_uuid=realm_uuid,
            faction_uuids=faction_uuids,
            npc_uuids=npc_uuids,
            location_uuids=location_uuids,
            establishment_type=establishment_type,
            establishment_name=establishment_name,
            npcs=npcs,
            quests=quests,
            inventory=inventory,
            faction_memberships=faction_memberships
        )

    @staticmethod
    def _extract_pocket_info(p_tag) -> tuple[dict[str, int], list[str]]:
        """Extract pocket money and items from <small> tags."""
        pocket_money = {}
        pocket_items = []
        small = p_tag.find('small')
        if small:
            text = small.get_text()
            # Money: e.g., "3 cp", "3 sp", "500 gp"
            for denom in ['cp', 'sp', 'gp', 'ep', 'pp']:
                m = re.findall(r'(\d+)\s*' + denom, text)
                if m:
                    pocket_money[denom] = sum(int(x) for x in m)
            # Items: look for "and <strong>item</strong>"
            for strong in small.find_all('strong'):
                val = strong.text.strip()
                if not re.match(r'^\d+\s*(cp|sp|gp|ep|pp)$', val):
                    pocket_items.append(val)
        return pocket_money, pocket_items

    @classmethod
    def extract_uuid_connections(cls) -> dict[str, str]:
        """Return mapping of UUID fields to their entity types."""
        return {
            "entity_uuid": "settlement_entity",
            "settlement_uuid": "settlement",
            "district_uuid": "district",
            "realm_uuid": "realm",
            "faction_uuids": "faction",
            "npc_uuids": "npc",
            "location_uuids": "location"
        }
```

---

**UUID Connection Fields and Their Entity Types:**
- `entity_uuid`: settlement_entity (from filename)
- `settlement_uuid`: settlement (from breadcrumbs `/location/{uuid}`)
- `district_uuid`: district (from breadcrumbs `/location/{uuid}` if present)
- `realm_uuid`: realm (from breadcrumbs `/realm/{uuid}`)
- `faction_uuids`: faction (from spoiler or link `/faction/{uuid}`)
- `npc_uuids`: npc (from `<a class="npc-anchor" name="{uuid}">`, not present in these samples)
- `location_uuids`: location (from any `/location/{uuid}` link not already used for settlement/district)

**Spatial Data:**
- `map_hex_id`, `map_x`, `map_y` from `<a class="map-coords" ...>`
- `settlement_name` from `<div hidden id="doc-title">`

**Social/Economic Data:**
- `npcs`: List of NPCs with role, name, description, mood, pocket money/items
- `quests`: List of quests (delivery, help, etc.) with reward and target location UUIDs
- `inventory`: List of items and prices (from tables)
- `faction_memberships`: Faction memberships revealed in spoilers

**Extraction Methods:**
- All extraction is performed with BeautifulSoup, using explicit patterns for UUIDs and entity relationships.

**No wildcard imports. All fields and relationships are explicit and documented.**