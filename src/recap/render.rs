use super::rows::Row;

pub(crate) fn to_markdown(rows: &[Row]) -> String {
    let mut s = String::new();
    s.push_str("| Instruction | Signers | Writable | Constrained | Seeded | Memory |\n");
    s.push_str("|---|---|---|---|---|---|\n");
    for r in rows {
        let signers = if r.signers.is_empty() {
            "—".to_string()
        } else {
            r.signers.join(", ")
        };
        let writables = if r.writables.is_empty() {
            "—".to_string()
        } else {
            r.writables.join(", ")
        };
        let constrained = if r.constrained.is_empty() {
            "—".to_string()
        } else {
            r.constrained.join("; ")
        };
        let seeded = if r.seeded.is_empty() {
            "—".to_string()
        } else {
            r.seeded.join(", ")
        };
        let memory = if r.memory.is_empty() {
            "—".to_string()
        } else {
            r.memory.join("; ")
        };
        s.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} |\n",
            r.instruction, signers, writables, constrained, seeded, memory
        ));
    }
    s
}
