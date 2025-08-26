# Path & Bridge Prompts (Consistency-Locked, Tileable Overlays)

These prompts are tailored for **top-down 2.5D hex tile overlays** in *Dragon's Labyrinth*.  
They enforce flat aerial perspective, constant width, and transparent backgrounds.

---

## Global Constraints (apply to all prompts)
- **Flat aerial top-down view**, no perspective, no tilt, no horizon  
- **1024×1024 transparent PNG**, seamless edge-to-edge overlay  
- **Width ~25% of canvas**, consistent across all paths  
- **Soft feathered edges** that fade to transparency (blend with biome tiles)  
- **No borders, no raised curbs, no shadows, no rails, no hardware**  
- **Style**: Medieval fantasy / dark fantasy as dread increases  

---

## Roads & Trails

### 1. Stone Road — Straight
> Centered straight cobblestone road strip running from top edge to bottom edge, constant width ~25% canvas, flat aerial top-down, seamless overlay, soft feathered edges to transparency, medieval fantasy, transparent background, no borders, no perspective, no shadows

### 2. Stone Road — Endcap
> Cobblestone road endcap: centered semicircle matching road width, fades to transparency, flat aerial top-down, medieval fantasy overlay, transparent background, no perspective, no shadows

### 3. Stone Road — Junction Node
> Cobblestone road junction node: round patch matching road width, slightly wider, soft feather to transparency for blending three roads, flat aerial top-down, medieval fantasy overlay, transparent background

---

### 4. Dirt Trail — Straight
> Centered straight dirt trail strip (packed earth with small pebbles) from top edge to bottom edge, ~25% canvas width, flat aerial top-down, seamless overlay, soft feathered edges, medieval fantasy, transparent background, no ruts, no perspective, no shadows

### 5. Dirt Trail — Endcap
> Dirt trail endcap: rounded finish matching trail width, fading to transparency, flat aerial top-down, medieval fantasy overlay, transparent background

### 6. Dirt Trail — Junction Node
> Dirt trail junction node: widened circular patch of packed earth, soft feather to transparency for three-way join, flat aerial top-down, medieval fantasy overlay, transparent background

---

### 7. Wooden Plank Road — Straight
> Straight wooden-plank road strip running top to bottom, planks perpendicular to travel direction, ~25% width, flat aerial top-down, seamless overlay, soft feather edges, medieval fantasy, transparent background, no perspective

### 8. Wooden Plank Road — Endcap
> Wooden-plank road endcap: rounded staggered plank ends, fading to transparency, flat aerial top-down, medieval fantasy overlay, transparent background

### 9. Wooden Plank Road — Junction Node
> Wooden-plank road junction node: compact patch of planks arranged radially, soft feather edges to transparency, flat aerial top-down, medieval fantasy overlay, transparent background

---

### 10. Brick Road — Straight
> Straight red-brick road strip, herringbone pattern, top to bottom, ~25% width, flat aerial top-down, seamless overlay, soft feather edges, medieval fantasy, transparent background, no perspective

### 11. Brick Road — Endcap
> Brick road endcap: rounded finish, brick pattern continues, fades to transparency, flat aerial top-down, medieval fantasy overlay, transparent background

### 12. Brick Road — Junction Node
> Brick road junction node: widened circular patch of bricks arranged radially, blending three roads, flat aerial top-down, medieval fantasy overlay, transparent background

---

## Degraded / Cursed / Nightmare Variants
For any road or trail above, append:  
- **Degraded**: “weathered, cracked, missing pieces, subtle weeds in gaps”  
- **Cursed**: “dark twisted version, blackened surface, unnatural glow, corrupted medieval dark fantasy”  
- **Nightmare**: “completely ruined, chaotic broken fragments, bone shards or molten cracks, horror fantasy overlay”  

Keep all base constraints (flat top-down, overlay, transparency).

---

## Bridges (Path Types over Obstacles)

### 13. Stone Bridge
> Stone arch bridge top-down overlay: flat cobblestone strip, ~25% canvas width, seamless edge-to-edge, soft transparency blend, medieval fantasy, transparent background, no shadows, no side walls

### 14. Wooden Bridge
> Wooden plank bridge top-down overlay: straight planks with rope lash hints, ~25% width, seamless edge-to-edge, soft transparency blend, medieval fantasy, transparent background, no shadows, no perspective

### 15. Coral Bridge (natural over shallow water)
> Coral-encrusted natural bridge strip, pale reef textures with sand and shell flecks, ~25% width, seamless edge-to-edge, soft transparency blend, flat aerial top-down, medieval fantasy, transparent background, no shadows

### 16. Ice Bridge (natural over icy terrain)
> Frozen ice bridge strip, frosted blue-white cracked ice, ~25% width, seamless edge-to-edge, soft transparency blend, flat aerial top-down, medieval fantasy, transparent background

### 17. Natural Rock Bridge
> Rugged natural rock bridge strip, uneven stone slabs and lichen patches, ~25% width, seamless edge-to-edge, soft transparency blend, flat aerial top-down, medieval fantasy, transparent background

---

## Usage Notes
- Each path type should have at least **3 assets**: straight, endcap, junction node.  
- Bridges can use just **straight edge-to-edge overlays** (rotate for placement).  
- Overlay opacity can be tuned in the engine for blending【9†source】.  
- Roads/bridges remain modular — combine endcaps and junctions to build networks.

