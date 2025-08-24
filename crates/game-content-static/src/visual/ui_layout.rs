//! UI layout parameters that adapt to dread levels

use crate::DreadLevel;
use glam::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UILayout {
    dread_level: DreadLevel,
}

impl UILayout {
    pub fn new(dread_level: DreadLevel) -> Self {
        Self { dread_level }
    }
    
    /// Base grid unit for all UI (in pixels)
    pub fn grid_unit(&self) -> f32 {
        // Grid becomes less regular with dread
        let base = 8.0;
        let chaos = self.dread_level.factor() * 2.0;
        base + chaos
    }
    
    /// Margin sizes
    pub fn margin(&self, size: MarginSize) -> f32 {
        let unit = self.grid_unit();
        let multiplier = match size {
            MarginSize::None => 0.0,
            MarginSize::Tiny => 0.5,
            MarginSize::Small => 1.0,
            MarginSize::Medium => 2.0,
            MarginSize::Large => 3.0,
            MarginSize::Huge => 4.0,
        };
        
        // Margins grow with dread (UI feels more cramped)
        unit * multiplier * (1.0 + self.dread_level.factor() * 0.3)
    }
    
    /// Padding inside elements
    pub fn padding(&self, size: PaddingSize) -> f32 {
        let unit = self.grid_unit();
        let multiplier = match size {
            PaddingSize::None => 0.0,
            PaddingSize::Tight => 0.5,
            PaddingSize::Normal => 1.0,
            PaddingSize::Comfortable => 1.5,
            PaddingSize::Spacious => 2.0,
        };
        
        // Padding shrinks with dread (text feels cramped)
        unit * multiplier * (1.0 - self.dread_level.factor() * 0.2)
    }
    
    /// Corner radius for rounded elements
    pub fn corner_radius(&self, element_type: &str) -> f32 {
        let base = match element_type {
            "button" => 4.0,
            "panel" => 8.0,
            "card" => 12.0,
            "tooltip" => 3.0,
            "dialogue_box" => 6.0,
            _ => 4.0,
        };
        
        // Corners become sharper with dread
        base * (1.0 - self.dread_level.factor() * 0.8)
    }
    
    /// Border width
    pub fn border_width(&self, style: BorderStyle) -> f32 {
        let base = match style {
            BorderStyle::None => 0.0,
            BorderStyle::Thin => 1.0,
            BorderStyle::Normal => 2.0,
            BorderStyle::Thick => 4.0,
            BorderStyle::Heavy => 6.0,
        };
        
        // Borders become heavier with dread
        base * (1.0 + self.dread_level.factor() * 0.5)
    }
    
    /// Element positions drift with dread
    pub fn position_offset(&self) -> Vec2 {
        let chaos = self.dread_level.factor();
        Vec2::new(
            (chaos * 5.0) * (fastrand::f32() - 0.5),
            (chaos * 5.0) * (fastrand::f32() - 0.5),
        )
    }
    
    /// Safe area insets (for mobile/notched displays)
    pub fn safe_area_inset(&self) -> SafeAreaInsets {
        SafeAreaInsets {
            top: 44.0 + self.margin(MarginSize::Medium),
            bottom: 34.0 + self.margin(MarginSize::Small),
            left: self.margin(MarginSize::Small),
            right: self.margin(MarginSize::Small),
        }
    }
    
    /// HUD element positions
    pub fn hud_position(&self, element: HudElement) -> HudPosition {
        match element {
            HudElement::HealthBar => HudPosition {
                anchor: Anchor::TopLeft,
                offset: Vec2::new(
                    self.margin(MarginSize::Medium),
                    self.margin(MarginSize::Medium),
                ),
                size: Vec2::new(200.0, 30.0),
            },
            HudElement::Inventory => HudPosition {
                anchor: Anchor::BottomRight,
                offset: Vec2::new(
                    -self.margin(MarginSize::Medium),
                    -self.margin(MarginSize::Medium),
                ),
                size: Vec2::new(300.0, 80.0),
            },
            HudElement::Minimap => HudPosition {
                anchor: Anchor::TopRight,
                offset: Vec2::new(
                    -self.margin(MarginSize::Medium),
                    self.margin(MarginSize::Medium),
                ),
                size: Vec2::new(150.0, 150.0),
            },
            HudElement::DialogueBox => HudPosition {
                anchor: Anchor::Bottom,
                offset: Vec2::new(
                    0.0,
                    -self.margin(MarginSize::Large),
                ),
                size: Vec2::new(600.0, 150.0),
            },
        }
    }
    
    /// Animation easing for UI transitions
    pub fn easing_curve(&self) -> EasingCurve {
        match self.dread_level.0 {
            0 => EasingCurve::EaseInOut,
            1 => EasingCurve::EaseOut,
            2 => EasingCurve::Linear,
            3 => EasingCurve::EaseIn,
            4 => EasingCurve::Bounce, // Erratic
            _ => EasingCurve::Linear,
        }
    }
    
    /// Z-index layering
    pub fn z_index(&self, layer: UILayer) -> i32 {
        match layer {
            UILayer::Background => 0,
            UILayer::GameWorld => 10,
            UILayer::HUD => 100,
            UILayer::Menus => 200,
            UILayer::Dialogue => 300,
            UILayer::Notifications => 400,
            UILayer::Debug => 999,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarginSize {
    None,
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaddingSize {
    None,
    Tight,
    Normal,
    Comfortable,
    Spacious,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderStyle {
    None,
    Thin,
    Normal,
    Thick,
    Heavy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HudElement {
    HealthBar,
    Inventory,
    Minimap,
    DialogueBox,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Anchor {
    TopLeft,
    Top,
    TopRight,
    Left,
    Center,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

#[derive(Debug, Clone, Copy)]
pub struct HudPosition {
    pub anchor: Anchor,
    pub offset: Vec2,
    pub size: Vec2,
}

#[derive(Debug, Clone, Copy)]
pub struct SafeAreaInsets {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EasingCurve {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Bounce,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UILayer {
    Background,
    GameWorld,
    HUD,
    Menus,
    Dialogue,
    Notifications,
    Debug,
}
