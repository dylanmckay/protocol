use protocol::{Parcel, Settings};

#[derive(Protocol, Debug, PartialEq, Eq)]
struct SkipIfField {
    pub condition: bool,
    #[protocol(skip_if("condition"))]
    pub message: Option<u8>,
}

#[derive(Protocol, Debug, PartialEq, Eq)]
struct SkipIfPath {
    pub condition: bool,
    #[protocol(skip_if(condition))]
    pub message: Option<u8>,
}

#[derive(Protocol, Debug, PartialEq, Eq)]
struct SkipIfBinaryExp {
    pub condition: u8,
    #[protocol(skip_if("condition == 7"))]
    pub message: Option<u8>,
}

#[derive(Protocol, Debug, PartialEq, Eq)]
struct SkipIfBinaryExpWithMultipleFieldReference {
    pub condition_1: u8,
    pub condition_2: bool,
    #[protocol(skip_if("condition_1 == 7 && condition_2"))]
    pub message: Option<u8>,
}

#[derive(Protocol, Debug, PartialEq, Eq)]
struct ReadOptionStillWorks {
    pub message: Option<u8>,
}

#[test]
fn should_read_option_without_skip() {
    assert_eq!(ReadOptionStillWorks {
        message: Some(42),
    }, ReadOptionStillWorks::from_raw_bytes(&[1, 42], &Settings::default()).unwrap());

    assert_eq!(ReadOptionStillWorks {
        message: None,
    }, ReadOptionStillWorks::from_raw_bytes(&[0], &Settings::default()).unwrap());
}

#[test]
fn should_skip_when_field_condition() {
    assert_eq!(SkipIfField {
        condition: false,
        message: Some(8),
    }, SkipIfField::from_raw_bytes(&[0, 8], &Settings::default()).unwrap());
}

#[test]
fn should_not_skip_field_when_not_field_condition() {
    assert_eq!(SkipIfField {
        condition: true,
        message: None,
    }, SkipIfField::from_raw_bytes(&[ 1 ], &Settings::default()).unwrap());
}

#[test]
fn should_skip_when_path_condition() {
    assert_eq!(SkipIfPath {
        condition: false,
        message: Some(8),
    }, SkipIfPath::from_raw_bytes(&[0, 8], &Settings::default()).unwrap());
}

#[test]
fn should_not_skip_field_when_not_path_condition() {
    assert_eq!(SkipIfPath {
        condition: true,
        message: None,
    }, SkipIfPath::from_raw_bytes(&[ 1 ], &Settings::default()).unwrap());
}


#[test]
fn should_skip_field_when_binary_exp_condition_is_met() {
    assert_eq!(SkipIfBinaryExp {
        condition: 7,
        message: None,
    }, SkipIfBinaryExp::from_raw_bytes(&[ 7 ], &Settings::default()).unwrap());
}

#[test]
fn should_not_skip_field_when_binary_exp_condition_is_not_met() {
    assert_eq!(SkipIfBinaryExp {
        condition: 1,
        message: Some(1),
    }, SkipIfBinaryExp::from_raw_bytes(&[ 1, 1 ], &Settings::default()).unwrap());
}

#[test]
fn should_skip_field_with_multiple_condition() {
    assert_eq!(SkipIfBinaryExpWithMultipleFieldReference {
        condition_1: 7,
        condition_2: true,
        message: None,
    }, SkipIfBinaryExpWithMultipleFieldReference::from_raw_bytes(&[ 7, 1], &Settings::default()).unwrap());
}

#[test]
fn should_not_skip_field_with_multiple_condition() {
    assert_eq!(SkipIfBinaryExpWithMultipleFieldReference {
        condition_1: 7,
        condition_2: false,
        message: Some(1),
    }, SkipIfBinaryExpWithMultipleFieldReference::from_raw_bytes(&[ 7, 0, 1], &Settings::default()).unwrap());
}




