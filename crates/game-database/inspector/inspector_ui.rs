//! Inspector UI module for visual validation of AI-generated assets
//! Provides human-in-the-loop review interface with dread progression preview

use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::egui;
use egui_dock::{DockArea, DockState, NodeIndex, Style, TabViewer};
use crate::{
    AssetMetadata, AssetSource, GeneratedAssetsDatabase, 
    InspectorUiState, ValidationStatus, PerformanceMetrics
};
use std::collections::HashMap;

/// Inspector window tabs
#[derive(Debug, Clone, PartialEq)]
pub enum InspectorTab {
    AssetDatabase,
    ValidationQueue,
    DreadPreview,
    PerformanceMetrics,
    AgentStatus,
    ExportQueue,
}

/// Tab viewer for the inspector dock
pub struct InspectorTabViewer<'a> {
    pub database: &'a mut GeneratedAssetsDatabase,
    pub ui_state: &'a mut InspectorUiState,
    pub asset_server: &'a AssetServer,
}

impl TabViewer for InspectorTabViewer<'_> {
    type Tab = InspectorTab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            InspectorTab::AssetDatabase => "Asset Database".into(),
            InspectorTab::ValidationQueue => {
                let count = self.database.approval_queue.len();
                format!("Validation Queue ({})", count).into()
            }
            InspectorTab::DreadPreview => "Dread Preview".into(),
            InspectorTab::PerformanceMetrics => "Performance".into(),
            InspectorTab::AgentStatus => "AI Agents".into(),
            InspectorTab::ExportQueue => "Export Queue".into(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            InspectorTab::AssetDatabase => asset_database_ui(ui, self.database, self.ui_state),
            InspectorTab::ValidationQueue => validation_queue_ui(ui, self.database, self.ui_state),
            InspectorTab::DreadPreview => dread_preview_ui(ui, self.database, self.ui_state),
            InspectorTab::PerformanceMetrics => performance_metrics_ui(ui, self.database),
            InspectorTab::AgentStatus => agent_status_ui(ui, self.database),
            InspectorTab::ExportQueue => export_queue_ui(ui, self.database),
        }
    }
}

/// Main inspector UI system
pub fn inspector_ui_system(
    mut contexts: Query<&mut bevy_inspector_egui::bevy_egui::EguiContext>,
    mut database: ResMut<GeneratedAssetsDatabase>,
    mut ui_state: ResMut<InspectorUiState>,
    asset_server: Res<AssetServer>,
) {
    let Ok(mut context) = contexts.get_single_mut() else { return };
    
    egui::Window::new("Dragon's Labyrinth Assets Inspector")
        .default_size([1200.0, 800.0])
        .show(context.get_mut(), |ui| {
            // Create tabbed interface
            let mut dock_state = DockState::new(vec![
                InspectorTab::AssetDatabase,
                InspectorTab::ValidationQueue,
            ]);
            
            // Add additional tabs
            dock_state.push_to_focused_leaf(InspectorTab::DreadPreview);
            dock_state.push_to_focused_leaf(InspectorTab::PerformanceMetrics);
            dock_state.push_to_focused_leaf(InspectorTab::AgentStatus);
            dock_state.push_to_focused_leaf(InspectorTab::ExportQueue);
            
            let mut tab_viewer = InspectorTabViewer {
                database: &mut database,
                ui_state: &mut ui_state,
                asset_server: &asset_server,
            };
            
            DockArea::new(&mut dock_state)
                .style(Style::from_egui(ui.style().as_ref()))
                .show_inside(ui, &mut tab_viewer);
        });
}

/// Asset database browser UI
fn asset_database_ui(
    ui: &mut egui::Ui,
    database: &mut GeneratedAssetsDatabase,
    ui_state: &mut InspectorUiState,
) {
    ui.heading("AI-Generated Assets Database");
    ui.separator();
    
    // Filters
    ui.horizontal(|ui| {
        ui.label("Filter by:");
        
        egui::ComboBox::from_label("Category")
            .selected_text(ui_state.filter_category.as_deref().unwrap_or("All"))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut ui_state.filter_category, None, "All");
                ui.selectable_value(&mut ui_state.filter_category, Some("hex_tiles".to_string()), "Hex Tiles");
                ui.selectable_value(&mut ui_state.filter_category, Some("companions".to_string()), "Companions");
                ui.selectable_value(&mut ui_state.filter_category, Some("ui".to_string()), "UI Elements");
                ui.selectable_value(&mut ui_state.filter_category, Some("audio".to_string()), "Audio");
            });
        
        egui::ComboBox::from_label("Agent")
            .selected_text(ui_state.filter_agent.as_deref().unwrap_or("All"))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut ui_state.filter_agent, None, "All");
                ui.selectable_value(&mut ui_state.filter_agent, Some("MapsAgent".to_string()), "MapsAgent");
                ui.selectable_value(&mut ui_state.filter_agent, Some("LevelsAgent".to_string()), "LevelsAgent");
                ui.selectable_value(&mut ui_state.filter_agent, Some("UIAgent".to_string()), "UIAgent");
                ui.selectable_value(&mut ui_state.filter_agent, Some("DialogueAgent".to_string()), "DialogueAgent");
                ui.selectable_value(&mut ui_state.filter_agent, Some("AudioAgent".to_string()), "AudioAgent");
            });
    });
    
    ui.separator();
    
    // Asset list
    egui::ScrollArea::vertical().show(ui, |ui| {
        for ((dread_level, category), assets) in &database.asset_index {
            // Apply filters
            if let Some(ref filter_cat) = ui_state.filter_category {
                if category != filter_cat {
                    continue;
                }
            }
            
            let header = format!("Dread {} - {}", dread_level, category);
            ui.collapsing(header, |ui| {
                for asset in assets {
                    // Apply agent filter
                    if let Some(ref filter_agent) = ui_state.filter_agent {
                        if &asset.generation_agent != filter_agent {
                            continue;
                        }
                    }
                    
                    asset_item_ui(ui, asset, ui_state);
                }
            });
        }
    });
}

/// Individual asset item UI
fn asset_item_ui(
    ui: &mut egui::Ui,
    asset: &AssetMetadata,
    ui_state: &mut InspectorUiState,
) {
    ui.horizontal(|ui| {
        // Selection checkbox
        let selected = ui_state.selected_asset.as_ref() == Some(&asset.id);
        if ui.checkbox(&mut selected.clone(), "").clicked() {
            ui_state.selected_asset = if selected {
                Some(asset.id.clone())
            } else {
                None
            };
        }
        
        // Asset ID and status
        ui.label(&asset.id);
        
        // Source badge
        let source_color = match asset.source {
            AssetSource::Core => egui::Color32::GOLD,
            AssetSource::Library => egui::Color32::LIGHT_BLUE,
            AssetSource::Generated => egui::Color32::LIGHT_GREEN,
            AssetSource::Hybrid => egui::Color32::YELLOW,
        };
        ui.colored_label(source_color, format!("{:?}", asset.source));
        
        // Validation status badge
        let status_color = match asset.validation_status {
            ValidationStatus::Generated => egui::Color32::GRAY,
            ValidationStatus::Loaded => egui::Color32::LIGHT_GRAY,
            ValidationStatus::Validated => egui::Color32::LIGHT_BLUE,
            ValidationStatus::Approved => egui::Color32::GREEN,
            ValidationStatus::Rejected => egui::Color32::RED,
            ValidationStatus::Error(_) => egui::Color32::DARK_RED,
        };
        ui.colored_label(status_color, format!("{:?}", asset.validation_status));
        
        // Performance score
        let score_color = if asset.performance_score > 0.8 {
            egui::Color32::GREEN
        } else if asset.performance_score > 0.5 {
            egui::Color32::YELLOW
        } else {
            egui::Color32::RED
        };
        ui.colored_label(score_color, format!("Perf: {:.1}", asset.performance_score));
    });
}

/// Validation queue UI for human review
fn validation_queue_ui(
    ui: &mut egui::Ui,
    database: &mut GeneratedAssetsDatabase,
    ui_state: &mut InspectorUiState,
) {
    ui.heading("Assets Awaiting Human Approval");
    ui.separator();
    
    if database.approval_queue.is_empty() {
        ui.label("No assets pending approval");
        return;
    }
    
    ui.label(format!("{} assets pending approval", database.approval_queue.len()));
    ui.separator();
    
    // Keyboard shortcuts help
    ui.horizontal(|ui| {
        ui.label("Shortcuts:");
        ui.colored_label(egui::Color32::GREEN, "A - Approve");
        ui.colored_label(egui::Color32::RED, "R - Reject");
        ui.colored_label(egui::Color32::LIGHT_BLUE, "Space - Cycle Dread");
        ui.colored_label(egui::Color32::YELLOW, "N - Next Asset");
    });
    ui.separator();
    
    // Current asset being reviewed
    if let Some(current_id) = database.approval_queue.first() {
        ui.label(format!("Reviewing: {}", current_id));
        
        // Find the asset metadata
        for assets in database.asset_index.values() {
            if let Some(asset) = assets.iter().find(|a| &a.id == current_id) {
                ui.group(|ui| {
                    ui.label(format!("Category: {}", asset.category));
                    ui.label(format!("Agent: {}", asset.generation_agent));
                    ui.label(format!("Dread Level: {}", asset.dread_level));
                    ui.label(format!("File: {}", asset.file_path.display()));
                });
                
                ui.separator();
                
                // Approval buttons
                ui.horizontal(|ui| {
                    if ui.button("✅ Approve (A)").clicked() {
                        approve_asset(database, current_id);
                    }
                    
                    if ui.button("❌ Reject (R)").clicked() {
                        reject_asset(database, current_id);
                    }
                    
                    if ui.button("⏭ Skip (N)").clicked() {
                        // Move to next asset without decision
                        if let Some(id) = database.approval_queue.remove(0) {
                            database.approval_queue.push(id);
                        }
                    }
                });
            }
        }
    }
}

/// Dread progression preview UI
fn dread_preview_ui(
    ui: &mut egui::Ui,
    database: &GeneratedAssetsDatabase,
    ui_state: &mut InspectorUiState,
) {
    ui.heading("Horror Progression Preview");
    ui.separator();
    
    // Dread level selector
    ui.horizontal(|ui| {
        ui.label("Current Dread Level:");
        for level in 0..=4 {
            let label = match level {
                0 => "Peace",
                1 => "Unease",
                2 => "Dread",
                3 => "Terror",
                4 => "Horror",
                _ => "Unknown",
            };
            
            if ui.selectable_label(
                ui_state.current_dread_preview == level,
                format!("{} ({})", label, level)
            ).clicked() {
                ui_state.current_dread_preview = level;
            }
        }
    });
    
    ui.separator();
    
    // Show assets for current dread level
    ui.label(format!("Assets for Dread Level {}", ui_state.current_dread_preview));
    
    egui::ScrollArea::vertical().show(ui, |ui| {
        for (key, assets) in &database.asset_index {
            if key.0 == ui_state.current_dread_preview {
                ui.collapsing(&key.1, |ui| {
                    for asset in assets {
                        ui.label(format!("• {} ({:?})", asset.id, asset.validation_status));
                    }
                });
            }
        }
    });
    
    ui.separator();
    
    // Dread transition preview
    ui.label("Press SPACE to cycle through dread variants in 3D view");
}

/// Performance metrics UI
fn performance_metrics_ui(
    ui: &mut egui::Ui,
    database: &GeneratedAssetsDatabase,
) {
    ui.heading("Performance Metrics");
    ui.separator();
    
    // Overall statistics
    let total_assets = database.validation_status.len();
    let approved = database.validation_status.values()
        .filter(|s| matches!(s, ValidationStatus::Approved))
        .count();
    let mobile_compatible = database.performance_metrics.values()
        .filter(|m| m.mobile_compatible)
        .count();
    
    ui.horizontal(|ui| {
        ui.label(format!("Total Assets: {}", total_assets));
        ui.label(format!("Approved: {}", approved));
        ui.label(format!("Mobile Compatible: {}", mobile_compatible));
    });
    
    ui.separator();
    
    // Performance breakdown by category
    let mut category_metrics: HashMap<String, Vec<&PerformanceMetrics>> = HashMap::new();
    
    for (asset_id, metrics) in &database.performance_metrics {
        // Find asset category
        for assets in database.asset_index.values() {
            if let Some(asset) = assets.iter().find(|a| &a.id == asset_id) {
                category_metrics
                    .entry(asset.category.clone())
                    .or_insert_with(Vec::new)
                    .push(metrics);
            }
        }
    }
    
    for (category, metrics) in category_metrics {
        ui.collapsing(category, |ui| {
            if !metrics.is_empty() {
                let avg_vertices = metrics.iter()
                    .map(|m| m.vertex_count as f32)
                    .sum::<f32>() / metrics.len() as f32;
                let avg_memory = metrics.iter()
                    .map(|m| m.memory_usage_mb)
                    .sum::<f32>() / metrics.len() as f32;
                let avg_fps_impact = metrics.iter()
                    .map(|m| m.fps_impact)
                    .sum::<f32>() / metrics.len() as f32;
                
                ui.label(format!("Avg Vertices: {:.0}", avg_vertices));
                ui.label(format!("Avg Memory: {:.1} MB", avg_memory));
                ui.label(format!("Avg FPS Impact: {:.1}", avg_fps_impact));
            }
        });
    }
}

/// AI agent status UI
fn agent_status_ui(
    ui: &mut egui::Ui,
    database: &GeneratedAssetsDatabase,
) {
    ui.heading("AI Agent Status");
    ui.separator();
    
    let agents = ["MapsAgent", "LevelsAgent", "UIAgent", "DialogueAgent", "AudioAgent"];
    
    for agent in agents {
        ui.collapsing(agent, |ui| {
            // Count assets by this agent
            let mut generated = 0;
            let mut validated = 0;
            let mut approved = 0;
            
            for assets in database.asset_index.values() {
                for asset in assets {
                    if asset.generation_agent == agent {
                        generated += 1;
                        if matches!(asset.validation_status, ValidationStatus::Validated | ValidationStatus::Approved) {
                            validated += 1;
                        }
                        if matches!(asset.validation_status, ValidationStatus::Approved) {
                            approved += 1;
                        }
                    }
                }
            }
            
            ui.label(format!("Generated: {}", generated));
            ui.label(format!("Validated: {}", validated));
            ui.label(format!("Approved: {}", approved));
            
            let approval_rate = if generated > 0 {
                (approved as f32 / generated as f32) * 100.0
            } else {
                0.0
            };
            ui.label(format!("Approval Rate: {:.1}%", approval_rate));
        });
    }
}

/// Export queue UI
fn export_queue_ui(
    ui: &mut egui::Ui,
    database: &GeneratedAssetsDatabase,
) {
    ui.heading("Export Queue");
    ui.separator();
    
    // Count approved assets ready for export
    let ready_for_export: Vec<_> = database.validation_status.iter()
        .filter(|(_, status)| matches!(status, ValidationStatus::Approved))
        .map(|(id, _)| id.clone())
        .collect();
    
    ui.label(format!("{} assets ready for export", ready_for_export.len()));
    ui.separator();
    
    if ui.button("Export All Approved Assets").clicked() {
        // Trigger export (handled by export system)
        info!("Exporting {} approved assets", ready_for_export.len());
    }
    
    ui.separator();
    
    // List assets ready for export
    egui::ScrollArea::vertical().show(ui, |ui| {
        for asset_id in ready_for_export {
            ui.label(format!("• {}", asset_id));
        }
    });
}

/// Human review system with keyboard shortcuts
pub fn human_review_system(
    mut database: ResMut<GeneratedAssetsDatabase>,
    mut ui_state: ResMut<InspectorUiState>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    // Only process in approval mode
    if !ui_state.approval_mode {
        if keyboard.just_pressed(KeyCode::Tab) {
            ui_state.approval_mode = true;
            info!("Entered approval mode");
        }
        return;
    }
    
    // Exit approval mode
    if keyboard.just_pressed(KeyCode::Escape) {
        ui_state.approval_mode = false;
        info!("Exited approval mode");
        return;
    }
    
    // Process keyboard shortcuts
    if let Some(current_id) = database.approval_queue.first().cloned() {
        if keyboard.just_pressed(KeyCode::KeyA) {
            // Approve current asset
            approve_asset(&mut database, &current_id);
            info!("Approved asset: {}", current_id);
        }
        
        if keyboard.just_pressed(KeyCode::KeyR) {
            // Reject current asset
            reject_asset(&mut database, &current_id);
            info!("Rejected asset: {}", current_id);
        }
        
        if keyboard.just_pressed(KeyCode::KeyN) {
            // Next asset without decision
            database.approval_queue.remove(0);
            if !database.approval_queue.is_empty() {
                database.approval_queue.push(current_id);
            }
            info!("Skipped to next asset");
        }
    }
    
    // Cycle dread preview
    if keyboard.just_pressed(KeyCode::Space) {
        ui_state.current_dread_preview = (ui_state.current_dread_preview + 1) % 5;
        info!("Dread preview level: {}", ui_state.current_dread_preview);
    }
}

/// Preview dread variants in 3D view
pub fn preview_dread_variants(
    ui_state: Res<InspectorUiState>,
    database: Res<GeneratedAssetsDatabase>,
    mut query: Query<(&mut Visibility, &AssetMetadata)>,
) {
    // Show only assets for current dread preview level
    for (mut visibility, asset) in query.iter_mut() {
        *visibility = if asset.dread_level == ui_state.current_dread_preview {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

// Helper functions
fn approve_asset(database: &mut GeneratedAssetsDatabase, asset_id: &str) {
    database.validation_status.insert(
        asset_id.to_string(),
        ValidationStatus::Approved,
    );
    database.approval_queue.retain(|id| id != asset_id);
    
    // Update in database
    if let Ok(conn) = database.connection.lock() {
        let _ = conn.execute(
            "UPDATE generated_assets SET validation_status = 'Approved', human_approved = TRUE WHERE id = ?1",
            [asset_id],
        );
    }
}

fn reject_asset(database: &mut GeneratedAssetsDatabase, asset_id: &str) {
    database.validation_status.insert(
        asset_id.to_string(),
        ValidationStatus::Rejected,
    );
    database.approval_queue.retain(|id| id != asset_id);
    
    // Update in database
    if let Ok(conn) = database.connection.lock() {
        let _ = conn.execute(
            "UPDATE generated_assets SET validation_status = 'Rejected', human_approved = FALSE WHERE id = ?1",
            [asset_id],
        );
    }
}
