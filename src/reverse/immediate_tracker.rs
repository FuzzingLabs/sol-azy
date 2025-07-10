use std::collections::BTreeMap;

/// Tracks ranges of offsets in a program's bytecode that represent immediate values.
///
/// This is used during static analysis to identify and register non-overlapping
/// memory regions, typically representing constants or data accessed via `LD_DW_IMM`.
#[derive(Debug)]
pub struct ImmediateTracker {
    ranges: BTreeMap<usize, usize>, // start => end
    program_len: usize,
}

impl ImmediateTracker {
    /// Creates a new `ImmediateTracker` for a given program length.
    ///
    /// # Arguments
    ///
    /// * `program_len` - The total length of the program bytecode.
    ///
    /// # Returns
    ///
    /// A new `ImmediateTracker` instance with an empty set of ranges.
    pub fn new(program_len: usize) -> Self {
        Self {
            ranges: BTreeMap::new(),
            program_len,
        }
    }

    /// Registers a new offset as the beginning of an immediate value range.
    ///
    /// If the new offset overlaps an existing range, the previous range is truncated
    /// to ensure no overlap occurs. The new range ends at the next registered start,
    /// or at the end of the program.
    ///
    /// # Arguments
    ///
    /// * `new_start` - The byte offset marking the start of a new immediate value.
    pub fn register_offset(&mut self, new_start: usize) {
        // Find where the new range should end: the next registered start, or end of program
        let new_end = self
            .ranges
            .range((new_start + 1)..)
            .map(|(&s, _)| s)
            .next()
            .unwrap_or(self.program_len);

        // Update existing ranges if the new_start falls inside
        for (_start, end) in self.ranges.range_mut(..new_start) {
            if *end > new_start {
                *end = new_start;
            }
        }

        // Insert the new range
        self.ranges.insert(new_start, new_end);
    }

    /// Retrieves the immediate value range that starts at a given offset (used for unit test).
    ///
    /// # Arguments
    ///
    /// * `start` - The offset to look up.
    ///
    /// # Returns
    ///
    /// An optional tuple `(start, end)` representing the registered range,
    /// or `None` if no range begins at that offset.
    pub fn get_range(&self, start: usize) -> Option<(usize, usize)> {
        self.ranges.get(&start).map(|&end| (start, end))
    }

    /// Returns a reference to the internal map of all registered immediate value ranges.
    ///
    /// # Returns
    ///
    /// A reference to a `BTreeMap` mapping start offsets to their corresponding end offsets.
    pub fn get_ranges(&self) -> &BTreeMap<usize, usize> {
        &self.ranges
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the registration logic of immediate value offsets,
    /// ensuring that overlapping ranges are properly truncated and segmented.
    #[test]
    fn test_register_and_truncate() {
        let mut tracker = ImmediateTracker::new(0x100);
        tracker.register_offset(0x10);
        assert_eq!(tracker.get_range(0x10), Some((0x10, 0x100)));

        tracker.register_offset(0x30);
        assert_eq!(tracker.get_range(0x10), Some((0x10, 0x30)));
        assert_eq!(tracker.get_range(0x30), Some((0x30, 0x100)));

        tracker.register_offset(0x20);
        assert_eq!(tracker.get_range(0x10), Some((0x10, 0x20)));
        assert_eq!(tracker.get_range(0x20), Some((0x20, 0x30)));
        assert_eq!(tracker.get_range(0x30), Some((0x30, 0x100)));
    }
}
