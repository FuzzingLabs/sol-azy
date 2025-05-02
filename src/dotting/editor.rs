use std::{collections::HashSet, fs, path::PathBuf, path::Path};
use log::debug;
use regex::Regex;
use serde::Deserialize;
use indicatif::{ProgressBar, ProgressIterator};

#[derive(Debug, Deserialize)]
struct Config {
    functions: Vec<String>,
}


pub fn editor_add_functions<P: AsRef<Path> + ToString>(
    json_path: P,    // path to config file (.json)
    reduced_path: P, // path to reduced .dot
    full_path: P,    // path to full .dot
) -> std::io::Result<()> {
    let json_content = std::fs::read_to_string(&json_path)?;
    let config: Config = serde_json::from_str(&json_content)?;
    let requested_clusters: HashSet<String> = config.functions.iter().cloned().collect();

    let mut reduced_dot = std::fs::read_to_string(&reduced_path)?;
    let full_dot = std::fs::read_to_string(&full_path)?;

    debug!("Loading clusters...");
    // Regex to match full subgraph blocks
    let subgraph_re = Regex::new(r"(?s)subgraph cluster_(\d+)\s*\{.*?\}").unwrap();

    debug!("Adding requested subgraphs...");
    // Add requested subgraphs if not already in reduced
    for cap in subgraph_re.captures_iter(&full_dot).collect::<Vec<_>>().iter().progress() {
        let cluster_id = &cap[1];
        let block = &cap[0];

        if requested_clusters.contains(cluster_id) {
            if !reduced_dot.contains(&format!("cluster_{}", cluster_id)) {
                if let Some(pos) = reduced_dot.rfind('}') {
                    reduced_dot.insert_str(pos, &format!("\n{}\n", block));
                }
            }
        }
    }

    // Extract all present basic blocks in reduced dot
    let mut present_lbbs = HashSet::new();
    let lbb_re = Regex::new(r"\b(lbb_\d+)\b").unwrap();
    for cap in lbb_re.captures_iter(&reduced_dot) {
        present_lbbs.insert(cap[1].to_string());
    }

    // Prepare to collect new edges
    let mut new_edges = Vec::new();
    let reduced_lines: HashSet<&str> = reduced_dot.lines().collect();
    let lbb_in_rhs_re = Regex::new(r"\blbb_\d+\b").unwrap();

    // Process full dot line-by-line to find new edges
    debug!("Processing for new edges...");
    for line in full_dot.lines().collect::<Vec<_>>().iter().progress() {
        if reduced_lines.contains(*line) {
            continue;
        }

        if line.contains(" -> {") && !line.contains("style=dotted") {
            if let Some((lhs, rhs)) = line.split_once("->") {
                let src = lhs.trim();
                if present_lbbs.contains(src) {
                    let rhs_trimmed = rhs.trim();
                    if rhs_trimmed.starts_with('{') && rhs_trimmed.ends_with(';') {
                        let inner = &rhs_trimmed[1..rhs_trimmed.len() - 2]; // remove '{' and '};'

                        let filtered: Vec<&str> = lbb_in_rhs_re
                            .find_iter(inner)
                            .map(|m| m.as_str())
                            .filter(|lbb| present_lbbs.contains(*lbb))
                            .collect();

                        if !filtered.is_empty() {
                            let cleaned_line = format!("{} -> {{{}}};", src, filtered.join(" "));
                            new_edges.push(cleaned_line);
                        }
                    }
                }
            }
        }
    }

    // Inject new edges before the last closing brace
    if let Some(pos) = reduced_dot.rfind('}') {
        reduced_dot.insert_str(pos, &format!("\n{}\n", new_edges.join("\n")));
    }

    // Save updated reduced dot
    let mut out_path = "updated_".to_string();
    out_path.push_str(&reduced_path.to_string());

    std::fs::write(&out_path, reduced_dot)?;
    debug!("Updated file saved to {:?}", out_path);

    Ok(())
}


#[test]
fn test() -> Result<(), Box<dyn std::error::Error>> {
    editor_add_functions(
        "src/dotting/functions.json",
        "src/dotting/cfg_reduced.dot",
        "src/dotting/cfg.dot"
    )?;
    Ok(())
}
