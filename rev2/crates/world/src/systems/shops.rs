use bevy::prelude::*;
use serde::Deserialize;
use crate::resources::{Stats, award_xp, Abilities, check_unlocks, LightDark};

#[derive(Debug, Clone, Deserialize)] struct ShopItem { item: String, cost: u32 }
#[derive(Debug, Clone, Deserialize)] struct Shop { id: String, name: String, inventory: Vec<ShopItem> }
#[derive(Debug, Clone, Deserialize)] struct ShopsFile { shops: Vec<Shop> }

#[derive(Resource, Default)] pub struct CurrentShop(pub Option<Shop>);
#[derive(Component)] pub struct ShopUIRoot;

pub fn shop_toggle_ui(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut cur: ResMut<CurrentShop>,
    q_ui: Query<Entity, With<ShopUIRoot>>,
    mut stats: ResMut<Stats>,
    mut abilities: ResMut<Abilities>,
    mut align: ResMut<LightDark>,
) {
    if keys.just_pressed(KeyCode::KeyT) {
        if cur.0.is_some() {
            for e in q_ui.iter() { commands.entity(e).despawn_recursive(); }
            cur.0 = None;
        } else {
            if let Ok(text) = std::fs::read_to_string("build/features/shops.json") {
                if let Ok(mut sf) = serde_json::from_str::<ShopsFile>(&text) {
                    if align.dark > align.light {
                        for s in &mut sf.shops { s.inventory.retain(|i| i.item != "bread"); s.inventory.push(ShopItem{ item:"iron_shard".into(), cost:6 }); }
                    }
                    if let Some(shop) = sf.shops.get(0).cloned() {
                        cur.0 = Some(shop.clone());
                        spawn_shop_ui(&mut commands, &shop);
                        award_xp(&mut stats, "craft", 1);
                        check_unlocks(&stats, &mut abilities);
                    }
                }
            }
        }
    }
}

fn spawn_shop_ui(commands: &mut Commands, shop: &Shop) {
    let mut root = commands.spawn((NodeBundle{
        style: Style {
            width: Val::Percent(40.0), height: Val::Percent(50.0),
            position_type: PositionType::Absolute, right: Val::Px(20.0), top: Val::Px(20.0),
            flex_direction: FlexDirection::Column, ..Default::default()
        },
        background_color: BackgroundColor(Color::rgba(0.05,0.05,0.06,0.9)), ..Default::default()
    }, ShopUIRoot));

    root.with_children(|p| {
        p.spawn(TextBundle::from_section(shop.name.clone(), TextStyle{ font: Default::default(), font_size: 20.0, color: Color::ORANGE_RED }));
        for it in &shop.inventory {
            p.spawn(TextBundle::from_section(format!("- {} ({})", it.item, it.cost), TextStyle{ font: Default::default(), font_size: 16.0, color: Color::WHITE }));
        }
        p.spawn(TextBundle::from_section("[T] close", TextStyle{ font: Default::default(), font_size: 14.0, color: Color::GRAY }));
    });
}
