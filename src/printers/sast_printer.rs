// src/pretty_printer.rs

use crate::parsers::syn_ast::{SourcePosition};
use crate::state::sast_state::{
    Certainty, SastState, Severity, SynAstMap, SynAstMapExt, SynAstResult, SynMatchResult,
    SynRuleMetadata,
};
use anyhow::{Context, Result};
use prettytable::{format, Cell, Row, Table};

#[derive(Debug, Clone)]
pub struct SastPrinter;

impl SastPrinter {
    pub fn print_sast_state(state: &SastState) -> Result<()> {
        println!("Files scanned: {}", state.syn_ast_map.count_files());

        let all_results: Vec<SynAstResult> = state
            .syn_ast_map
            .values()
            .flat_map(|ast| ast.results.clone())
            .collect();

        Self::print_rules_summary(&all_results)?;

        let results_with_matches: Vec<&SynAstResult> = all_results
            .iter()
            .filter(|result| !result.matches.is_empty())
            .collect();

        if !results_with_matches.is_empty() {
            println!("\nDetailed findings:");
            results_with_matches
                .iter()
                .try_for_each(|result| Self::print_result(result, &state.syn_ast_map))?;
        } else {
            println!("\nNo vulnerabilities detected.");
        }

        Ok(())
    }

    pub fn print_rules_summary(results: &[SynAstResult]) -> Result<()> {
        let filtered_results: Vec<&SynAstResult> = results
            .iter()
            .filter(|result| !result.matches.is_empty())
            .collect();

        if filtered_results.is_empty() {
            println!("No rule matches found.");
            return Ok(());
        }

        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_BOX_CHARS);

        table.add_row(Row::new(vec![
            Cell::new("Rule Name").style_spec("bFc"),
            Cell::new("Severity").style_spec("bFc"),
            Cell::new("Certainty").style_spec("bFc"),
            Cell::new("File").style_spec("bFc"),
            Cell::new("Matches").style_spec("bFc"),
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

        filtered_results.iter().for_each(|result| {
            table.add_row(Row::new(vec![
                Cell::new(&result.rule_metadata.name),
                severity_to_cell(&result.rule_metadata.severity),
                certainty_to_cell(&result.rule_metadata.certainty),
                Cell::new(&result.rule_filename),
                Cell::new(&result.matches.len().to_string()),
            ]));
        });

        table.printstd();

        Ok(())
    }

    pub fn print_result(result: &SynAstResult, syn_ast_map: &SynAstMap) -> Result<()> {
        println!("\n{}", "=".repeat(80));
        println!("Rule: {}", result.rule_metadata.name);
        println!("File: {}", result.rule_filename);

        Self::print_metadata(&result.rule_metadata)?;

        if !result.matches.is_empty() {
            println!("\nMatches found: {}", result.matches.len());
            for (i, match_result) in result.matches.iter().enumerate() {
                Self::print_match(match_result, i + 1, 0, syn_ast_map)?;
            }
        } else {
            println!("\nNo matches found.");
        }

        println!("{}", "=".repeat(80));

        Ok(())
    }

    fn print_metadata(metadata: &SynRuleMetadata) -> Result<()> {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

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

    fn print_source_line(code_metadata: &SourcePosition) {
        println!("{}", code_metadata);
    }

    fn print_match(
        match_result: &SynMatchResult,
        index: usize,
        indent: usize,
        syn_ast_map: &SynAstMap,
    ) -> Result<()> {
        println!(
            "{}Match #{}: {} (parent: {})",
            " ".repeat(indent),
            index,
            match_result.ident,
            match_result.parent
        );

        if let Some(code_metadata) = Self::find_ident_position(match_result, syn_ast_map) {
            println!(
                "{}Location: {}",
                " ".repeat(indent + 2),
                code_metadata
            );
            Self::print_source_line(code_metadata);
        } else {
            println!("{}Source location not found", " ".repeat(indent + 2));
        }

        if !match_result.metadata.is_empty() {
            println!("{}Metadata:", " ".repeat(indent + 2));
            for (key, value) in &match_result.metadata {
                println!("{}{}: {}", " ".repeat(indent + 4), key, value);
            }
        }

        if !match_result.children.is_empty() {
            println!("{}Children:", " ".repeat(indent + 2));
            for (child_idx, child) in match_result.children.iter().enumerate() {
                Self::print_match(child, child_idx + 1, indent + 4, syn_ast_map)?;
            }
        }

        Ok(())
    }

    fn find_ident_position<'a>(
        match_result: &'a SynMatchResult,
        syn_ast_map: &'a SynAstMap,
    ) -> Option<&'a SourcePosition> {
        // for syn_ast in syn_ast_map.values() {
        //     if let Some(position) = syn_ast.enriched_ast.get(&match_result.ident) {
        //         return Some(position);
        //     }
        // }
        None
    }

    pub fn print_results_as_json(results: &[SynAstResult]) -> Result<()> {
        let json =
            serde_json::to_string_pretty(results).context("Failed to serialize results to JSON")?;
        println!("{}", json);
        Ok(())
    }
}

impl SynAstResult {
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
