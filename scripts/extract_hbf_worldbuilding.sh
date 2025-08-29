#!/bin/bash

# Extract HBF data by category for worldbuilding analysis
# Creates organized dumps of all content by regions, settlements, factions, dungeons

set -euo pipefail

HBF_DB="memory-bank/world-output/nTR8nJOW.hbf"
OUTPUT_DIR="memory-bank/world-building"

# Create output directory structure
mkdir -p "$OUTPUT_DIR"/{regions,settlements,factions,dungeons}

echo "🗂️  Extracting HBF data for worldbuilding analysis..."

# REGIONS - Extract all regional data
REGIONS=(
    "Aurora Bushes"
    "Black Shield Timberlands" 
    "Blood Blade Fields"
    "Bonecrusher Plains"
    "Darkfall Dunes"
    "Darkfall Plains"
    "Fallen Star Steppe"
    "Fearless Wilds"
    "Firefly Cliffs"
    "Goblinchaser Jungle"
    "Goblinchaser Wilderness"
    "Goldenswan Timberlands"
    "Goldseeker's Cliffs"
    "Grey Mist Snowlands"
    "Heartseeker Forest"
    "Heartseeker Moors"
    "Hell's Gate Desert"
    "Holloweye Wilderness"
    "Iceborn Wilderness"
    "Javelin Plains"
    "Javelin Wetlands"
    "Moonwatcher Wetlands"
    "Nightmare Desert"
    "Ragthorn Meadows"
    "Ragthorn Woods"
    "Thunderwave Woodlands"
    "Vicious Crags"
)

echo "📍 Extracting regions..."
for region in "${REGIONS[@]}"; do
    echo "  Processing: $region"
    safe_name=$(echo "$region" | tr ' ' '_' | tr '[:upper:]' '[:lower:]')
    
    sqlite3 "$HBF_DB" "SELECT value FROM Entities WHERE value LIKE '%$region%';" > "$OUTPUT_DIR/regions/${safe_name}.txt" 2>/dev/null || echo "No data for $region"
    
    # Count entities found
    count=$(wc -l < "$OUTPUT_DIR/regions/${safe_name}.txt")
    echo "    Found: $count entities"
done

# SETTLEMENTS - Extract settlement data  
SETTLEMENTS=(
    "Village of Ashamar"
    "Village of Balaal"
    "Town of Devilville" 
    "Village of Dokar"
    "Village of Dorith"
    "Village of Harad"
    "Village of Headbone"
    "City of Headsmen"
    "Village of Kothian"
    "City of Palemoon"
)

echo "🏘️  Extracting settlements..."
for settlement in "${SETTLEMENTS[@]}"; do
    echo "  Processing: $settlement"
    safe_name=$(echo "$settlement" | tr ' ' '_' | tr '[:upper:]' '[:lower:]')
    
    sqlite3 "$HBF_DB" "SELECT value FROM Entities WHERE value LIKE '%$settlement%';" > "$OUTPUT_DIR/settlements/${safe_name}.txt" 2>/dev/null || echo "No data for $settlement"
    
    count=$(wc -l < "$OUTPUT_DIR/settlements/${safe_name}.txt")
    echo "    Found: $count entities"
done

# FACTIONS - Extract faction data
FACTIONS=(
    "The Defiled Wolves"
    "The Fists Of Justice" 
    "The Red Snakes"
    "The Swords Of Justice"
    "The White Wyverns"
)

echo "⚔️  Extracting factions..."
for faction in "${FACTIONS[@]}"; do
    echo "  Processing: $faction"
    safe_name=$(echo "$faction" | tr ' ' '_' | tr '[:upper:]' '[:lower:]')
    
    sqlite3 "$HBF_DB" "SELECT value FROM Entities WHERE value LIKE '%$faction%';" > "$OUTPUT_DIR/factions/${safe_name}.txt" 2>/dev/null || echo "No data for $faction"
    
    count=$(wc -l < "$OUTPUT_DIR/factions/${safe_name}.txt")
    echo "    Found: $count entities"
done

# DUNGEONS - Extract dungeon data
DUNGEONS=(
    "Bowel of the Raging Pits"
    "Caverns of the Burning Souls"
    "Caverns of the Infernal Lich"
    "Crypt of the Corrupted Order"
    "Crypt of the Infernal Blades"
    "Crypt of the Mourning Goblin"
    "Crypt of the Unholy Goblin" 
    "Crypt of the Violent Ogre"
    "Hideout of the Corrupted Order"
    "Hideout of the Unspoken Desire"
    "Lair of the Foresaken Desire"
    "Lair of the Mourning Hopes"
    "Shrine of the Infernal Blades"
    "Shrine of the Infernal Desire"
    "Temple of the Violent Ogre"
    "Tomb of the Cursed Pits"
    "Tomb of the Grey Ogre"
    "Tomb of the Unspoken Skeletons"
)

echo "🏰 Extracting dungeons..."
for dungeon in "${DUNGEONS[@]}"; do
    echo "  Processing: $dungeon"
    safe_name=$(echo "$dungeon" | tr ' ' '_' | tr '[:upper:]' '[:lower:]')
    
    sqlite3 "$HBF_DB" "SELECT value FROM Entities WHERE value LIKE '%$dungeon%';" > "$OUTPUT_DIR/dungeons/${safe_name}.txt" 2>/dev/null || echo "No data for $dungeon"
    
    count=$(wc -l < "$OUTPUT_DIR/dungeons/${safe_name}.txt")
    echo "    Found: $count entities"
done

echo "✅ HBF worldbuilding extraction complete!"
echo "📁 Data organized in: $OUTPUT_DIR"
echo "🗺️  Next: Review region data to establish level bands and political boundaries"
