// src/lib.rs
/*
 * Core library for ContentExtractor
 */

use log::{info, error, debug};
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessResult {
    pub success: bool,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug)]
pub struct ContentExtractorProcessor {
    verbose: bool,
    processed_count: usize,
}

impl ContentExtractorProcessor {
    pub fn new(verbose: bool) -> Self {
        Self {
            verbose,
            processed_count: 0,
        }
    }

    pub fn process(&mut self, data: &str) -> Result<ProcessResult> {
        if self.verbose {
            debug!("Processing data of length: {}", data.len());
        }

        // Simulate processing
        self.processed_count += 1;
        
        let result = ProcessResult {
            success: true,
            message: format!("Successfully processed item #{}", self.processed_count),
            data: Some(serde_json::json!({
                "length": data.len(),
                "processed_at": chrono::Utc::now().to_rfc3339(),
                "item_number": self.processed_count
            })),
        };

        Ok(result)
    }

    pub fn get_stats(&self) -> serde_json::Value {
        serde_json::json!({
            "processed_count": self.processed_count,
            "verbose": self.verbose
        })
    }
}

/// Main processing function
pub fn run(verbose: bool, input: Option<String>, output: Option<String>) -> Result<()> {
    if verbose {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Debug)
            .init();
    } else {
        env_logger::init();
    }

    if content.trim().is_empty() {
    eprintln!("Warning: Input file is empty.");
    return Ok(()); // or return an error if preferred
}
    
    info!("Starting ContentExtractor processing");
    
    let mut processor = ContentExtractorProcessor::new(verbose);
    
    // Read input
    let input_data = match input {
        Some(path) => {
            info!("Reading from file: {}", path);
            fs::read_to_string(&path)?
        },
        None => {
            info!("Using default test data");
            "Sample data for processing".to_string()
        }
    };
    
    // Process data
    let result = processor.process(&input_data)?;
    
    if verbose {
        debug!("Processing result: {:#?}", result);
    }
    
    // Save output
    let output_json = serde_json::to_string_pretty(&result)?;
    
    match output {
        Some(path) => {
            info!("Writing results to: {}", path);
            fs::write(&path, &output_json)?;
        },
        None => {
            println!("{}", output_json);
        }
    }
    
    let stats = processor.get_stats();
    info!("Processing complete. Stats: {}", stats);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processor_creation() {
        let processor = ContentExtractorProcessor::new(true);
        assert_eq!(processor.verbose, true);
        assert_eq!(processor.processed_count, 0);
    }

    #[test]
    fn test_data_processing() {
        let mut processor = ContentExtractorProcessor::new(false);
        let result = processor.process("test data").unwrap();
        
        assert!(result.success);
        assert_eq!(processor.processed_count, 1);
    }

    #[test]
    fn test_run_function() {
        // Test the main run function
        let result = run(false, None, None);
        assert!(result.is_ok());
    }
}
