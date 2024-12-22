use postcard_schema::{
    schema::{DataModelType, DataModelVariant, NamedType, NamedValue, NamedVariant},
    Schema,
};

#[test]
fn skipped_fields() {
    #[derive(Schema)]
    #[allow(dead_code)]
    struct Newtype(#[serde(skip)] u8);

    #[derive(Schema)]
    #[allow(dead_code)]
    struct Tuple(#[serde(skip)] u8, u16);

    #[derive(Schema)]
    #[allow(dead_code)]
    struct Struct {
        #[serde(skip)]
        a: u8,
        b: u16,
    }

    #[derive(Schema)]
    #[allow(dead_code)]
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
    // but it does on newtype variant fields (turning them into unit variants)
    assert_eq!(
        Newtype::SCHEMA,
        &NamedType {
            name: "Newtype",
            ty: &DataModelType::NewtypeStruct(u8::SCHEMA)
        }
    );
    assert_eq!(
        Enum::SCHEMA,
        &NamedType {
            name: "Enum",
            ty: &DataModelType::Enum(&[
                &NamedVariant {
                    name: "A",
                    // Unit variant instead of newtype variant because field is skipped
                    ty: &DataModelVariant::UnitVariant
                },
                &NamedVariant {
                    name: "B",
                    // No `u8` since it is skipped
                    ty: &DataModelVariant::TupleVariant(&[u16::SCHEMA])
                },
                &NamedVariant {
                    name: "C",
                    ty: &DataModelVariant::StructVariant(
                        // No `a` since it is skipped
                        &[&NamedValue {
                            name: "b",
                            ty: u16::SCHEMA
                        }]
                    )
                },
            ])
        }
    );

    assert_eq!(
        Tuple::SCHEMA,
        &NamedType {
            name: "Tuple",
            // No `u8` since it is skipped
            ty: &DataModelType::TupleStruct(&[u16::SCHEMA])
        }
    );
    assert_eq!(
        Struct::SCHEMA,
        &NamedType {
            name: "Struct",
            ty: &DataModelType::Struct(
                // No `a` since it is skipped
                &[&NamedValue {
                    name: "b",
                    ty: u16::SCHEMA
                }]
            )
        }
    );
}
