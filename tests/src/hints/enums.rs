use protocol::{hint, Settings};
use super::{SaveHints, HasSavedHints};

#[derive(protocol::Protocol, Debug, PartialEq)]
pub enum WithNamedFields {
    OnlyVariant {
        f0: SaveHints<u8>,
        f1: SaveHints<String>,
        f2: SaveHints<u64>,
        f3: SaveHints<bool>,
        f4: SaveHints<i32>,
        f5: SaveHints<[u8; 5]>,
    },
}

#[derive(protocol::Protocol, Debug, PartialEq)]
pub enum WithUnnamedFields {
    OnlyVariant(
        SaveHints<u8>,
        SaveHints<String>,
        SaveHints<u64>,
        SaveHints<bool>,
        SaveHints<i32>,
        SaveHints<[u8; 5]>,
    ),
}

define_common_hint_invariant_tests!(with_named_fields => WithNamedFields : WithNamedFields::default());
define_common_hint_invariant_tests!(with_unnamed_fields => WithUnnamedFields : WithUnnamedFields::default());

mod named_fields {
    use protocol::Parcel;
    use super::*;

    #[test]
    fn current_field_index_is_incremented() {
        let settings = Settings::default();

        let test_struct = WithNamedFields::default();
        let read_back = WithNamedFields::read(&mut test_struct.into_stream(&settings).unwrap(), &settings).unwrap();

        match read_back {
            WithNamedFields::OnlyVariant { f0, f1, f2, f3, f4, f5 } => {
                assert_eq!(Some(0), f0.hints().current_field_index);
                assert_eq!(Some(1), f1.hints().current_field_index);
                assert_eq!(Some(2), f2.hints().current_field_index);
                assert_eq!(Some(3), f3.hints().current_field_index);
                assert_eq!(Some(4), f4.hints().current_field_index);
                assert_eq!(Some(5), f5.hints().current_field_index);
            },
        }
    }
}

mod unnamed_fields {
    use protocol::Parcel;
    use super::*;

    #[test]
    fn current_field_index_is_incremented() {
        let settings = Settings::default();

        let test_struct = WithUnnamedFields::default();
        let read_back = WithUnnamedFields::read(&mut test_struct.into_stream(&settings).unwrap(), &settings).unwrap();

        match read_back {
            WithUnnamedFields::OnlyVariant(f0, f1, f2, f3, f4, f5) => {
                assert_eq!(Some(0), f0.hints().current_field_index);
                assert_eq!(Some(1), f1.hints().current_field_index);
                assert_eq!(Some(2), f2.hints().current_field_index);
                assert_eq!(Some(3), f3.hints().current_field_index);
                assert_eq!(Some(4), f4.hints().current_field_index);
                assert_eq!(Some(5), f5.hints().current_field_index);
            },
        }
    }
}

impl Default for WithNamedFields {
    fn default() -> Self {
        WithNamedFields::OnlyVariant {
            f0: 99.into(), f1: "hello".to_owned().into(), f2: 77.into(),
            f3: false.into(), f4: 333.into(), f5: [1,2,3,4,5].into(),
        }
    }
}

impl HasSavedHints for WithNamedFields {
    fn saved_hints_after_reading(&self) -> &hint::Hints {
        match *self {
            WithNamedFields::OnlyVariant { ref f5, .. } => f5.hints(),
        }
    }
}

impl Default for WithUnnamedFields {
    fn default() -> Self {
        WithUnnamedFields::OnlyVariant(
            99.into(), "hello".to_owned().into(), 77.into(),
            false.into(), 333.into(), [1,2,3,4,5].into(),
        )
    }
}

impl HasSavedHints for WithUnnamedFields {
    fn saved_hints_after_reading(&self) -> &hint::Hints {
        match *self {
            WithUnnamedFields::OnlyVariant(_,_,_,_,_, ref f5) => f5.hints(),
        }
    }
}

