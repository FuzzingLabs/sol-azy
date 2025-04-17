use std::collections::BTreeMap;

#[derive(Debug)]
pub struct ImmutableTracker {
    ranges: BTreeMap<usize, usize>, // start => end
    program_len: usize,
}

impl ImmutableTracker {
    pub fn new(program_len: usize) -> Self {
        Self {
            ranges: BTreeMap::new(),
            program_len,
        }
    }

    pub fn register_offset(&mut self, new_start: usize) {
        let new_end = self
            .ranges
            .range((new_start + 1)..)
            .map(|(&s, _)| s)
            .next()
            .unwrap_or(self.program_len);

        let to_truncate: Vec<usize> = self
            .ranges
            .iter()
            .filter_map(|(&start, &end)| {
                if start < new_start && end > new_start {
                    Some(start)
                } else {
                    None
                }
            })
            .collect();

        for start in to_truncate {
            self.ranges.insert(start, new_start);
        }

        self.ranges.insert(new_start, new_end);
    }

    pub fn get_range(&self, start: usize) -> Option<(usize, usize)> {
        self.ranges.get(&start).map(|&end| (start, end))
    }

    pub fn get_ranges(&self) -> &BTreeMap<usize, usize> {
        &self.ranges
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_truncate() {
        let mut tracker = ImmutableTracker::new(0x100);
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
