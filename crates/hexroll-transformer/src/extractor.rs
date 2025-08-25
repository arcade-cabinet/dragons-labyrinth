use regex::Regex;
use scraper::{Html, Selector, ElementRef};
use std::collections::HashMap;
use crate::models::*;

// ------------------------------------------------------------------------------------
// Utility helpers
// ------------------------------------------------------------------------------------
fn sel(s: &str) -> Option<Selector> { Selector::parse(s).ok() }
fn text(node: &ElementRef) -> String { node.text().collect::<String>().trim().to_string() }
fn norm(s: &str) -> String { Regex::new(r"\s+").unwrap().replace_all(s.trim(), " ").into_owned() }
fn parse_int(s: &str) -> Option<i64> {
    let re = Regex::new(r"([0-9][0-9,]*)").ok()?;
    let caps = re.captures(s)?;
    caps.get(1).map(|m| m.as_str().replace(',', "").parse().ok()).flatten()
}

fn has_words(hay: &str, words: &[&str]) -> bool {
    let l = hay.to_lowercase();
    words.iter().all(|w| l.contains(&w.to_lowercase()))
}

fn first_text(doc: &Html, selector: &str) -> Option<String> {
    let s = sel(selector)?;
    doc.select(&s).next().map(|n| text(&n))
}

fn labelled_value(doc: &Html, label: &str) -> Option<String> {
    // Common in these exports: <span class="section-label">Armor Class:</span> 15 (leather)
    let s = sel("span.section-label")?;
    for span in doc.select(&s) {
        if text(&span).trim_end_matches(':').eq_ignore_ascii_case(label) {
            let mut out = String::new();
            // Capture immediate following text siblings and inline elements
            let parent = span.parent()?;
            let parent = ElementRef::wrap(parent)?;
            out = parent.text().collect::<String>();
            // Remove the label prefix
            let out = out.replace(label, "");
            let out = out.trim().trim_matches(':').trim().to_string();
            if !out.is_empty() { return Some(norm(&out)); }
        }
    }
    None
}

// Ability table: find a table that contains STR/DEX/CON/INT/WIS/CHA
fn extract_abilities(doc: &Html) -> Option<[i8;6]> {
    let s_table = sel("table")?;
    for table in doc.select(&s_table) {
        let t = table.text().collect::<String>();
        if ["STR","DEX","CON","INT","WIS","CHA"].iter().all(|k| t.contains(k)) {
            let re = Regex::new(r"STR\s*([0-9]+)|DEX\s*([0-9]+)|CON\s*([0-9]+)|INT\s*([0-9]+)|WIS\s*([0-9]+)|CHA\s*([0-9]+)").ok()?;
            let mut vals = [0i8;6];
            for cap in re.captures_iter(&t) {
                for (i, m) in cap.iter().skip(1).enumerate() {
                    if let Some(m) = m { vals[i] = m.as_str().parse::<i8>().unwrap_or(0) }
                }
            }
            // Fallback: parse order by columns
            return Some(vals);
        }
    }
    None
}

fn list_after_heading(doc: &Html, heading: &str) -> Vec<String> {
    // Find heading tag whose text equals `heading`, then capture first following <ul><li>…</li></ul>
    let mut out = vec![];
    for level in ["h1","h2","h3","h4","h5","h6"] {
        if let Some(sel_h) = sel(level) {
            for h in doc.select(&sel_h) {
                if text(&h).trim().eq_ignore_ascii_case(heading) {
                    if let Some(ul_sel) = sel("ul") {
                        if let Some(ul) = h.next_siblings().filter_map(ElementRef::wrap).find(|n| n.value().name() == "ul") {
                            for li in ul.select(&sel("li").unwrap()) { out.push(text(&li)); }
                            return out;
                        }
                        // fallback: next <ul> anywhere after
                        for ul in doc.select(&ul_sel) { for li in ul.select(&sel("li").unwrap()) { out.push(text(&li)); } return out; }
                    }
                }
            }
        }
    }
    out
}

// ------------------------------------------------------------------------------------
// Existing functions (kept)
// ------------------------------------------------------------------------------------
pub fn extract_doc_title(html: &str) -> Option<String> {
    let sel = Selector::parse("#doc-title").ok()?;
    let doc = Html::parse_document(html);
    doc.select(&sel).next().map(|n| n.text().collect::<String>().trim().to_string())
}

pub fn detect_page_type(html: &str) -> PageType {
    // More nuanced detection using title first
    let title_l = extract_doc_title(html).unwrap_or_default().to_lowercase();
    let l = html.to_lowercase();

    // Specific subtypes first
    if title_l.contains("village") { return PageType::Village; }
    if title_l.contains("town")    { return PageType::Town; }
    if title_l.contains("city")    { return PageType::City; }

    if title_l.contains("cave")    || l.contains(" cave ")    { return PageType::Cave; }
    if title_l.contains("temple")  || l.contains(" temple ")  { return PageType::Temple; }
    if title_l.contains("tomb")    || l.contains(" tomb ")    { return PageType::Tomb; }

    if title_l.contains("inn") || l.contains(" inn ") { return PageType::Inn; }

    if title_l.contains("farm") || title_l.contains("cabin") || l.contains(" stronghold ") {
        // Disambiguate dwelling kinds
        if title_l.contains("stronghold") || l.contains(" stronghold ") { return PageType::Stronghold; }
        return PageType::FarmsCabins;
    }

    if l.contains(" faction ") || l.contains(" cult ") || l.contains(" militia ") || l.contains(" syndicate ") {
        return PageType::Faction;
    }

    if html.contains("Hex ") { return PageType::Hex; }
    if l.contains("rumor") && html.contains("<table") { return PageType::RumorTable; }
    if l.contains("str") && l.contains("dex") && l.contains("cha") { return PageType::NPC; }
    if l.contains("dungeon") { return PageType::Dungeon; }
    if l.contains("shop") { return PageType::Shop; }
    if l.contains("biome") { return PageType::Biome; }
    if l.contains("region") { return PageType::Region; }

    // Generic settlement fallback
    if l.contains("village") || l.contains("city") || l.contains("town") { return PageType::Settlement; }
    PageType::Unknown
}

pub fn extract_hex_info(html: &str) -> Option<HexPage> {
    let title = extract_doc_title(html);
    let coord = title.as_ref().and_then(|t| {
        let re = Regex::new(r"Hex\s+([A-Z]?\d+)").ok()?;
        re.captures(t).and_then(|c| c.get(1)).map(|m| m.as_str().to_string())
    });
    let biome = {
        // naive: look for the word 'Desert', 'Forest', 'Hills', etc.
        let words = ["Desert","Forest","Jungle","Hills","Swamp","Plains","Mountains","Wasteland","Coast","Tundra"];
        let lower = html.to_lowercase();
        words.iter().find(|w| lower.contains(&w.to_lowercase())).map(|s| s.to_string())
    };
    coord.map(|c| HexPage{ coord: c, biome, title, region: None, description: None })
}

pub fn parse_rumor_table(html: &str) -> Option<RumorTable> {
    let doc = Html::parse_document(html);
    let sel_table = Selector::parse("table").ok()?;
    let mut out = vec![];
    for table in doc.select(&sel_table) {
        let text = table.text().collect::<String>();
        if text.to_lowercase().contains("rumor") {
            // get li or td lines
            let sel_li = Selector::parse("li").ok()?;
            for li in table.select(&sel_li) {
                let line = li.text().collect::<String>().trim().to_string();
                if !line.is_empty() { out.push(line); }
            }
        }
    }
    if out.is_empty() { None } else { Some(RumorTable{ entries: out }) }
}

pub fn parse_weather_table(html: &str) -> Option<WeatherTable> {
    let doc = Html::parse_document(html);
    let sel_table = Selector::parse("table").ok()?;
    let mut rows = vec![];
    for table in doc.select(&sel_table) {
        let header = table.text().collect::<String>().to_lowercase();
        if header.contains("dry") && header.contains("wet") {
            // naive row parse
            let sel_tr = Selector::parse("tr").ok()?;
            let sel_td = Selector::parse("td").ok()?;
            for tr in table.select(&sel_tr).skip(1) {
                let cells: Vec<String> = tr.select(&sel_td).map(|td| td.text().collect::<String>().trim().to_string()).collect();
                if cells.len() >= 3 {
                    rows.push(WeatherRow{ roll: cells[0].clone(), dry: cells[1].clone(), wet: cells[2].clone() });
                }
            }
        }
    }
    if rows.is_empty() { None } else { Some(WeatherTable{ rows, flood_one_in_6_weekly: header_mentions_weekly_flood(html) }) }
}

fn header_mentions_weekly_flood(html: &str) -> bool {
    html.to_lowercase().contains("1-in-6") || html.to_lowercase().contains("one-in-six")
}

// ------------------------------------------------------------------------------------
// New Detected* result structs (local to extractor to avoid breaking existing models)
// ------------------------------------------------------------------------------------
#[derive(Debug, Clone, Default)]
pub struct DetectedRegion {
    pub name: String,
    pub hex_count: Option<i64>,
    pub biome_counts: HashMap<String, i64>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct DetectedBiome { pub name: String, pub description: Option<String> }

#[derive(Debug, Clone, Default)]
pub struct DetectedSettlement { pub name: String, pub kind: String, pub population: Option<i64>, pub description: Option<String> }

#[derive(Debug, Clone, Default)]
pub struct DetectedShop { pub name: String, pub kind: String }

#[derive(Debug, Clone, Default)]
pub struct DetectedInn { pub name: String, pub description: Option<String>, pub has_healer: Option<bool> }

#[derive(Debug, Clone, Default)]
pub struct DetectedDwelling { pub kind: String, pub name: Option<String>, pub description: Option<String> }

#[derive(Debug, Clone, Default)]
pub struct DetectedDungeon { pub name: String, pub kind: String, pub description: Option<String> }

#[derive(Debug, Clone, Default)]
pub struct DetectedMonster { pub name: String, pub cr: Option<String>, pub ac: Option<String>, pub hp: Option<String>, pub speed: Option<String>, pub abilities: Option<[i8;6]> }

#[derive(Debug, Clone, Default)]
pub struct DetectedFaction { pub name: String, pub kind: String, pub description: Option<String>, pub member_names: Vec<String> }

#[derive(Debug, Clone, Default)]
pub struct DetectedNpc {
    pub name: String,
    pub description: Option<String>,
    pub level: Option<i64>,
    pub ac: Option<String>,
    pub hp: Option<String>,
    pub speed: Option<String>,
    pub abilities: Option<[i8;6]>,
    pub senses: Vec<String>,
    pub languages: Vec<String>,
    pub actions: Vec<String>,
}

// ------------------------------------------------------------------------------------
// Region & Biome extraction
// ------------------------------------------------------------------------------------
pub fn extract_region_summary(html: &str) -> Option<DetectedRegion> {
    let doc = Html::parse_document(html);
    let title = extract_doc_title(html).unwrap_or_default();
    // Heuristic: Region name likely present in doc title
    let name = if title.to_lowercase().contains("region") { title } else { title.clone() };

    // Try to find a table with biome counts; headers contain "Biome" and "Hex"
    let mut biome_counts = HashMap::new();
    if let Some(t_sel) = sel("table") { for table in doc.select(&t_sel) {
        let head = table.text().collect::<String>().to_lowercase();
        if head.contains("biome") && (head.contains("hex") || head.contains("count")) {
            let tr = sel("tr").unwrap(); let td = sel("td").unwrap();
            for row in table.select(&tr).skip(1) {
                let cells: Vec<String> = row.select(&td).map(|td| text(&td)).collect();
                if cells.len() >= 2 { if let Some(n) = parse_int(&cells[1]) { biome_counts.insert(cells[0].clone(), n); } }
            }
        }
    }}

    // Hex count: look for a number near the word "hexes"
    let mut hex_count = None;
    let lower = html.to_lowercase();
    if lower.contains("hexes") { hex_count = parse_int(&html); }

    if name.is_empty() && biome_counts.is_empty() { return None; }
    Some(DetectedRegion { name, hex_count, biome_counts, description: None })
}

pub fn extract_biome_info(html: &str) -> Option<DetectedBiome> {
    let title = extract_doc_title(html).unwrap_or_default();
    if title.is_empty() { return None; }
    Some(DetectedBiome { name: title, description: None })
}

// ------------------------------------------------------------------------------------
// Settlement, Inn, Dwelling and Shops
// ------------------------------------------------------------------------------------
pub fn extract_settlement_info(html: &str) -> Option<DetectedSettlement> {
    let title = extract_doc_title(html).unwrap_or_default();
    if title.is_empty() { return None; }
    let l = title.to_lowercase();
    let kind = if l.contains("city") { "City" } else if l.contains("town") { "Town" } else { "Village" };

    let doc = Html::parse_document(html);
    // Population: look for "Population" label or a bare number followed by inhabitants/citizens
    let pop = labelled_value(&doc, "Population").and_then(|s| parse_int(&s)).or_else(|| parse_int(&html));
    Some(DetectedSettlement { name: title, kind: kind.to_string(), population: pop, description: None })
}

pub fn extract_shops(html: &str) -> Vec<DetectedShop> {
    let mut out = vec![];
    let doc = Html::parse_document(html);
    // Pattern seen: <h4><span id="editable-title">Name</span> (<em>Type</em>)</h4>
    if let Some(h4) = sel("h4") { for n in doc.select(&h4) {
        let t = n.text().collect::<String>();
        if t.contains("(") && t.contains(")") {
            let re = Regex::new(r"^(?s)(?P<name>.+?)\s*\(\s*(?P<kind>[^)]+)\s*\)").unwrap();
            if let Some(c) = re.captures(&t) {
                let name = c.name("name").map(|m| m.as_str().trim().to_string()).unwrap_or_else(|| "Shop".into());
                let kind = c.name("kind").map(|m| m.as_str().trim().to_string()).unwrap_or_else(|| "Shop".into());
                out.push(DetectedShop { name, kind });
            }
        }
    }}
    out
}

pub fn extract_inn_info(html: &str) -> Option<DetectedInn> {
    let title = extract_doc_title(html).unwrap_or_default();
    if title.is_empty() { return None; }
    let has_healer = html.to_lowercase().contains("healer") || html.to_lowercase().contains("healing");
    Some(DetectedInn { name: title, description: None, has_healer: Some(has_healer) })
}

pub fn extract_dwelling_info(html: &str) -> Option<DetectedDwelling> {
    let title = extract_doc_title(html).unwrap_or_default();
    if title.is_empty() { return None; }
    let l = title.to_lowercase();
    let kind = if l.contains("stronghold") { "Stronghold" } else { "FarmsCabins" };
    Some(DetectedDwelling { kind: kind.to_string(), name: Some(title), description: None })
}

// ------------------------------------------------------------------------------------
// Dungeons and Monsters
// ------------------------------------------------------------------------------------
pub fn extract_dungeon_info(html: &str) -> Option<DetectedDungeon> {
    let title = extract_doc_title(html).unwrap_or_default();
    if title.is_empty() { return None; }
    let l = title.to_lowercase();
    let kind = if l.contains("cave") { "Cave" } else if l.contains("temple") { "Temple" } else if l.contains("tomb") { "Tomb" } else { "Dungeon" };
    Some(DetectedDungeon { name: title, kind: kind.to_string(), description: None })
}

pub fn extract_monsters(html: &str) -> Vec<DetectedMonster> {
    let mut out = vec![];
    let doc = Html::parse_document(html);

    // Heuristic: each stat block often includes these labels; we’ll treat the whole page as one monster if found
    let ac = labelled_value(&doc, "Armor Class");
    let hp = labelled_value(&doc, "Hit Points").or_else(|| labelled_value(&doc, "HP"));
    let speed = labelled_value(&doc, "Speed");
    let abilities = extract_abilities(&doc);
    if ac.is_some() || hp.is_some() || abilities.is_some() {
        let name = extract_doc_title(html).unwrap_or_else(|| "Unknown Monster".into());
        out.push(DetectedMonster { name, cr: labelled_value(&doc, "Challenge"), ac, hp, speed, abilities });
        return out;
    }

    // Fallback: look for repeating subheaders and treat each as a monster name (rare in these exports)
    out
}

// ------------------------------------------------------------------------------------
// NPCs
// ------------------------------------------------------------------------------------
pub fn extract_npc(html: &str) -> Option<DetectedNpc> {
    let doc = Html::parse_document(html);
    // Title as name
    let name = extract_doc_title(html).unwrap_or_default();
    if name.is_empty() { return None; }

    let ac = labelled_value(&doc, "Armor Class");
    let hp = labelled_value(&doc, "Hit Points").or_else(|| labelled_value(&doc, "HP"));
    let speed = labelled_value(&doc, "Speed");
    let abilities = extract_abilities(&doc);

    // Senses / Languages headings
    let senses = list_after_heading(&doc, "Senses");
    let languages = list_after_heading(&doc, "Languages");
    let actions = list_after_heading(&doc, "Actions");

    // Level: scan text for "Level: N" or "level N"
    let level = Regex::new(r"(?i)level[:\s]+([0-9]+)").ok()
        .and_then(|re| re.captures(html).and_then(|c| c.get(1)))
        .and_then(|m| m.as_str().parse::<i64>().ok());

    Some(DetectedNpc { name, description: None, level, ac, hp, speed, abilities, senses, languages, actions })
}

// ------------------------------------------------------------------------------------
// Factions
// ------------------------------------------------------------------------------------
pub fn extract_faction(html: &str) -> Option<DetectedFaction> {
    let name = extract_doc_title(html).unwrap_or_default();
    if name.is_empty() { return None; }
    let l = html.to_lowercase();
    let kind = if l.contains("cult") { "Cult" } else if l.contains("militia") { "Militia" } else if l.contains("syndicate") { "Syndicate" } else { "Faction" };

    // Members list: look for list items following a heading that contains "Members"
    let doc = Html::parse_document(html);
    let mut members = list_after_heading(&doc, "Members");
    if members.is_empty() {
        // fallback: scan for "Member of <Name>" style phrases (yields names poorly; heuristic)
        let re = Regex::new(r"(?i)member[s]?\s+of\s+([A-Z][A-Za-z'\- ]+)").unwrap();
        for cap in re.captures_iter(html) { members.push(cap.get(1).map(|m| m.as_str().to_string()).unwrap_or_default()); }
        members.dedup();
    }

    Some(DetectedFaction { name, kind: kind.to_string(), description: None, member_names: members })
}

// ------------------------------------------------------------------------------------
// Biome & Region convenience detectors (lightweight)
// ------------------------------------------------------------------------------------
pub fn guess_region_name(html: &str) -> Option<String> {
    let t = extract_doc_title(html)?;
    let l = t.to_lowercase();
    if l.contains("region") { Some(t) } else { None }
}

pub fn guess_biome_name(html: &str) -> Option<String> {
    let words = ["Desert","Forest","Jungle","Hills","Swamp","Plains","Mountains","Wasteland","Coast","Tundra"];
    let l = html.to_lowercase();
    words.iter().find(|w| l.contains(&w.to_lowercase())).map(|s| s.to_string())
}