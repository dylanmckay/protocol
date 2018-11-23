#![cfg(test)]

use protocol::Parcel;

#[derive(Protocol, Clone, Debug, PartialEq)]
pub enum WithGenerics<A, B> {
    Foo(A, B),
    Bar,
}

mod string_discriminants {
    #[allow(unused_imports)]
    use protocol::{Parcel, Settings};

    #[derive(Protocol, Clone, Debug, PartialEq)]
    #[protocol]
    pub enum PlayerState {
      Stationary,
      Flying { velocity: (f32,f32,f32) },
      Jumping { height: f32 },
    }

    #[derive(Protocol, Debug, PartialEq)]
    #[protocol(discriminant = "string")]
    pub enum Axis { X, Y, Z, Other(String), Bimp { val: u64 } }

    #[derive(Protocol, Debug, PartialEq)]
    #[protocol(discriminant = "string")]
    pub enum RenamedVariant {
        Hello,
        #[protocol(name = "Universe")]
        World,
    }

    #[test]
    fn variant_names_are_discriminators() {
        let settings = Settings::default();
        assert_eq!(vec![0, 0, 0, 1, 'X' as _], Axis::X.raw_bytes(&settings).unwrap());
        assert_eq!(vec![0, 0, 0, 5, 'O' as _, 't' as _, 'h' as _, 'e' as _, 'r' as _,
                        0, 0, 0, 4, 'r' as _, 'o' as _, 'l' as _, 'l' as _],
                   Axis::Other("roll".to_owned()).raw_bytes(&settings).unwrap());
    }

    verify_read_back!(mixed_enum_tuple_variant => Axis::Other("boop".to_owned()));
    verify_read_back!(mixed_enum_unit_variant1 => Axis::X);
    verify_read_back!(mixed_enum_unit_variant2 => Axis::Y);
    verify_read_back!(mixed_enum_struct_variant => Axis::Bimp { val: 77 });

    #[test]
    fn renamed_variants_are_transmitted() {
        let settings = Settings::default();

        assert_eq!(vec![0, 0, 0, 5, 'H' as _, 'e' as _, 'l' as _, 'l' as _, 'o' as _], RenamedVariant::Hello.raw_bytes(&settings).unwrap());
        assert_eq!(vec![0, 0, 0, 8, 'U' as _, 'n' as _, 'i' as _, 'v' as _, 'e' as _, 'r' as _, 's' as _, 'e' as _], RenamedVariant::World.raw_bytes(&settings).unwrap());
    }

    verify_read_back!(variant_with_custom_name_attribute => RenamedVariant::World);
}

mod generics {
    use std::fmt;

    #[derive(Protocol, Debug, PartialEq)]
    pub enum EnumWithEmptyGenerics<> { First { a: u32, b: String, c: u64 } }
    #[derive(Protocol, Debug, PartialEq)]
    pub enum EnumWithUnconstrainedType<T> { Variant1 { a: T, b: T }, Variant2 { c: T } }
    #[derive(Protocol, Debug, PartialEq)]
    pub enum EnumWithUnconstrainedTypes<A,B,C,D> { Value { a: A, b: B, c: C, d: D }, Variant2 { a: A } }
    #[derive(Protocol, Debug, PartialEq)]
    pub enum EnumWithConstrainedType<T: Clone + PartialEq + fmt::Debug + fmt::Display> { Variant1 { inner: T }, Variant2 { c: T } }
    #[derive(Protocol, Debug, PartialEq)]
    pub enum EnumWithConstrainedTypes<T: Clone, A: fmt::Debug + fmt::Display, B: Copy> { Variant1 { t: T, a: A, b: B }, Variant2 { c: T } }
    #[derive(Protocol, Debug, PartialEq)]
    pub enum EnumWithWhereClause<T> where T: fmt::Debug + fmt::Display { Variant1 { t: T }, Variant2 { t: T } }
    #[derive(Protocol, Debug, PartialEq)]
    pub enum EnumWithWhereClauses<A,B,C> where A: Copy, B: fmt::Debug + fmt::Display, C: Clone + Copy { Variant1 { a: A, b: B, c: C }, Variant2 { a: A } }


    verify_read_back!(enum_with_empty_generics => EnumWithEmptyGenerics::First { a: 22, b: "boop".to_owned(), c: !0 });

    verify_read_back!(single_unconstrained_type => EnumWithUnconstrainedType::Variant2 { c: "hello".to_string() });

    verify_read_back!(multiple_unconstrained_types => EnumWithUnconstrainedTypes::Value {
        a: "hello".to_string(), b: 55u8, c: 128u64, d: 99i64,
    });

    verify_read_back!(single_constrained_type => EnumWithConstrainedType::Variant1 { inner: "hello".to_string() });

    verify_read_back!(multiple_constrained_types => EnumWithConstrainedTypes::Variant1 { t: "hello".to_string(), a: 250u8, b: 155i16 });

    verify_read_back!(where_clause => EnumWithWhereClause::Variant1 { t: "hello".to_owned() });

    verify_read_back!(where_clauses => EnumWithWhereClauses::Variant1 { a: 7u16, b: "hello".to_owned(), c: 99u8 });
}

mod integer_discriminants {
    #[allow(unused_imports)]
    use protocol::{Parcel, Settings};

    #[derive(Protocol, Debug, PartialEq, Eq)]
    #[protocol(discriminant = "integer")]
    pub enum BoatKind {
        Speedboat { warp_speed_enabled: bool },
        Dingy(u8, u8),
        Fart,
    }

    #[derive(Protocol, Debug, PartialEq, Eq)]
    #[protocol(discriminant = "integer")]
    #[repr(u8)]
    enum WithCustomRepr {
        First = 1,
        Second = 2,
    }

    #[derive(Protocol, Debug, PartialEq, Eq)]
    #[protocol(discriminant = "integer")]
    #[repr(i8)]
    enum WithoutExplicitDiscriminators {
        Only,
    }

    #[test]
    fn discriminator_zero_is_reserved() {
        assert_eq!(vec![1],
                   WithoutExplicitDiscriminators::Only.raw_bytes(&protocol::Settings::default()).unwrap());
    }

    #[test]
    fn named_fields_are_correctly_written() {
        assert_eq!(vec![0, 0, 0, 1, 1], BoatKind::Speedboat {
            warp_speed_enabled: true,
        }.raw_bytes(&protocol::Settings::default()).unwrap());
    }

    #[test]
    fn unnamed_fields_are_correctly_written() {
        assert_eq!(vec![0, 0, 0, 2, // discriminator
                        0xf1, 0xed], BoatKind::Dingy(0xf1, 0xed).raw_bytes(&Settings::default()).unwrap());
    }

    #[test]
    fn unit_variants_are_correctly_written() {
        assert_eq!(vec![0, 0, 0, 3], // discriminator
                   BoatKind::Fart.raw_bytes(&Settings::default()).unwrap());
    }

    #[test]
    fn named_fields_are_correctly_read() {
        assert_eq!(BoatKind::Speedboat {
            warp_speed_enabled: true,
        }, BoatKind::from_raw_bytes(&[0, 0, 0, 1, 1], &Settings::default()).unwrap());
    }

    #[test]
    fn unnamed_fields_are_correctly_read() {
        assert_eq!(BoatKind::Dingy(99, 78),
                   BoatKind::from_raw_bytes(&[0, 0, 0, 2, 99, 78], &Settings::default()).unwrap());
    }

    #[test]
    fn unit_variants_are_correctly_read() {
        assert_eq!(BoatKind::Fart,
                   BoatKind::from_raw_bytes(&[0, 0, 0, 3], &Settings::default()).unwrap());
    }

    #[test]
    fn custom_int_discriminator_repr_is_respected() {
        assert_eq!(vec![1], WithCustomRepr::First.raw_bytes(&Settings::default()).unwrap());
    }
}

#[derive(Protocol)]
enum OneVariant { A }

#[derive(Protocol)]
enum BuzzyBee { B(u32, u32) }

#[test]
fn type_name_is_correct() {
    assert_eq!("OneVariant", OneVariant::A.type_name());
    assert_eq!("BuzzyBee", BuzzyBee::B(2,1).type_name());
}
