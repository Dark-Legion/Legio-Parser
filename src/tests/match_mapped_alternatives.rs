use crate::{result::TransformMappedMatch, traits::*};

fn match_mapped_alternatives_str_test(data: &str) {
    enum HexColourType {
        ThreeDigit,
        SixDigit,
    }

    enum ValueType {
        HexColour(HexColourType),
        UnsignedInteger,
        Float,
    }

    let (matched, _): (Option<(&str, ValueType)>, &str) = data
        .mapped_alternatives()
        .add_path(|rest| {
            rest.match_static("#")
                .match_exact_with_mapped(3, char::is_ascii_hexdigit, HexColourType::SixDigit)
                .optional_ref(|_, rest| {
                    rest.match_exact_with_mapped(
                        3,
                        char::is_ascii_hexdigit,
                        HexColourType::ThreeDigit,
                    )
                })
                .transform_matched(|_| &rest[..(data.len() - rest.len())])
                .transform_mapped(ValueType::HexColour)
        })
        .add_path(|rest| {
            rest.match_min_with_mapped(1, char::is_ascii_digit, ValueType::UnsignedInteger)
                .optional_ref(|_, rest| {
                    rest.match_static(".").match_min_with_mapped(
                        1,
                        char::is_ascii_digit,
                        ValueType::Float,
                    )
                })
        })
        .finalize()
        .transform_full(|_, rest, mapped| {
            TransformMappedMatch::Full(&data[..(data.len() - rest.len())], rest, mapped)
        })
        .unwrap();

    let (matched, value_type): (&str, ValueType) = matched.unwrap();

    match value_type {
        ValueType::HexColour(_) => {
            let (_red, _green, _blue): (u8, u8, u8) = (
                u8::from_str_radix(&matched[1..3], 16).unwrap(),
                u8::from_str_radix(&matched[3..5], 16).unwrap(),
                u8::from_str_radix(&matched[5..7], 16).unwrap(),
            );
        }
        ValueType::Float => {
            matched.parse::<f64>().unwrap();
        }
        ValueType::UnsignedInteger => {
            matched.parse::<u128>().unwrap();
        }
    }
}

#[test]
fn match_mapped_alternatives_str() {
    match_mapped_alternatives_str_test("123.456");
}

#[test]
#[should_panic]
fn match_mapped_alternatives_str_panic() {
    match_mapped_alternatives_str_test("#123XYZ");
}
