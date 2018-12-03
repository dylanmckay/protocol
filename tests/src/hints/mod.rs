//! Tests the derived `Parcel` implementations and ensures
//! various invariants are followed.

use protocol::{Error, Parcel, Settings};
use protocol::hint;
use std::io;

/// Wraps another Parcel and saves the hints at read time.
#[derive(Debug, PartialEq)]
pub struct SaveHints<T> where T: Parcel {
    pub inner: T,
    saved_hints: Option<hint::Hints>,
}

impl<T> SaveHints<T>
    where T: Parcel {
    pub fn hints(&self) -> &hint::Hints {
        self.saved_hints.as_ref()
            .expect("there are no saved hints for manually built values")
    }
}

impl<T> Parcel for SaveHints<T>
    where T: Parcel {
    const TYPE_NAME: &'static str = "SaveHints";

    fn read_field(read: &mut io::Read, settings: &Settings,
                  hints: &mut hint::Hints) -> Result<Self, Error> {
        let saved_hints = Some(hints.clone());
        let inner = T::read_field(read, settings, hints)?;
        Ok(SaveHints { inner, saved_hints })
    }

    fn write_field(&self, write: &mut io::Write,
                   settings: &Settings,
                   _: &mut hint::Hints) -> Result<(), Error> {
        self.inner.write(write, settings)
    }
}

impl<T> From<T> for SaveHints<T> where T: Parcel {
    fn from(v: T) -> Self {
        SaveHints {
            inner: v,
            saved_hints: None,
        }
    }
}

trait HasSavedHints: Parcel {
    fn saved_hints_after_reading(&self) -> &hint::Hints;
}

/// Writes a value to a stream, reads it back with the given hints,
/// and returns the hint state after the read.
fn get_hints_after_read<P>(mut input_hints: hint::Hints,
                           input_value: P)
    -> hint::Hints
    where P: HasSavedHints + Default {
    let mut parcel_stream = input_value.into_stream(&Settings::default()).unwrap();
    let v = P::read_field(&mut parcel_stream, &Settings::default(), &mut input_hints).unwrap();

    v.saved_hints_after_reading().clone()
}

/// Use this macro in places where we should probably
/// add new tests when adding new hints.
///
/// It triggers an unmentioned field in struct pattern error
/// until the new field is mentioned in the callsite.
macro_rules! force_contributor_to_acknowledge_new_hints {
    ( $( $field:ident ),* ) => {
        {
            // This is here so new hints must have tests added due
            // to exhaustive pattern matching.
            #[allow(unused_variables)]
            let hint::Hints { $( $field ),* } = hint::Hints::default();
        }
    };
}

/// Defines a bunch of test functions that test invariants
/// true for hints on every type.
macro_rules! define_common_hint_invariant_tests {
    ($parcel_description:ident => $parcel_ty:ty : $parcel_value:expr) => {
        mod $parcel_description {
            use super::*;
            use hints::*;
            use protocol::hint;

            const PRETTY_LARGE_NUMBER: usize = 10_000;

            /// Whenever any automatically derived type is read,
            #[test]
            fn all_prior_hints_are_ignored_when_reading() {

                let mut hints = hint::Hints::default();

                force_contributor_to_acknowledge_new_hints!(
                    current_field_index, known_field_lengths
                );

                // Set current field index to its maximum value, so that
                // if the system is actually incremented, it'll panic.
                hints.current_field_index = Some(usize::max_value());
                // Insert three bullshit field lengths.
                hints.known_field_lengths = (0..PRETTY_LARGE_NUMBER).into_iter().map(|i| {
                    (i, protocol::hint::FieldLength { length: i+i/2, kind: protocol::hint::LengthPrefixKind::Bytes })
                }).collect();

                let hints_afterwards = get_hints_after_read::<$parcel_ty>(hints, $parcel_value);

                assert!(hints_afterwards.current_field_index.unwrap() < PRETTY_LARGE_NUMBER);
                assert!(hints_afterwards.known_field_lengths.len() < PRETTY_LARGE_NUMBER);
            }
        }
    };
}

pub mod enums;
pub mod strukt;

