use std::{collections::HashSet, fs, path::PathBuf};
use regex::Regex;
use serde::Deserialize;
use indicatif::{ProgressBar, ProgressIterator};

#[derive(Debug, Deserialize)]
struct Config {
    functions: Vec<String>,
}

#[test]
pub fn test() -> Result<(), Box<dyn std::error::Error>> {
    let json_path = PathBuf::from("src/dotting/functions.json");
    let reduced_path = PathBuf::from("src/dotting/cfg_reduced.dot");
    let full_path = PathBuf::from("src/dotting/cfg.dot");
    
    let json_content = fs::read_to_string(&json_path)?;
    let config: Config = serde_json::from_str(&json_content)?;
    let requested_clusters: HashSet<String> = config.functions.iter().cloned().collect();

    let mut reduced_dot = fs::read_to_string(&reduced_path)?;
    let full_dot = fs::read_to_string(&full_path)?;

    // Append requested subgraphs from the full dot file to the reduced one
    let subgraph_re = Regex::new(r"(?s)subgraph cluster_(\d+)\s*\{.*?\}").unwrap();

    // Append requested subgraphs from the full dot file to the reduced one
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

    // Extract all lbb_xxxx basic blocks currently in the reduced dot
    let mut present_lbbs = HashSet::new();
    let lbb_re = Regex::new(r"\b(lbb_\d+)\b").unwrap();
    for cap in lbb_re.captures_iter(&reduced_dot).collect::<Vec<_>>().iter().progress() {
        present_lbbs.insert(cap[1].to_string());
    }

    let mut new_edges = Vec::new();

    // Index all lines in the reduced dot for fast existence checks
    let reduced_lines: HashSet<&str> = reduced_dot.lines().collect();

    // Process edge lines with fast lookup
    for line in full_dot.lines().collect::<Vec<_>>().iter().progress() {
        // Skip if already included
        if reduced_lines.contains(*line) {
            continue;
        }
    
        // Only consider lines that define real (non-dotted) edges
        if line.contains(" -> {") && !line.contains("style=dotted") {
            if let Some((lhs, rhs)) = line.split_once("->") {
                let src = lhs.trim();
                
                if present_lbbs.contains(src) {
                    let rhs_trimmed = rhs.trim();
                    // Always expect { ... } on right-hand side
                    if rhs_trimmed.starts_with('{') && rhs_trimmed.ends_with(';') {
                        let inner = &rhs_trimmed[1..rhs_trimmed.len() - 2]; // removing "{" and  "};"
                    
                        // Extract all lbb_xxxx using regex
                        let lbb_in_rhs_re = Regex::new(r"\blbb_\d+\b").unwrap();
                        let filtered: Vec<&str> = lbb_in_rhs_re
                            .find_iter(inner)
                            .map(|m| m.as_str())
                            .filter(|lbb| present_lbbs.contains(*lbb))
                            .collect();
                        println!("hello3");

                        if !filtered.is_empty() {
                            let cleaned_line = format!("{} -> {{{}}};", src, filtered.join(" "));
                            new_edges.push(cleaned_line);
                        }
                    }                    
                }
            }
        }
    }
    

    // Insert new edges before the final closing brace
    if let Some(pos) = reduced_dot.rfind('}') {
        println!("DEBUG! {}", &format!("\n{}\n", new_edges.join("\n")));
        reduced_dot.insert_str(pos, &format!("\n{}\n", new_edges.join("\n")));
    }

    fs::write("src/dotting/cfg_reduced_updated.dot", reduced_dot)?;
    println!("✅ Updated file saved as cfg_reduced_updated.dot");

    Ok(())
}
