// src/pretty_printer.rs

use crate::state::sast_state::{
    Certainty, SastState, Severity, SynAstMapExt, SynAstResult, SynRuleMetadata,
};
use anyhow::{Context, Result};
use prettytable::{format, Cell, Row, Table};
use std::collections::HashMap;

/// A utility for displaying Static Analysis (SAST) results in a readable format.
///
/// This printer handles the presentation of scan summaries, detailed findings,
/// and rule metadata in structured tables.
#[derive(Debug, Clone)]
pub struct SastPrinter;

impl SastPrinter {
    /// Displays a comprehensive report of the SAST analysis results.
    ///
    /// This function orchestrates the printing of the scan summary, a summary table
    /// of rule matches, and detailed findings for each vulnerability.
    ///
    /// # Arguments
    ///
    /// * `state` - The `SastState` containing the analysis results.
    /// * `scanned_dir` - The directory on which the scan was performed.
    ///
    /// # Returns
    ///
    /// An empty `Result` on success, or an error if printing fails.
    pub fn print_sast_state(state: &SastState, scanned_dir: &String) -> Result<()> {
        Self::print_scan_summary(state, scanned_dir);

        let all_results = Self::collect_all_results(state);
        Self::print_rules_summary(&all_results)?;

        let results_with_matches = Self::collect_results_with_matches(state);

        if !results_with_matches.is_empty() {
            Self::print_detailed_findings(&results_with_matches)?;
        } else {
            println!("\nNo vulnerabilities detected.");
        }

        Ok(())
    }

    /// Prints a summary of the scan, including the number of files scanned and the target directory.
    ///
    /// # Arguments
    ///
    /// * `state` - The `SastState` from the analysis.
    /// * `scanned_dir` - The directory that was scanned.
    fn print_scan_summary(state: &SastState, scanned_dir: &String) {
        println!(
            "\n================================================================================\n\n{} files scanned in {} directory\n",
            state.syn_ast_map.count_files(),
            scanned_dir
        );
    }

    /// Collects and flattens all analysis results from the SAST state.
    ///
    /// # Arguments
    ///
    /// * `state` - The `SastState` containing results across multiple files.
    ///
    /// # Returns
    ///
    /// A vector of `SynAstResult` containing all findings.
    fn collect_all_results(state: &SastState) -> Vec<SynAstResult> {
        state
            .syn_ast_map
            .values()
            .flat_map(|ast| ast.results.clone())
            .collect()
    }

    /// Collects results that have at least one match, pairing them with their source filename.
    ///
    /// # Arguments
    ///
    /// * `state` - The `SastState` to filter results from.
    ///
    /// # Returns
    ///
    /// A vector of tuples, each containing a filename and a reference to the `SynAstResult`.
    fn collect_results_with_matches(state: &SastState) -> Vec<(String, &SynAstResult)> {
        state
            .syn_ast_map
            .iter()
            .flat_map(|(filename, ast)| {
                ast.results
                    .iter()
                    .filter(|result| !result.matches.is_empty())
                    .map(move |result| (filename.clone(), result))
            })
            .collect()
    }

    /// Prints the detailed findings, grouped by rule, for all identified matches.
    ///
    /// # Arguments
    ///
    /// * `results_with_matches` - A slice of tuples, each with a filename and a result.
    ///
    /// # Returns
    ///
    /// An empty `Result` on success, or an error if printing fails.
    fn print_detailed_findings(results_with_matches: &[(String, &SynAstResult)]) -> Result<()> {
        println!("\nDetailed findings:");
        let grouped_results = Self::group_results_by_rule_name(results_with_matches);

        for (_rule_name, results) in grouped_results {
            let first_result = &results[0].1;
            println!("\n{}", "=".repeat(80));
            Self::print_rule_metadata(
                &first_result.rule_metadata,
                first_result.rule_filename.to_string(),
            )?;

            let total_matches: usize = results.iter().map(|(_, res)| res.matches.len()).sum();
            println!("\nMatches found: {}", total_matches);

            Self::print_match_locations(&results);
            println!("{}", "=".repeat(80));
        }

        Ok(())
    }

    /// Groups analysis results by rule name for organized reporting.
    ///
    /// # Arguments
    ///
    /// * `results_with_matches` - A slice of tuples containing filenames and results.
    ///
    /// # Returns
    ///
    /// A `HashMap` where keys are rule names and values are vectors of corresponding results.
    fn group_results_by_rule_name<'a>(
        results_with_matches: &[(String, &'a SynAstResult)],
    ) -> HashMap<String, Vec<(String, &'a SynAstResult)>> {
        let mut grouped_results: HashMap<String, Vec<(String, &'a SynAstResult)>> = HashMap::new();

        for (filename, ast_res) in results_with_matches {
            let rule_name = ast_res.rule_metadata.name.clone();
            grouped_results
                .entry(rule_name)
                .or_default()
                .push((filename.clone(), *ast_res));
        }

        grouped_results
    }

    /// Prints the source code location for each match in a set of results.
    ///
    /// # Arguments
    ///
    /// * `results` - A slice of tuples containing filenames and results to print locations for.
    fn print_match_locations(results: &[(String, &SynAstResult)]) {
        for (filename, ast_res) in results {
            for match_result in &ast_res.matches {
                match match_result.get_location_metadata() {
                    Ok(pos) => println!("{}", pos.get_pretty_string()),
                    Err(_) => println!("{}: {}", filename, match_result.access_path),
                }
            }
        }
    }

    /// Displays a summary table of all matched rules.
    ///
    /// Each row includes the rule name, severity, certainty, associated files, and total matches.
    ///
    /// # Arguments
    ///
    /// * `results` - A slice of `SynAstResult` entries to be summarized.
    ///
    /// # Returns
    ///
    /// An empty `Result` on success, or an error if rendering the table fails.
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

        let mut rule_groups: HashMap<String, Vec<&SynAstResult>> = HashMap::new();

        for result in results {
            rule_groups
                .entry(result.rule_metadata.name.clone())
                .or_default()
                .push(result);
        }

        for (rule_name, group_results) in rule_groups {
            let first_result = &group_results[0];
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

    /// Displays the metadata for a given rule in a structured table.
    ///
    /// # Arguments
    ///
    /// * `metadata` - The `SynRuleMetadata` object to display.
    /// * `rule_filename` - The filename of the rule being displayed.
    ///
    /// # Returns
    ///
    /// An empty `Result` on success.
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

    /// Outputs the analysis results in a prettified JSON format.
    ///
    /// # Arguments
    ///
    /// * `results` - A slice of `SynAstResult` entries to serialize and print.
    ///
    /// # Returns
    ///
    /// An empty `Result` on success, or an error if serialization fails.
    #[allow(dead_code)]
    pub fn print_results_as_json(results: &[SynAstResult]) -> Result<()> {
        let json =
            serde_json::to_string_pretty(results).context("Failed to serialize results to JSON")?;
        println!("{}", json);
        Ok(())
    }
}

impl SynAstResult {
    /// Converts a `SynAstResult` into a `prettytable::Row` for tabular display.
    ///
    /// # Returns
    ///
    /// A `Row` containing cells for the rule name, severity, certainty, filename, and match count.
    #[allow(dead_code)]
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
