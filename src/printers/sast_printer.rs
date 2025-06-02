// src/pretty_printer.rs

use std::collections::HashMap;
use crate::parsers::syn_ast::SourcePosition;
use crate::state::sast_state::{Certainty, SastState, Severity, SynAst, SynAstMap, SynAstMapExt, SynAstResult, SynMatchResult, SynRuleMetadata};
use anyhow::{Context, Result};
use prettytable::{format, row, Cell, Row, Table};

/// Utility responsible for displaying SAST analysis results in a human-readable format.
///
/// Supports printing summaries, detailed match reports, and JSON output.
#[derive(Debug, Clone)]
pub struct SastPrinter;

impl SastPrinter {
    /// Displays a full summary and detailed output of the SAST state results.
    ///
    /// Outputs number of scanned files, a summary table of rule matches,
    /// and detailed findings with metadata and source location.
    ///
    /// # Arguments
    ///
    /// * `state` - The `SastState` containing results to print.
    ///
    /// # Returns
    ///
    /// `Ok(())` on success or an error if printing fails.
    pub fn print_sast_state(state: &SastState, scanned_dir: &String) -> Result<()> {
        println!("\n================================================================================\n\n{} files scanned in {} directory\n", state.syn_ast_map.count_files(), scanned_dir);

        let all_results: Vec<SynAstResult> = state
            .syn_ast_map
            .values()
            .flat_map(|ast| ast.results.clone())
            .collect();

        Self::print_rules_summary(&all_results)?;

        let results_with_matches: Vec<(String, &SynAstResult)> = state
            .syn_ast_map
            .iter()
            .flat_map(|(filename, ast)| {
                ast.results
                    .iter()
                    .filter(|result| !result.matches.is_empty())
                    .map(|result| (filename.clone(), result))
            })
            .collect();

        if !results_with_matches.is_empty() {
            println!("\nDetailed findings:");
            let mut grouped_results: HashMap<String, Vec<(String, &SynAstResult)>> = HashMap::new();
            
            for (filename, ast_res) in results_with_matches.iter() {
                let rule_name = ast_res.rule_metadata.name.clone();
                grouped_results.entry(rule_name).or_default().push((filename.clone(), *ast_res));
            }

            for (rule_name, results) in grouped_results {
                let first_result = &results[0].1;
                println!("\n{}", "=".repeat(80));
                Self::print_rule_metadata(&first_result.rule_metadata, first_result.rule_filename.to_string())?;

                let total_matches: usize = results.iter().map(|(_, res)| res.matches.len()).sum();
                println!("\nMatches found: {}", total_matches);

                for (filename, ast_res) in results {
                    for match_result in &ast_res.matches {
                        match match_result.get_location_metadata() {
                            Ok(pos) => println!("{}", pos.get_pretty_string()),
                            Err(_) => println!("{}: {}", filename, match_result.access_path)    
                        }
                    }
                }

                println!("{}", "=".repeat(80));
            }
        } else {
            println!("\nNo vulnerabilities detected.");
        }

        Ok(())

    }

    /// Displays a summary table of all rule matches across all analyzed files.
    ///
    /// Each row includes the rule name, severity, certainty, file name, and match count.
    ///
    /// # Arguments
    ///
    /// * `results` - A slice of `SynAstResult` entries to summarize.
    ///
    /// # Returns
    ///
    /// `Ok(())` on success or an error if rendering fails.
    pub fn print_rules_summary(results: &[SynAstResult]) -> Result<()> {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_BOX_CHARS);

        table.add_row(Row::new(vec![
            Cell::new("Rule Name").style_spec("bFc"),
            Cell::new("Severity").style_spec("bFc"),
            Cell::new("Certainty").style_spec("bFc"),
            Cell::new("Files").style_spec("bFc"),
            Cell::new("Total Matches").style_spec("bFc"),
        ]));

        let severity_to_cell = |severity: &Severity| -> Cell {
            match severity {
                Severity::Critical => Cell::new("Critical").style_spec("Fr"),
                Severity::High => Cell::new("High").style_spec("Fr"),
                Severity::Medium => Cell::new("Medium").style_spec("Fy"),
                Severity::Low => Cell::new("Low").style_spec("Fg"),
                Severity::Unknown => Cell::new("Unknown").style_spec("Fw"),
            }
        };

        let certainty_to_cell = |certainty: &Certainty| -> Cell {
            match certainty {
                Certainty::High => Cell::new("High").style_spec("Fg"),
                Certainty::Medium => Cell::new("Medium").style_spec("Fy"),
                Certainty::Low => Cell::new("Low").style_spec("Fr"),
                Certainty::Unknown => Cell::new("Unknown").style_spec("Fw"),
            }
        };

        let mut rule_groups: std::collections::HashMap<String, Vec<&SynAstResult>> =
            std::collections::HashMap::new();

        for result in results {
            rule_groups
                .entry(result.rule_metadata.name.clone())
                .or_default()
                .push(result);
        }

        for (rule_name, group_results) in rule_groups {
            let first_result = &group_results[0];

            let file_count = group_results
                .iter()
                .map(|r| &r.rule_filename)
                .collect::<std::collections::HashSet<_>>()
                .len();

            let total_matches: usize = group_results.iter().map(|r| r.matches.len()).sum();

            let file_list = group_results
                .iter()
                .map(|r| r.rule_filename.clone())
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect::<Vec<_>>()
                .join(", ");

            table.add_row(Row::new(vec![
                Cell::new(&rule_name),
                severity_to_cell(&first_result.rule_metadata.severity),
                certainty_to_cell(&first_result.rule_metadata.certainty),
                Cell::new(&file_list),
                Cell::new(&total_matches.to_string()),
            ]));
        }

        table.printstd();

        Ok(())
    }
    /// Prints detailed information about a single rule result, including all matches.
    ///
    /// # Arguments
    ///
    /// * `result` - The rule result to display.
    /// * `syn_ast_map` - A map of syntax trees used to resolve source locations.
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or an error if printing fails.
    pub fn print_result(
        filename: String,
        result: &SynAstResult,
    ) -> Result<()> {
        println!("\n{}", "=".repeat(80));
        Self::print_rule_metadata(&result.rule_metadata, result.rule_filename.to_string())?;

        if !result.matches.is_empty() {
            println!("\nMatches found: {}", result.matches.len());
            for (i, match_result) in result.matches.iter().enumerate() {
                let match_number = i + 1;
                println!("#{}: {}", match_number, match_result.access_path)
            }
        } else {
            println!("\nNo matches found.");
        }

        println!("{}", "=".repeat(80));

        Ok(())
    }
    /// Displays the metadata of a given rule, including version, author,
    /// severity, certainty, and description.
    ///
    /// # Arguments
    ///
    /// * `metadata` - Metadata object to display.
    ///
    /// # Returns
    ///
    /// `Ok(())` on success.
    fn print_rule_metadata(metadata: &SynRuleMetadata, rule_filename: String) -> Result<()> {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

        table.add_row(Row::new(vec![
            Cell::new("Name:").style_spec("b"),
            Cell::new(&metadata.name),
        ]));

        table.add_row(Row::new(vec![
            Cell::new("File:").style_spec("b"),
            Cell::new(&rule_filename),
        ]));

        table.add_row(Row::new(vec![
            Cell::new("Version:").style_spec("b"),
            Cell::new(&metadata.version),
        ]));

        table.add_row(Row::new(vec![
            Cell::new("Author:").style_spec("b"),
            Cell::new(&metadata.author),
        ]));

        let severity_text = format!("{:?}", metadata.severity);
        let severity_cell = match metadata.severity {
            Severity::Critical => Cell::new(&severity_text).style_spec("Fr"),
            Severity::High => Cell::new(&severity_text).style_spec("Fr"),
            Severity::Medium => Cell::new(&severity_text).style_spec("Fy"),
            Severity::Low => Cell::new(&severity_text).style_spec("Fg"),
            Severity::Unknown => Cell::new(&severity_text).style_spec("Fw"),
        };

        table.add_row(Row::new(vec![
            Cell::new("Severity:").style_spec("b"),
            severity_cell,
        ]));

        let certainty_text = format!("{:?}", metadata.certainty);
        let certainty_cell = match metadata.certainty {
            Certainty::High => Cell::new(&certainty_text).style_spec("Fg"),
            Certainty::Medium => Cell::new(&certainty_text).style_spec("Fy"),
            Certainty::Low => Cell::new(&certainty_text).style_spec("Fr"),
            Certainty::Unknown => Cell::new(&certainty_text).style_spec("Fw"),
        };

        table.add_row(Row::new(vec![
            Cell::new("Certainty:").style_spec("b"),
            certainty_cell,
        ]));

        table.add_row(Row::new(vec![
            Cell::new("Description:").style_spec("b"),
            Cell::new(&metadata.description),
        ]));

        table.printstd();

        Ok(())
    }

    fn find_source_position(
        match_result: &SynMatchResult,
    ) -> Option<SourcePosition> {
        match match_result.get_location_metadata().ok() {
            Some(pos) => Option::from(pos),
            None => None,
        }
    }

    /// Outputs the entire result set in a prettified JSON format.
    ///
    /// # Arguments
    ///
    /// * `results` - A slice of `SynAstResult` entries to serialize.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the output was successful, or an error if serialization fails.
    pub fn print_results_as_json(results: &[SynAstResult]) -> Result<()> {
        let json =
            serde_json::to_string_pretty(results).context("Failed to serialize results to JSON")?;
        println!("{}", json);
        Ok(())
    }
}

impl SynAstResult {
    /// Converts a `SynAstResult` into a `prettytable::Row` suitable for tabular display.
    pub fn to_table_row(&self) -> Row {
        Row::new(vec![
            Cell::new(&self.rule_metadata.name),
            Cell::new(&format!("{:?}", self.rule_metadata.severity)),
            Cell::new(&format!("{:?}", self.rule_metadata.certainty)),
            Cell::new(&self.rule_filename),
            Cell::new(&self.matches.len().to_string()),
        ])
    }
}
