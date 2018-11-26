use std::collections::HashMap;

pub type FieldIndex = usize;

/// Hints given when reading parcels.
#[derive(Clone, Debug)]
pub struct Hints {
    pub current_field_index: FieldIndex,
    /// The fields for which a length prefix
    /// was already present earlier in the layout.
    pub known_field_lengths: HashMap<FieldIndex, usize>,
}


impl Default for Hints {
    fn default() -> Self {
        Hints {
            current_field_index: 0,
            known_field_lengths: HashMap::new(),
        }
    }
}
