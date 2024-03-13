use rand::{distributions::WeightedIndex, prelude::*};

/// A simplified weighted-random table.
pub struct DecisionTable<T> {
    options: Vec<(usize, T)>,
}

impl<T> DecisionTable<T> {
    /// Creates an empty table.
    pub fn new() -> DecisionTable<T> {
        DecisionTable { options: vec![] }
    }

    /// Adds an option to the table with a specified weight.
    pub fn push(&mut self, item: (usize, T)) {
        self.options.push(item);
    }

    /// Randomly removes and returns a single item from the table.
    pub fn decide(&mut self) -> Option<T> {
        match WeightedIndex::new(self.options.iter().map(|item| item.0)) {
            Ok(windex) => {
                let mut rng = thread_rng();
                Some(self.options.swap_remove(windex.sample(&mut rng)).1)
            }
            Err(_) => None,
        }
    }

    pub fn len(&self) -> usize {
        self.options.len()
    }

    pub fn is_empty(&self) -> bool {
        self.options.is_empty()
    }
}

impl<T> DecisionTable<T>
where
    T: Clone,
{
    /// Picks a single item from the table and returns a clone.
    pub fn decide_clone(&self) -> Option<T> {
        match WeightedIndex::new(self.options.iter().map(|item| item.0)) {
            Ok(windex) => {
                let mut rng = thread_rng();
                Some(self.options[windex.sample(&mut rng)].1.clone())
            }
            Err(_) => None,
        }
    }
}

impl<T> Default for DecisionTable<T> {
    fn default() -> Self {
        Self::new()
    }
}
