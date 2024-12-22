use serde::{de::DeserializeOwned, Deserialize, Serialize};

fn roundtrip<T>(val: T) -> T
where
    T: Serialize + DeserializeOwned,
{
    let serialized = postcard::to_allocvec(&val).unwrap();
    let deserialized: T = postcard::from_bytes(&serialized).unwrap();
    deserialized
}

#[test]
fn skipped_fields() {
    #[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
    struct Newtype(#[serde(skip)] u8);

    #[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
    struct Tuple(#[serde(skip)] u8, u16);

    #[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
    struct Struct {
        #[serde(skip)]
        a: u8,
        b: u16,
    }

    #[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
    enum Enum {
        A(#[serde(skip)] u8),
        B(#[serde(skip)] u8, u16),
        C {
            #[serde(skip)]
            a: u8,
            b: u16,
        },
    }

    // Serde doesn't seem to handle #[serde(skip)] on newtype struct fields,
    // but it does on newtype variant fields
    assert_eq!(roundtrip(Newtype(10)), Newtype(10));
    assert_eq!(roundtrip(Enum::A(10)), Enum::A(0));

    assert_eq!(roundtrip(Tuple(10, 20)), Tuple(0, 20));
    assert_eq!(roundtrip(Struct { a: 10, b: 20 }), Struct { a: 0, b: 20 });
    assert_eq!(roundtrip(Enum::B(10, 20)), Enum::B(0, 20));
    assert_eq!(roundtrip(Enum::C { a: 10, b: 20 }), Enum::C { a: 0, b: 20 });
}
