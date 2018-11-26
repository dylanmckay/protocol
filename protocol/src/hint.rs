use std::collections::HashMap;

pub type FieldIndex = usize;

/// Hints given when reading parcels.
#[derive(Clone, Debug, PartialEq)]
pub struct Hints {
    pub current_field_index: Option<FieldIndex>,
    /// The fields for which a length prefix
    /// was already present earlier in the layout.
    pub known_field_lengths: HashMap<FieldIndex, usize>,
}


impl Default for Hints {
    fn default() -> Self {
        Hints {
            current_field_index: None,
            known_field_lengths: HashMap::new(),
        }
    }
}

impl Hints {
    /// Gets the length of the field currently being
    /// read, if known.
    pub fn current_field_length(&self) -> Option<usize> {
        self.current_field_index.and_then(|index| self.known_field_lengths.get(&index)).cloned()
    }
}

/// Helpers for the `protocol-derive` crate.
mod protocol_derive_helpers {
    use super::*;

    impl Hints {
        // Sets hints indicating a new set of fields are beginning.
        #[doc(hidden)]
        pub fn begin_fields(&mut self) {
            self.current_field_index = Some(0);
        }

        // Updates the hints to indicate a field was just read.
        #[doc(hidden)]
        pub fn next_field(&mut self) {
            *self.current_field_index.as_mut()
                .expect("cannot increment next field when not in a struct")+= 1;
        }
    }
}

