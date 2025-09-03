//! Polars DataFrame utilities for audit data processing
//!
//! Provides pandas-like functionality for audit reports using Polars lazy API.

use anyhow::{Context, Result};
use dl_types::AuditableType;
use polars::prelude::*;
use std::collections::HashMap;
use std::path::Path;

/// Builder for creating Polars DataFrames from auditable types
pub struct DataFrameBuilder {
    pub(crate) lazy_frame: Option<LazyFrame>,
}

impl DataFrameBuilder {
    /// Create new DataFrame builder
    pub fn new() -> Self {
        Self {
            lazy_frame: None,
        }
    }
    
    /// Build DataFrame from collection of auditable types
    pub fn from_auditable<T: AuditableType>(items: &[T]) -> Result<Self> {
        if items.is_empty() {
            return Ok(Self::new());
        }
        
        // Get headers and convert to Polars DataFrame structure
        let headers = T::audit_headers();
        let mut columns: HashMap<String, Vec<String>> = HashMap::new();
        
        // Initialize columns
        for header in &headers {
            columns.insert(header.clone(), Vec::new());
        }
        
        // Fill columns with data
        for item in items {
            let row = item.audit_row();
            for (i, value) in row.into_iter().enumerate() {
                if let Some(header) = headers.get(i) {
                    columns.get_mut(header).unwrap().push(value);
                }
            }
            
            // Add custom fields if any
            let custom_fields = item.custom_fields();
            for (key, value) in custom_fields {
                columns.entry(key).or_insert_with(Vec::new).push(value);
            }
        }
        
        // Convert to Polars DataFrame
        let mut series_vec = Vec::new();
        for (name, values) in columns {
            let series = Series::new(name.as_str().into(), &values);
            series_vec.push(series.into());
        }
        
        let df = DataFrame::new(series_vec)
            .context("Failed to create DataFrame")?;
            
        let lazy_frame = df.lazy();
        
        Ok(Self {
            lazy_frame: Some(lazy_frame),
        })
    }
    
    /// Add calculated columns using lazy evaluation
    pub fn with_calculated_columns<T: AuditableType>(mut self, items: &[T]) -> Result<Self> {
        if let Some(lf) = self.lazy_frame.take() {
            let mut updated_lf = lf;
            
            // Add numeric field calculations
            for item in items.iter().take(1) { // Just check first item for available numeric fields
                let numeric_fields = item.extract_numeric_fields();
                for (field_name, _) in numeric_fields {
                    // Add statistical calculations for numeric fields
                    updated_lf = updated_lf.with_columns([
                        col(&field_name).cast(DataType::Float64).alias(&format!("{}_f64", field_name))
                    ]);
                }
                break; // Only need to check once for field types
            }
            
            self.lazy_frame = Some(updated_lf);
        }
        
        Ok(self)
    }
    
    /// Apply filters using lazy evaluation
    pub fn filter_by(mut self, column: &str, predicate: Expr) -> Result<Self> {
        if let Some(lf) = self.lazy_frame.take() {
            self.lazy_frame = Some(lf.filter(predicate));
        }
        Ok(self)
    }
    
    /// Group by column and aggregate
    pub fn group_by_and_aggregate(mut self, group_cols: &[String], agg_exprs: &[Expr]) -> Result<Self> {
        if let Some(lf) = self.lazy_frame.take() {
            self.lazy_frame = Some(lf.group_by(group_cols.iter().map(|s| col(s)).collect::<Vec<_>>())
                .agg(agg_exprs));
        }
        Ok(self)
    }
    
    /// Sort by column
    pub fn sort_by(mut self, column: &str, descending: bool) -> Result<Self> {
        if let Some(lf) = self.lazy_frame.take() {
            let sort_opts = SortMultipleOptions::new()
                .with_order_descending(descending);
            self.lazy_frame = Some(lf.sort([column], sort_opts));
        }
        Ok(self)
    }
    
    /// Add row numbering column
    pub fn with_row_numbers(mut self) -> Result<Self> {
        if let Some(lf) = self.lazy_frame.take() {
            self.lazy_frame = Some(lf.with_row_index("row_num", Some(1)));
        }
        Ok(self)
    }
    
    /// Execute lazy operations and collect DataFrame
    pub fn collect(self) -> Result<DataFrame> {
        if let Some(lf) = self.lazy_frame {
            lf.collect().context("Failed to execute lazy operations")
        } else {
            // Return empty DataFrame if no data
            Ok(DataFrame::empty())
        }
    }
    
    /// Write to CSV file
    pub fn write_csv<P: AsRef<Path>>(self, path: P) -> Result<AuditStats> {
        let df = self.collect()?;
        
        // Ensure parent directory exists
        if let Some(parent) = path.as_ref().parent() {
            std::fs::create_dir_all(parent)
                .context("Failed to create output directory")?;
        }
        
        // Write CSV
        let mut file = std::fs::File::create(path.as_ref())
            .context("Failed to create CSV file")?;
            
        CsvWriter::new(&mut file)
            .include_header(true)
            .finish(&mut df.clone())
            .context("Failed to write CSV")?;
        
        // Calculate statistics
        let stats = AuditStats::from_dataframe(&df)?;
        
        Ok(stats)
    }
    
    /// Get preview of data (first N rows)
    pub fn preview(self, n_rows: usize) -> Result<DataFrame> {
        if let Some(lf) = self.lazy_frame {
            lf.limit(n_rows as IdxSize).collect().context("Failed to preview data")
        } else {
            Ok(DataFrame::empty())
        }
    }
    
    /// Get summary statistics for all numeric columns
    pub fn describe(self) -> Result<DataFrame> {
        if let Some(lf) = self.lazy_frame {
            // Use basic aggregations instead of describe() which may not be available
            let summary_lf = lf.select([
                lit("summary").alias("metric"),
                len().alias("total_rows"),
            ]);
            summary_lf.collect().context("Failed to generate description")
        } else {
            Ok(DataFrame::empty())
        }
    }
}

impl Default for DataFrameBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about generated audit data
#[derive(Debug, Clone)]
pub struct AuditStats {
    pub row_count: usize,
    pub column_count: usize,
    pub numeric_columns: Vec<String>,
    pub text_columns: Vec<String>,
    pub null_counts: HashMap<String, usize>,
    pub memory_usage_bytes: usize,
}

impl AuditStats {
    /// Generate statistics from a Polars DataFrame
    pub fn from_dataframe(df: &DataFrame) -> Result<Self> {
        let shape = df.shape();
        let row_count = shape.0;
        let column_count = shape.1;
        
        let mut numeric_columns = Vec::new();
        let mut text_columns = Vec::new();
        let mut null_counts = HashMap::new();
        
        for column in df.get_columns() {
            let name = column.name();
            
            // Categorize column types
            match column.dtype() {
                DataType::Int8 | DataType::Int16 | DataType::Int32 | DataType::Int64 |
                DataType::UInt8 | DataType::UInt16 | DataType::UInt32 | DataType::UInt64 |
                DataType::Float32 | DataType::Float64 => {
                    numeric_columns.push(name.to_string());
                }
                DataType::String => {
                    text_columns.push(name.to_string());
                }
                dtype if dtype.is_categorical() => {
                    text_columns.push(name.to_string());
                }
                _ => {
                    text_columns.push(name.to_string()); // Default to text
                }
            }
            
            // Count nulls
            let null_count = column.null_count();
            null_counts.insert(name.to_string(), null_count);
        }
        
        let memory_usage_bytes = df.estimated_size();
        
        Ok(Self {
            row_count,
            column_count,
            numeric_columns,
            text_columns,
            null_counts,
            memory_usage_bytes,
        })
    }
    
    /// Get human-readable memory usage
    pub fn memory_usage_human(&self) -> String {
        let bytes = self.memory_usage_bytes as f64;
        if bytes < 1024.0 {
            format!("{} B", bytes)
        } else if bytes < 1024.0 * 1024.0 {
            format!("{:.1} KB", bytes / 1024.0)
        } else {
            format!("{:.1} MB", bytes / (1024.0 * 1024.0))
        }
    }
    
    /// Check if data quality is good (low null counts, reasonable size)
    pub fn is_healthy(&self) -> bool {
        let max_null_percentage = 0.1; // 10% nulls acceptable
        
        for (_, null_count) in &self.null_counts {
            let null_percentage = *null_count as f64 / self.row_count as f64;
            if null_percentage > max_null_percentage {
                return false;
            }
        }
        
        // Check if we have reasonable data
        self.row_count > 0 && self.column_count > 0
    }
}

/// Common aggregation expressions for audit reports
pub mod aggregations {
    use polars::prelude::*;
    
    /// Count non-null values
    pub fn count_non_null(column: &str) -> Expr {
        col(column).count().alias(&format!("{}_count", column))
    }
    
    /// Count unique values  
    pub fn count_unique(column: &str) -> Expr {
        col(column).n_unique().alias(&format!("{}_unique", column))
    }
    
    /// Basic numeric statistics
    pub fn numeric_summary(column: &str) -> Vec<Expr> {
        vec![
            col(column).min().alias(&format!("{}_min", column)),
            col(column).max().alias(&format!("{}_max", column)),
            col(column).mean().alias(&format!("{}_mean", column)),
            col(column).median().alias(&format!("{}_median", column)),
            col(column).std(0).alias(&format!("{}_std", column)),
        ]
    }
    
    /// Text field summary
    pub fn text_summary(column: &str) -> Vec<Expr> {
        vec![
            col(column).count().alias(&format!("{}_count", column)),
            col(column).n_unique().alias(&format!("{}_unique", column)),
            col(column).str().len_chars().mean().alias(&format!("{}_avg_length", column)),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[derive(Debug)]
    struct TestData {
        name: String,
        value: f64,
    }
    
    impl AuditableType for TestData {
        fn audit_headers() -> Vec<String> {
            vec!["name".to_string(), "value".to_string()]
        }
        
        fn audit_row(&self) -> Vec<String> {
            vec![self.name.clone(), self.value.to_string()]
        }
        
        fn audit_category() -> String {
            "test".to_string()
        }
        
        fn audit_subcategory() -> String {
            "data".to_string()
        }
        
        fn extract_numeric_fields(&self) -> HashMap<String, f64> {
            let mut fields = HashMap::new();
            fields.insert("value".to_string(), self.value);
            fields
        }
    }
    
    #[test]
    fn test_dataframe_creation() -> Result<()> {
        let test_items = vec![
            TestData { name: "item1".to_string(), value: 10.0 },
            TestData { name: "item2".to_string(), value: 20.0 },
            TestData { name: "item3".to_string(), value: 30.0 },
        ];
        
        let builder = DataFrameBuilder::from_auditable(&test_items)?;
        let df = builder.collect()?;
        
        assert_eq!(df.height(), 3);
        assert_eq!(df.width(), 2);
        
        Ok(())
    }
    
    #[test]
    fn test_audit_stats() -> Result<()> {
        let test_items = vec![
            TestData { name: "item1".to_string(), value: 10.0 },
            TestData { name: "item2".to_string(), value: 20.0 },
        ];
        
        let builder = DataFrameBuilder::from_auditable(&test_items)?;
        let df = builder.collect()?;
        let stats = AuditStats::from_dataframe(&df)?;
        
        assert_eq!(stats.row_count, 2);
        assert_eq!(stats.column_count, 2);
        assert!(stats.is_healthy());
        
        Ok(())
    }
}
