//! Primitive narrative structures that get converted to YarnSpinner/Cobweb
//!
//! AI generates these clean data structures, then we convert them to specific formats

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A narrative spanning tree - the core structure AI generates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeTree {
    pub root: NodeId,
    pub nodes: HashMap<NodeId, NarrativeNode>,
    pub edges: Vec<NarrativeEdge>,
}

pub type NodeId = String;

/// A single node in the narrative graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeNode {
    pub id: NodeId,
    pub content: NodeContent,
    pub metadata: NodeMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeContent {
    Dialogue {
        speaker: String,
        text: String,
        emotion: String,
    },
    Choice {
        prompt: String,
        options: Vec<ChoiceOption>,
    },
    Action {
        description: String,
        effects: Vec<String>,
    },
    Condition {
        check: String,
        true_branch: NodeId,
        false_branch: Option<NodeId>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChoiceOption {
    pub text: String,
    pub requirements: Vec<String>,
    pub consequences: Vec<Consequence>,
    pub next_node: NodeId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Consequence {
    pub type_: ConsequenceType,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsequenceType {
    SetVariable,
    ModifyTrust,
    ModifyDread,
    TriggerEvent,
    UnlockPath,
    LockPath,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeMetadata {
    pub dread_level: u8,
    pub context: String,
    pub tags: Vec<String>,
    pub weight: f32, // For prioritizing paths
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeEdge {
    pub from: NodeId,
    pub to: NodeId,
    pub condition: Option<String>,
    pub weight: f32,
}

impl NarrativeTree {
    /// Convert this tree to YarnSpinner format
    pub fn to_yarnspinner(&self) -> String {
        let mut yarn = String::new();
        
        // Generate title block
        yarn.push_str(&format!("title: {}\n", self.root));
        yarn.push_str("tags:\n");
        yarn.push_str("colorID: 0\n");
        yarn.push_str("position: 0,0\n");
        yarn.push_str("---\n");
        
        // Convert nodes to Yarn format
        self.write_node_yarn(&self.root, &mut yarn, &mut Vec::new());
        
        yarn.push_str("===\n");
        yarn
    }
    
    fn write_node_yarn(&self, node_id: &str, output: &mut String, visited: &mut Vec<String>) {
        if visited.contains(&node_id.to_string()) {
            return; // Avoid cycles
        }
        visited.push(node_id.to_string());
        
        let node = &self.nodes[node_id];
        
        match &node.content {
            NodeContent::Dialogue { speaker, text, emotion } => {
                output.push_str(&format!("{}: {} [{}]\n", speaker, text, emotion));
                
                // Find next nodes
                for edge in &self.edges {
                    if edge.from == node_id {
                        self.write_node_yarn(&edge.to, output, visited);
                    }
                }
            }
            
            NodeContent::Choice { prompt, options } => {
                output.push_str(&format!("{}\n", prompt));
                
                for option in options {
                    output.push_str(&format!("-> {}\n", option.text));
                    
                    // Handle consequences
                    for consequence in &option.consequences {
                        match consequence.type_ {
                            ConsequenceType::SetVariable => {
                                output.push_str(&format!("    <<set ${}>>>\n", consequence.value));
                            }
                            ConsequenceType::ModifyTrust => {
                                output.push_str(&format!("    <<set $trust {}>>>\n", consequence.value));
                            }
                            _ => {}
                        }
                    }
                    
                    self.write_node_yarn(&option.next_node, output, visited);
                }
            }
            
            NodeContent::Condition { check, true_branch, false_branch } => {
                output.push_str(&format!("<<if {}>>>\n", check));
                self.write_node_yarn(true_branch, output, visited);
                
                if let Some(false_branch) = false_branch {
                    output.push_str("<<else>>>\n");
                    self.write_node_yarn(false_branch, output, visited);
                }
                
                output.push_str("<<endif>>>\n");
            }
            
            NodeContent::Action { description, effects } => {
                output.push_str(&format!("// Action: {}\n", description));
                for effect in effects {
                    output.push_str(&format!("<<{}>>>\n", effect));
                }
                
                // Continue to next node
                for edge in &self.edges {
                    if edge.from == node_id {
                        self.write_node_yarn(&edge.to, output, visited);
                    }
                }
            }
        }
    }
    
    /// Convert to Cobweb story graph format
    pub fn to_cobweb(&self) -> serde_json::Value {
        serde_json::json!({
            "nodes": self.nodes.values().map(|n| {
                serde_json::json!({
                    "id": n.id,
                    "content": n.content,
                    "metadata": n.metadata,
                })
            }).collect::<Vec<_>>(),
            "edges": self.edges,
            "root": self.root,
        })
    }
    
    /// Calculate complexity metrics for token optimization
    pub fn complexity(&self) -> ComplexityMetrics {
        ComplexityMetrics {
            node_count: self.nodes.len(),
            edge_count: self.edges.len(),
            max_depth: self.calculate_max_depth(),
            branching_factor: self.calculate_branching_factor(),
            choice_points: self.count_choices(),
        }
    }
    
    fn calculate_max_depth(&self) -> usize {
        // DFS to find max depth
        fn dfs(tree: &NarrativeTree, node: &str, depth: usize, visited: &mut Vec<String>) -> usize {
            if visited.contains(&node.to_string()) {
                return depth;
            }
            visited.push(node.to_string());
            
            let mut max = depth;
            for edge in &tree.edges {
                if edge.from == node {
                    let d = dfs(tree, &edge.to, depth + 1, visited);
                    if d > max {
                        max = d;
                    }
                }
            }
            max
        }
        
        dfs(self, &self.root, 0, &mut Vec::new())
    }
    
    fn calculate_branching_factor(&self) -> f32 {
        let mut total_branches = 0;
        let mut branching_nodes = 0;
        
        for node_id in self.nodes.keys() {
            let outgoing = self.edges.iter().filter(|e| &e.from == node_id).count();
            if outgoing > 1 {
                total_branches += outgoing;
                branching_nodes += 1;
            }
        }
        
        if branching_nodes > 0 {
            total_branches as f32 / branching_nodes as f32
        } else {
            1.0
        }
    }
    
    fn count_choices(&self) -> usize {
        self.nodes.values().filter(|n| {
            matches!(n.content, NodeContent::Choice { .. })
        }).count()
    }
}

#[derive(Debug)]
pub struct ComplexityMetrics {
    pub node_count: usize,
    pub edge_count: usize,
    pub max_depth: usize,
    pub branching_factor: f32,
    pub choice_points: usize,
}

impl ComplexityMetrics {
    /// Estimate tokens needed to generate this structure
    pub fn estimated_tokens(&self) -> usize {
        // Rough estimate: each node ~50 tokens, each edge ~10 tokens
        (self.node_count * 50) + (self.edge_count * 10) + (self.choice_points * 30)
    }
}
