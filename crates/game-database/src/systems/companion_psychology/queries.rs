//! Companion Psychology System - Database Queries
//!
//! Production-ready database queries using SeaORM for companion psychology data,
//! therapy sessions, and recovery tracking with full type safety.

use sea_orm::{
    prelude::*, QueryFilter, QuerySelect, QueryOrder, Set, ActiveModelTrait,
    DatabaseConnection, DbErr, TransactionTrait,
};
use crate::router::DatabaseRouter;
use crate::engine::OperationType;
use crate::models::{companions, psychology};
use uuid::Uuid;
use serde_json::Value as JsonValue;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Query builder for companion psychology operations with intelligent routing
#[derive(Debug)]
pub struct CompanionPsychologyQueries {
    router: DatabaseRouter,
}

impl CompanionPsychologyQueries {
    pub fn new(router: DatabaseRouter) -> Self {
        Self { router }
    }
    
    pub fn from_database(db: DatabaseConnection) -> Self {
        // Legacy compatibility - this would need actual conversion
        // For now, this is a placeholder
        todo!("Convert DatabaseConnection to DatabaseRouter")
    }
    
    /// Load companion with their psychology data (uses intelligent routing)
    pub async fn load_companion_with_psychology(&self, companion_id: Uuid) -> Result<Option<(companions::Model, Option<psychology::Model>)>, DbErr> {
        // Companion data -> player.db (player's companions)
        let companion = self.router.route_query("companions", "query", |conn| async move {
            companions::Entity::find_by_id(companion_id)
                .one(&*conn.read().await)
                .await
                .map_err(|e| crate::error::DatabaseError::Query(e))
        }).await.map_err(|e| DbErr::Custom(format!("Router error: {}", e)))?;
        
        let companion = match companion {
            Some(c) => c,
            None => return Ok(None),
        };
        
        // Psychology data -> player.db (therapy progress)
        let psychology = self.router.route_query("psychology", "query", |conn| async move {
            psychology::Entity::find()
                .filter(psychology::Column::CompanionId.eq(companion_id))
                .one(&*conn.read().await)
                .await
                .map_err(|e| crate::error::DatabaseError::Query(e))
        }).await.map_err(|e| DbErr::Custom(format!("Router error: {}", e)))?;
        
        Ok(Some((companion, psychology)))
    }
    
    /// Load all active companions with psychology data for player (uses intelligent routing)
    pub async fn load_active_companions_psychology(&self, player_id: Uuid) -> Result<Vec<(companions::Model, Option<psychology::Model>)>, DbErr> {
        // Companions data -> player.db (player's active companions)
        let companions_data = self.router.route_query("companions", "query", |conn| async move {
            companions::Entity::find()
                .filter(companions::Column::PlayerId.eq(player_id))
                .filter(companions::Column::IsActive.eq(true))
                .all(&*conn.read().await)
                .await
                .map_err(|e| crate::error::DatabaseError::Query(e))
        }).await.map_err(|e| DbErr::Custom(format!("Router error: {}", e)))?;
        
        let mut results = Vec::new();
        
        for companion in companions_data {
            // Psychology data -> player.db (therapy progress for this companion)
            let psychology = self.router.route_query("psychology", "query", |conn| async move {
                psychology::Entity::find()
                    .filter(psychology::Column::CompanionId.eq(companion.id))
                    .one(&*conn.read().await)
                    .await
                    .map_err(|e| crate::error::DatabaseError::Query(e))
            }).await.map_err(|e| DbErr::Custom(format!("Router error: {}", e)))?;
            
            results.push((companion, psychology));
        }
        
        Ok(results)
    }
    
    /// Create or update companion trauma data
    pub async fn update_companion_trauma(&self, companion_id: Uuid, trauma_data: CompanionTraumaUpdate) -> Result<(), DbErr> {
        let txn = self.db.begin().await?;
        
        // Update companion model
        let companion = companions::Entity::find_by_id(companion_id)
            .one(&txn)
            .await?
            .ok_or_else(|| DbErr::Custom("Companion not found".to_string()))?;
        
        let mut companion: companions::ActiveModel = companion.into();
        companion.trauma_level = Set(trauma_data.new_trauma_level);
        companion.loyalty = Set(trauma_data.new_loyalty);
        companion.trust = Set(trauma_data.new_trust);
        companion.updated_at = Set(Utc::now());
        
        // Parse and update trauma sources JSON
        let trauma_sources_json = serde_json::to_value(&trauma_data.trauma_sources)
            .map_err(|e| DbErr::Custom(format!("Failed to serialize trauma sources: {}", e)))?;
        companion.trauma_sources = Set(trauma_sources_json);
        
        companion.update(&txn).await?;
        
        // Update or create psychology record
        let psychology_record = psychology::Entity::find()
            .filter(psychology::Column::CompanionId.eq(companion_id))
            .one(&txn)
            .await?;
        
        if let Some(existing) = psychology_record {
            // Update existing psychology record
            let mut psychology: psychology::ActiveModel = existing.into();
            
            psychology.trauma_triggers = Set(serde_json::to_value(&trauma_data.trauma_triggers)
                .map_err(|e| DbErr::Custom(format!("Failed to serialize trauma triggers: {}", e)))?);
            psychology.therapy_readiness = Set(trauma_data.therapy_readiness);
            psychology.breakthrough_potential = Set(trauma_data.breakthrough_potential);
            psychology.updated_at = Set(Utc::now());
            
            psychology.update(&txn).await?;
        } else {
            // Create new psychology record
            let new_psychology = psychology::ActiveModel {
                id: Set(Uuid::new_v4()),
                companion_id: Set(companion_id),
                player_id: Set(trauma_data.player_id),
                therapy_quest_id: Set("initial_assessment".to_string()),
                therapy_stage: Set("assessment".to_string()),
                therapy_progress: Set(0.0),
                total_therapy_quests_completed: Set(0),
                trauma_triggers: Set(serde_json::to_value(&trauma_data.trauma_triggers)
                    .map_err(|e| DbErr::Custom(format!("Failed to serialize trauma triggers: {}", e)))?),
                therapy_readiness: Set(trauma_data.therapy_readiness),
                breakthrough_potential: Set(trauma_data.breakthrough_potential),
                therapeutic_relationship_quality: Set(0.0),
                recovery_milestones: Set(serde_json::Value::Array(vec![])),
                created_at: Set(Utc::now()),
                updated_at: Set(Utc::now()),
                ..Default::default()
            };
            
            new_psychology.insert(&txn).await?;
        }
        
        txn.commit().await?;
        Ok(())
    }
    
    /// Record therapy session progress
    pub async fn record_therapy_session(&self, session_data: TherapySessionRecord) -> Result<Uuid, DbErr> {
        let txn = self.db.begin().await?;
        
        // Update psychology record with session progress
        let psychology = psychology::Entity::find()
            .filter(psychology::Column::CompanionId.eq(session_data.companion_id))
            .one(&txn)
            .await?
            .ok_or_else(|| DbErr::Custom("Psychology record not found".to_string()))?;
        
        let mut psychology: psychology::ActiveModel = psychology.into();
        
        // Update therapy progress
        psychology.therapy_progress = Set(session_data.progress_after_session);
        psychology.therapy_stage = Set(session_data.current_stage.clone());
        
        // Update conversation history
        let mut conversation_history: Vec<JsonValue> = 
            serde_json::from_value(psychology.therapeutic_conversation_history.clone().unwrap_or_default())
            .unwrap_or_default();
        
        conversation_history.push(serde_json::to_value(&session_data.session_summary)
            .map_err(|e| DbErr::Custom(format!("Failed to serialize session: {}", e)))?);
        
        psychology.therapeutic_conversation_history = Set(serde_json::to_value(&conversation_history)
            .map_err(|e| DbErr::Custom(format!("Failed to serialize conversation history: {}", e)))?);
        
        // Update breakthrough potential based on session
        psychology.breakthrough_potential = Set(session_data.breakthrough_potential_after);
        psychology.therapeutic_relationship_quality = Set(session_data.relationship_quality_after);
        psychology.updated_at = Set(Utc::now());
        
        psychology.update(&txn).await?;
        
        // If session completed a therapy quest, increment completion count
        if session_data.quest_completed {
            let mut psychology_update: psychology::ActiveModel = 
                psychology::Entity::find_by_id(psychology.id.unwrap())
                .one(&txn)
                .await?
                .ok_or_else(|| DbErr::Custom("Psychology record not found after update".to_string()))?
                .into();
            
            psychology_update.total_therapy_quests_completed = 
                Set(psychology_update.total_therapy_quests_completed.unwrap() + 1);
            psychology_update.update(&txn).await?;
        }
        
        txn.commit().await?;
        Ok(session_data.session_id)
    }
    
    /// Record recovery milestone
    pub async fn record_recovery_milestone(&self, companion_id: Uuid, milestone: RecoveryMilestone) -> Result<(), DbErr> {
        let psychology = psychology::Entity::find()
            .filter(psychology::Column::CompanionId.eq(companion_id))
            .one(&self.db)
            .await?
            .ok_or_else(|| DbErr::Custom("Psychology record not found".to_string()))?;
        
        let mut psychology: psychology::ActiveModel = psychology.into();
        
        // Parse existing milestones
        let mut milestones: Vec<JsonValue> = 
            serde_json::from_value(psychology.recovery_milestones.clone().unwrap_or_default())
            .unwrap_or_default();
        
        // Add new milestone
        milestones.push(serde_json::to_value(&milestone)
            .map_err(|e| DbErr::Custom(format!("Failed to serialize milestone: {}", e)))?);
        
        psychology.recovery_milestones = Set(serde_json::to_value(&milestones)
            .map_err(|e| DbErr::Custom(format!("Failed to serialize milestones: {}", e)))?);
        psychology.updated_at = Set(Utc::now());
        
        psychology.update(&self.db).await?;
        
        Ok(())
    }
    
    /// Get companion trauma history
    pub async fn get_trauma_history(&self, companion_id: Uuid) -> Result<Option<TraumaHistory>, DbErr> {
        let companion = companions::Entity::find_by_id(companion_id)
            .one(&self.db)
            .await?;
        
        let companion = match companion {
            Some(c) => c,
            None => return Ok(None),
        };
        
        let psychology = psychology::Entity::find()
            .filter(psychology::Column::CompanionId.eq(companion_id))
            .one(&self.db)
            .await?;
        
        let trauma_sources: Vec<TraumaSourceData> = 
            serde_json::from_value(companion.trauma_sources.clone())
            .unwrap_or_default();
        
        let trauma_triggers: Vec<String> = if let Some(psychology) = &psychology {
            serde_json::from_value(psychology.trauma_triggers.clone().unwrap_or_default())
                .unwrap_or_default()
        } else {
            vec![]
        };
        
        let recovery_milestones: Vec<RecoveryMilestone> = if let Some(psychology) = &psychology {
            serde_json::from_value(psychology.recovery_milestones.clone().unwrap_or_default())
                .unwrap_or_default()
        } else {
            vec![]
        };
        
        Ok(Some(TraumaHistory {
            companion_id,
            current_trauma_level: companion.trauma_level,
            trauma_sources,
            trauma_triggers,
            recovery_milestones,
            therapy_progress: psychology.as_ref().map(|p| p.therapy_progress).unwrap_or(0.0),
            last_therapy_session: psychology.as_ref().map(|p| p.updated_at.timestamp()),
            breaking_point_reached: companion.trauma_level >= companion.breaking_point,
        }))
    }
    
    /// Get therapy session statistics
    pub async fn get_therapy_statistics(&self, companion_id: Uuid) -> Result<Option<TherapyStatistics>, DbErr> {
        let psychology = psychology::Entity::find()
            .filter(psychology::Column::CompanionId.eq(companion_id))
            .one(&self.db)
            .await?;
        
        let psychology = match psychology {
            Some(p) => p,
            None => return Ok(None),
        };
        
        let conversation_history: Vec<JsonValue> = 
            serde_json::from_value(psychology.therapeutic_conversation_history.clone().unwrap_or_default())
            .unwrap_or_default();
        
        let session_count = conversation_history.len();
        
        let recovery_milestones: Vec<RecoveryMilestone> = 
            serde_json::from_value(psychology.recovery_milestones.clone().unwrap_or_default())
            .unwrap_or_default();
        
        let breakthrough_count = recovery_milestones.iter()
            .filter(|m| m.milestone_type.contains("breakthrough"))
            .count();
        
        let average_session_effectiveness = if session_count > 0 {
            // This would be calculated from actual session data
            psychology.therapeutic_relationship_quality * 0.8 + psychology.therapy_progress * 0.2
        } else {
            0.0
        };
        
        Ok(Some(TherapyStatistics {
            companion_id,
            total_sessions: session_count,
            completed_quests: psychology.total_therapy_quests_completed,
            breakthroughs_achieved: breakthrough_count,
            current_therapy_stage: psychology.therapy_stage,
            overall_progress: psychology.therapy_progress,
            therapeutic_relationship_quality: psychology.therapeutic_relationship_quality,
            average_session_effectiveness,
            most_recent_session: Some(psychology.updated_at.timestamp()),
            recommended_next_steps: vec![
                if psychology.therapy_progress < 0.3 {
                    "Continue assessment and trust building".to_string()
                } else if psychology.therapy_progress < 0.7 {
                    "Begin trauma processing work".to_string()
                } else {
                    "Focus on integration and resilience building".to_string()
                }
            ],
        }))
    }
    
    /// Get companions at risk (high trauma, low support)
    pub async fn get_companions_at_risk(&self, player_id: Uuid) -> Result<Vec<CompanionRiskAssessment>, DbErr> {
        let companions_data = companions::Entity::find()
            .filter(companions::Column::PlayerId.eq(player_id))
            .filter(companions::Column::IsActive.eq(true))
            .all(&self.db)
            .await?;
        
        let mut at_risk_companions = Vec::new();
        
        for companion in companions_data {
            // High trauma level
            if companion.trauma_level >= companion.breaking_point * 0.8 {
                let psychology = psychology::Entity::find()
                    .filter(psychology::Column::CompanionId.eq(companion.id))
                    .one(&self.db)
                    .await?;
                
                let support_quality = psychology.as_ref()
                    .and_then(|p| serde_json::from_value::<f32>(p.peer_support_effectiveness.clone().unwrap_or_default()).ok())
                    .unwrap_or(0.0);
                
                let risk_factors = vec![
                    if companion.trauma_level > companion.breaking_point * 0.9 {
                        "Critical trauma level".to_string()
                    } else {
                        "High trauma level".to_string()
                    },
                    if companion.trust < 0.3 {
                        "Very low trust".to_string()
                    } else if companion.trust < 0.5 {
                        "Low trust".to_string()
                    } else {
                        "Trust issues developing".to_string()
                    },
                    if support_quality < 0.3 {
                        "Inadequate support network".to_string()
                    } else {
                        "Declining support quality".to_string()
                    },
                ];
                
                let immediate_interventions = vec![
                    if companion.trauma_level >= companion.breaking_point {
                        "Emergency crisis intervention needed".to_string()
                    } else {
                        "Intensive therapy sessions recommended".to_string()
                    },
                    "Professional support evaluation".to_string(),
                    "Enhanced peer support network".to_string(),
                    "Safe environment provision".to_string(),
                ];
                
                at_risk_companions.push(CompanionRiskAssessment {
                    companion_id: companion.id,
                    companion_name: companion.name,
                    risk_level: if companion.trauma_level >= companion.breaking_point { 1.0 } else { 0.8 },
                    trauma_level: companion.trauma_level,
                    breaking_point: companion.breaking_point,
                    trust_level: companion.trust,
                    loyalty_level: companion.loyalty,
                    support_network_quality: support_quality,
                    risk_factors,
                    immediate_interventions,
                    estimated_time_to_crisis: if companion.trauma_level >= companion.breaking_point {
                        Some(0)
                    } else {
                        Some(((companion.breaking_point - companion.trauma_level) * 86400.0) as i64) // Rough estimate in seconds
                    },
                });
            }
        }
        
        // Sort by risk level
        at_risk_companions.sort_by(|a, b| b.risk_level.partial_cmp(&a.risk_level).unwrap());
        
        Ok(at_risk_companions)
    }
    
    /// Search companions by psychological criteria
    pub async fn search_companions_by_psychology(&self, player_id: Uuid, criteria: PsychologySearchCriteria) -> Result<Vec<companions::Model>, DbErr> {
        let mut query = companions::Entity::find()
            .filter(companions::Column::PlayerId.eq(player_id));
        
        // Filter by trauma level range
        if let Some(min_trauma) = criteria.min_trauma_level {
            query = query.filter(companions::Column::TraumaLevel.gte(min_trauma));
        }
        if let Some(max_trauma) = criteria.max_trauma_level {
            query = query.filter(companions::Column::TraumaLevel.lte(max_trauma));
        }
        
        // Filter by trust level
        if let Some(min_trust) = criteria.min_trust_level {
            query = query.filter(companions::Column::Trust.gte(min_trust));
        }
        
        // Filter by loyalty level
        if let Some(min_loyalty) = criteria.min_loyalty_level {
            query = query.filter(companions::Column::Loyalty.gte(min_loyalty));
        }
        
        // Filter by active status
        if let Some(is_active) = criteria.is_active {
            query = query.filter(companions::Column::IsActive.eq(is_active));
        }
        
        // Filter by companion type
        if let Some(companion_type) = &criteria.companion_type {
            query = query.filter(companions::Column::CompanionType.eq(companion_type.clone()));
        }
        
        let mut results = query.all(&self.db).await?;
        
        // Additional filtering that requires psychology data
        if criteria.has_professional_support.is_some() || 
           criteria.therapy_stage.is_some() ||
           criteria.min_therapy_progress.is_some() {
            
            let mut filtered_results = Vec::new();
            
            for companion in results {
                let psychology = psychology::Entity::find()
                    .filter(psychology::Column::CompanionId.eq(companion.id))
                    .one(&self.db)
                    .await?;
                
                let mut include = true;
                
                if let Some(psychology) = &psychology {
                    if let Some(required_stage) = &criteria.therapy_stage {
                        if &psychology.therapy_stage != required_stage {
                            include = false;
                        }
                    }
                    
                    if let Some(min_progress) = criteria.min_therapy_progress {
                        if psychology.therapy_progress < min_progress {
                            include = false;
                        }
                    }
                }
                
                if include {
                    filtered_results.push(companion);
                }
            }
            
            results = filtered_results;
        }
        
        Ok(results)
    }
}

// Data structures for queries

#[derive(Debug, Clone)]
pub struct CompanionTraumaUpdate {
    pub player_id: Uuid,
    pub new_trauma_level: f32,
    pub new_loyalty: f32,
    pub new_trust: f32,
    pub trauma_sources: Vec<TraumaSourceData>,
    pub trauma_triggers: Vec<String>,
    pub therapy_readiness: f32,
    pub breakthrough_potential: f32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TraumaSourceData {
    pub source_id: String,
    pub trauma_type: String,
    pub severity: f32,
    pub acquisition_context: String,
    pub active_triggers: Vec<String>,
    pub healing_progress: f32,
}

#[derive(Debug, Clone)]
pub struct TherapySessionRecord {
    pub session_id: Uuid,
    pub companion_id: Uuid,
    pub therapy_type: String,
    pub current_stage: String,
    pub progress_after_session: f32,
    pub breakthrough_potential_after: f32,
    pub relationship_quality_after: f32,
    pub session_summary: TherapySessionSummary,
    pub quest_completed: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TherapySessionSummary {
    pub session_date: i64,
    pub duration_minutes: f32,
    pub activities_performed: Vec<String>,
    pub emotional_progress: f32,
    pub insights_gained: Vec<String>,
    pub breakthrough_moments: Vec<String>,
    pub homework_assigned: Vec<String>,
    pub next_session_focus: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RecoveryMilestone {
    pub milestone_id: String,
    pub milestone_type: String,
    pub achievement_date: i64,
    pub significance: f32,
    pub description: String,
    pub celebration_enacted: bool,
}

#[derive(Debug, Clone)]
pub struct TraumaHistory {
    pub companion_id: Uuid,
    pub current_trauma_level: f32,
    pub trauma_sources: Vec<TraumaSourceData>,
    pub trauma_triggers: Vec<String>,
    pub recovery_milestones: Vec<RecoveryMilestone>,
    pub therapy_progress: f32,
    pub last_therapy_session: Option<i64>,
    pub breaking_point_reached: bool,
}

#[derive(Debug, Clone)]
pub struct TherapyStatistics {
    pub companion_id: Uuid,
    pub total_sessions: usize,
    pub completed_quests: i32,
    pub breakthroughs_achieved: usize,
    pub current_therapy_stage: String,
    pub overall_progress: f32,
    pub therapeutic_relationship_quality: f32,
    pub average_session_effectiveness: f32,
    pub most_recent_session: Option<i64>,
    pub recommended_next_steps: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CompanionRiskAssessment {
    pub companion_id: Uuid,
    pub companion_name: String,
    pub risk_level: f32,
    pub trauma_level: f32,
    pub breaking_point: f32,
    pub trust_level: f32,
    pub loyalty_level: f32,
    pub support_network_quality: f32,
    pub risk_factors: Vec<String>,
    pub immediate_interventions: Vec<String>,
    pub estimated_time_to_crisis: Option<i64>,
}

#[derive(Debug, Clone, Default)]
pub struct PsychologySearchCriteria {
    pub min_trauma_level: Option<f32>,
    pub max_trauma_level: Option<f32>,
    pub min_trust_level: Option<f32>,
    pub min_loyalty_level: Option<f32>,
    pub is_active: Option<bool>,
    pub companion_type: Option<String>,
    pub has_professional_support: Option<bool>,
    pub therapy_stage: Option<String>,
    pub min_therapy_progress: Option<f32>,
}
