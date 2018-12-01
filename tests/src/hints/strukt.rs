use protocol::{hint, Settings};
use super::{SaveHints, HasSavedHints};

#[derive(Protocol, Debug, PartialEq)]
pub struct WithNamedFields {
    pub f0: SaveHints<u8>,
    pub f1: SaveHints<String>,
    pub f2: SaveHints<u64>,
    pub f3: SaveHints<bool>,
    pub f4: SaveHints<FooBar>,
    pub f5: SaveHints<[u8; 16]>,
}

#[derive(Protocol, Debug, PartialEq)]
pub struct WithUnnamedFields(
    SaveHints<i64>,
    SaveHints<String>,
    SaveHints<bool>,
    SaveHints<u8>,
    SaveHints<Vec<u8>>,
    SaveHints<Vec<char>>,
);

#[derive(Protocol, Debug, PartialEq)]
pub struct FooBar(pub u32);

define_common_hint_invariant_tests!(with_named_fields => WithNamedFields : WithNamedFields::default());
define_common_hint_invariant_tests!(with_unnamed_fields => WithUnnamedFields : WithUnnamedFields::default());

mod named_fields {
    use protocol::Parcel;
    use super::*;

    #[test]
    fn current_field_index_is_incremented() {
        let settings = Settings::default();

        let test_struct = WithNamedFields::default();
        let read_back = WithNamedFields::read_new(&mut test_struct.into_stream(&settings).unwrap(), &settings).unwrap();

        assert_eq!(Some(0), read_back.f0.hints().current_field_index);
        assert_eq!(Some(1), read_back.f1.hints().current_field_index);
        assert_eq!(Some(2), read_back.f2.hints().current_field_index);
        assert_eq!(Some(3), read_back.f3.hints().current_field_index);
        assert_eq!(Some(4), read_back.f4.hints().current_field_index);
        assert_eq!(Some(5), read_back.f5.hints().current_field_index);
    }
}

mod unnamed_fields {
    use protocol::Parcel;
    use super::*;

    #[test]
    fn current_field_index_is_incremented() {
        let settings = Settings::default();

        let test_struct = WithUnnamedFields::default();
        let read_back = WithUnnamedFields::read_new(&mut test_struct.into_stream(&settings).unwrap(), &settings).unwrap();

        let WithUnnamedFields(f0, f1, f2, f3, f4, f5) = read_back;

        assert_eq!(Some(0), f0.hints().current_field_index);
        assert_eq!(Some(1), f1.hints().current_field_index);
        assert_eq!(Some(2), f2.hints().current_field_index);
        assert_eq!(Some(3), f3.hints().current_field_index);
        assert_eq!(Some(4), f4.hints().current_field_index);
        assert_eq!(Some(5), f5.hints().current_field_index);
    }
}

impl Default for WithNamedFields {
    fn default() -> Self {
        WithNamedFields {
            f0: 1.into(),
            f1: "hello".to_owned().into(),
            f2: (!0).into(),
            f3: false.into(),
            f4: FooBar(0xffaabb00).into(),
            f5: [9; 16].into(),
        }
    }
}

impl Default for WithUnnamedFields {
    fn default() -> Self {
        WithUnnamedFields(
            99.into(), "hello".to_owned().into(), true.into(),
            127.into(), vec![9,8,7].into(), vec!['a', 'p'].into(),
        )
    }
}

impl HasSavedHints for WithNamedFields {
    fn saved_hints_after_reading(&self) -> &hint::Hints {
        self.f5.hints()
    }
}

impl HasSavedHints for WithUnnamedFields {
    fn saved_hints_after_reading(&self) -> &hint::Hints {
        let &WithUnnamedFields(_, _, _, _, _, ref last) = self;
        last.hints()
    }
}

