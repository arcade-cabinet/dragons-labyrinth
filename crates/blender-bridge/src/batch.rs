//! Batch processing and request routing for 3D model operations
//! 
//! This module handles queuing, routing, and parallel execution of model conversions
//! and BPY script generation

use std::path::PathBuf;
use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;
use crate::error::BlenderBridgeError;
use crate::conversion::ConversionResult;
use crate::ron_models::generate_model_from_ron;
use serde::{Deserialize, Serialize};

/// Type of operation to perform
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationType {
    /// Convert an existing model file (OBJ, FBX, etc.) to GLB
    ModelConversion {
        src: PathBuf,
        dst: PathBuf,
        scale: f32,
    },
    /// Execute a BPY script to generate procedural content
    BpyGeneration {
        script_path: PathBuf,
        output_path: PathBuf,
        scale: f32,
    },
    /// Execute inline BPY script content
    BpyScript {
        script_content: String,
        output_path: PathBuf,
        scale: f32,
    }
}

/// A batch request that can handle multiple operations
#[derive(Debug, Clone)]
pub struct BatchRequest {
    pub operations: Vec<OperationType>,
    pub parallel: bool,  // Whether to process in parallel
    pub continue_on_error: bool,
}

/// Result of a batch operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResult {
    pub total_operations: usize,
    pub successful: usize,
    pub failed: usize,
    pub results: Vec<ConversionResult>,
    pub total_time_ms: u64,
}

/// Batch processor that routes requests appropriately
pub struct BatchProcessor {
    queue: Arc<Mutex<VecDeque<OperationType>>>,
    max_parallel: usize,
}

impl BatchProcessor {
    /// Create a new batch processor
    pub fn new(max_parallel: usize) -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            max_parallel: max_parallel.max(1),
        }
    }
    
    /// Process a batch request
    pub fn process_batch(&mut self, request: BatchRequest) -> Result<BatchResult, BlenderBridgeError> {
        let start_time = Instant::now();
        let mut results = Vec::new();
        let mut successful = 0;
        let mut failed = 0;
        
        for operation in request.operations {
            let result = self.process_operation(operation)?;
            
            if result.success {
                successful += 1;
            } else {
                failed += 1;
                if !request.continue_on_error {
                    return Err(BlenderBridgeError::BatchFailed {
                        message: format!("Operation failed: {:?}", result.error),
                    });
                }
            }
            
            results.push(result);
        }
        
        Ok(BatchResult {
            total_operations: results.len(),
            successful,
            failed,
            results,
            total_time_ms: start_time.elapsed().as_millis() as u64,
        })
    }
    
    /// Process a single operation
    fn process_operation(&self, operation: OperationType) -> Result<ConversionResult, BlenderBridgeError> {
        let _start_time = Instant::now();
        
        match operation {
            OperationType::ModelConversion { src, dst, scale: _ } => {
                // For now, model conversion isn't implemented
                // Return an error result
                Ok(ConversionResult {
                    success: false,
                    input_file: src.to_string_lossy().to_string(),
                    output_file: dst.to_string_lossy().to_string(),
                    stats: None,
                    error: Some("Direct model conversion not yet implemented".to_string()),
                    skipped: false,
                })
            }
            
            OperationType::BpyGeneration { script_path, output_path, scale: _ } => {
                // This should now be a RON file path
                if script_path.extension().map_or(false, |ext| ext == "ron") {
                    generate_model_from_ron(&script_path, &output_path)?;
                    Ok(ConversionResult {
                        success: true,
                        input_file: script_path.to_string_lossy().to_string(),
                        output_file: output_path.to_string_lossy().to_string(),
                        stats: None,
                        error: None,
                        skipped: false,
                    })
                } else {
                    Ok(ConversionResult {
                        success: false,
                        input_file: script_path.to_string_lossy().to_string(),
                        output_file: output_path.to_string_lossy().to_string(),
                        stats: None,
                        error: Some("Expected RON file, not Python script".to_string()),
                        skipped: false,
                    })
                }
            }
            
            OperationType::BpyScript { script_content, output_path, scale } => {
                // BPY scripts are no longer supported
                _ = (script_content, scale);
                Ok(ConversionResult {
                    success: false,
                    input_file: "inline_script".to_string(),
                    output_file: output_path.to_string_lossy().to_string(),
                    stats: None,
                    error: Some("BPY script execution no longer supported. Use RON models instead.".to_string()),
                    skipped: false,
                })
            }
        }
    }

}

/// Queue-based batch manager for handling large numbers of operations
pub struct QueuedBatchManager {
    processor: BatchProcessor,
    pending: Arc<Mutex<VecDeque<OperationType>>>,
}

impl QueuedBatchManager {
    pub fn new(max_parallel: usize) -> Self {
        Self {
            processor: BatchProcessor::new(max_parallel),
            pending: Arc::new(Mutex::new(VecDeque::new())),
        }
    }
    
    /// Add an operation to the queue
    pub fn enqueue(&self, operation: OperationType) {
        let mut queue = self.pending.lock().unwrap();
        queue.push_back(operation);
    }
    
    /// Process all pending operations
    pub fn process_all(&mut self) -> Result<BatchResult, BlenderBridgeError> {
        let mut queue = self.pending.lock().unwrap();
        let operations: Vec<_> = queue.drain(..).collect();
        drop(queue); // Release lock
        
        let request = BatchRequest {
            operations,
            parallel: true,
            continue_on_error: true,
        };
        
        self.processor.process_batch(request)
    }
    
    /// Get the number of pending operations
    pub fn pending_count(&self) -> usize {
        self.pending.lock().unwrap().len()
    }
}
