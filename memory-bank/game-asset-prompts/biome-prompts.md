# Biome Prompts (Consistency-Locked, Tileable)

These prompts are tailored for **top-down 2.5D hex tiles** in *Dragon's Labyrinth*. They hard-lock camera, framing, and tiling to avoid circular crops, perspective tilt, or illustrative borders.

---

## Global Constraints (append to every prompt)
- **Seamless edge-to-edge terrain texture**, **flat aerial top-down view**
- **Square 1024x1024 tileable**, **game-ready tile**, **no border**, **no frame**
- **No circles, no symbols, no portals**, **no horizon**, **no perspective tilt**
- **Medieval dark fantasy color palette** (bright → darker as dread increases)

> You can paste the full prompts below as-is. They already include the global constraints.

---

## Base Biomes (Dread 0)

### plains.png
**Prompt**
> Lush green grassland with scattered wildflowers and small patches of clover, soft variation in grass density without paths or objects, seamless edge-to-edge terrain texture, flat aerial top-down view, medieval fantasy tile, square 1024x1024 tileable, game-ready, no border, no frame, no circles, no horizon, no perspective tilt

### forest.png
**Prompt**
> Dense healthy forest canopy made of layered tree crowns (top-down foliage shapes), subtle gaps of darker understory, no trunks, no paths, seamless edge-to-edge terrain texture, flat aerial top-down view, medieval fantasy tile, square 1024x1024 tileable, game-ready, no border, no frame, no circles, no horizon, no perspective tilt

### mountain.png
**Prompt**
> Gray rocky mountain terrain seen from above: broken stone plates, gravel scree fans, light height cues via micro shading only, no cliffs, no isometric slopes, seamless edge-to-edge terrain texture, flat aerial top-down view, medieval fantasy tile, square 1024x1024 tileable, game-ready, no border, no frame, no circles, no horizon, no perspective tilt

### jagged_rock.png
**Prompt**
> Sharp dangerous rock shards and fractured ground, angular stone fragments and dark crevices, hazardous feel without vertical walls, seamless edge-to-edge terrain texture, flat aerial top-down view, medieval fantasy tile, square 1024x1024 tileable, game-ready, no border, no frame, no circles, no horizon, no perspective tilt

### snow.png
**Prompt**
> Clean white snow surface with gentle wind-carved ripples and subtle blue shadows, few exposed icy specks, no trees, no footprints, seamless edge-to-edge terrain texture, flat aerial top-down view, medieval fantasy tile, square 1024x1024 tileable, game-ready, no border, no frame, no circles, no horizon, no perspective tilt

### swamp.png
**Prompt**
> Murky marsh ground with dark peat patches, reeds in top-down clusters, shallow water gleam in irregular puddles, algae film variation, seamless edge-to-edge terrain texture, flat aerial top-down view, medieval fantasy tile, square 1024x1024 tileable, game-ready, no border, no frame, no circles, no horizon, no perspective tilt

### desert.png
**Prompt**
> Golden sandy desert with small wind-formed dunes and ripples, occasional coarse grains and tiny pebbles, warm light, seamless edge-to-edge terrain texture, flat aerial top-down view, medieval fantasy tile, square 1024x1024 tileable, game-ready, no border, no frame, no circles, no horizon, no perspective tilt

### coast.png
**Prompt**
> Sandy shoreline meeting shallow clear water from above: foamy fringe band, wet darker sand transition, subtle shells and pebbles, seamless edge-to-edge terrain texture, flat aerial top-down view, medieval fantasy tile, square 1024x1024 tileable, game-ready, no border, no frame, no circles, no horizon, no perspective tilt

### ocean.png
**Prompt**
> Deep blue seawater viewed from above, layered waves and foam patterns, subtle caustic variation, no boats, no islands, seamless edge-to-edge terrain texture, flat aerial top-down view, medieval fantasy tile, square 1024x1024 tileable, game-ready, no border, no frame, no circles, no horizon, no perspective tilt

---

## Corrupted Biomes (Dread 1–2)

### plains_cursed.png
**Prompt**
> Withered yellow-brown grassland with dry tufts and bare soil scars, faint shadowy stains, ominous desaturation, seamless edge-to-edge terrain texture, flat aerial top-down view, medieval dark fantasy tile, square 1024x1024 tileable, game-ready, no border, no frame, no circles, no horizon, no perspective tilt

### forest_cursed.png
**Prompt**
> Blackened twisted tree crowns (top-down canopy) with sparse dead gaps, soot-stained understory, subtle ash dusting, seamless edge-to-edge terrain texture, flat aerial top-down view, medieval dark fantasy tile, square 1024x1024 tileable, game-ready, no border, no frame, no circles, no horizon, no perspective tilt

### mountain_cursed.png
**Prompt**
> Dark foreboding rock plates with oily sheen and deep hairline fractures, cold desaturated stone, seamless edge-to-edge terrain texture, flat aerial top-down view, medieval dark fantasy tile, square 1024x1024 tileable, game-ready, no border, no frame, no circles, no horizon, no perspective tilt

### snow_cursed.png
**Prompt**
> Gray tainted snow with dirty slush veins and scattered soot specks, cold greenish shadow hints, seamless edge-to-edge terrain texture, flat aerial top-down view, medieval dark fantasy tile, square 1024x1024 tileable, game-ready, no border, no frame, no circles, no horizon, no perspective tilt

### swamp_cursed.png
**Prompt**
> Putrid black swampwater with filament algae and stagnant oil-like film, rotting reed clusters from above, bone fragments suggested, seamless edge-to-edge terrain texture, flat aerial top-down view, medieval dark fantasy tile, square 1024x1024 tileable, game-ready, no border, no frame, no circles, no horizon, no perspective tilt

### desert_cursed.png
**Prompt**
> Black glassy sand with jagged obsidian grains and scorched patches, faint ember flecks, heat-singed look, seamless edge-to-edge terrain texture, flat aerial top-down view, medieval dark fantasy tile, square 1024x1024 tileable, game-ready, no border, no frame, no circles, no horizon, no perspective tilt

---

## Nightmare / Extreme (Dread 3–4)

### plains_nightmare.png
**Prompt**
> Dead cracked earth with bone-dry plates, tiny bone shards and ash flecks, oppressive desaturation, seamless edge-to-edge terrain texture, flat aerial top-down view, medieval horror fantasy tile, square 1024x1024 tileable, game-ready, no border, no frame, no circles, no horizon, no perspective tilt

### forest_nightmare.png
**Prompt**
> Hellish scorched canopy silhouettes (top-down) with glowing ember seams and drifting ash specks, deep crimson shadows, seamless edge-to-edge terrain texture, flat aerial top-down view, medieval horror fantasy tile, square 1024x1024 tileable, game-ready, no border, no frame, no circles, no horizon, no perspective tilt

### lava.png
**Prompt**
> Molten lava terrain seen from above: cracked dark basalt plates with bright magma glowing in channels, heat bloom suggestion, seamless edge-to-edge terrain texture, flat aerial top-down view, medieval fantasy tile, square 1024x1024 tileable, game-ready, no border, no frame, no circles, no horizon, no perspective tilt

### void_terrain.png
**Prompt**
> Dark slate ground fractured by faint violet void fissures and subtle non-Euclidean misalignments in grain, eerie star-dust specks, grounded as terrain (not a portal), seamless edge-to-edge terrain texture, flat aerial top-down view, medieval eldritch fantasy tile, square 1024x1024 tileable, game-ready, no border, no frame, no circles, no horizon, no perspective tilt

---

## Optional Variants (use sparingly for diversity)

- **wet_plains**: dew-darkened patches and trampled muddy arcs, same constraints.  
- **rocky_plains**: interspersed small stones and thin dirt bands, same constraints.  
- **snow_dirty**: wind-exposed dirt streaks and granular crust, same constraints.  
- **desert_rocky**: scattered gravel pans and firmer compacted sand, same constraints.

---

## Notes
- Keep **features and paths as overlays**; biomes must remain object-free.
- If the model persists in circular crops, append: **“do not crop to a circle; fill the entire square edge-to-edge.”**
- For stronger repetition safety across hex seams, add: **“subtle low-frequency variation; avoid large distinct shapes.”**
