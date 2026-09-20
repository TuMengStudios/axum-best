use bson::oid::ObjectId;

/// Generates an ID in the format `YYYYMMDDHHmmss` plus the last 8 bytes of a
/// MongoDB ObjectId encoded as lowercase hexadecimal.
pub fn next_id() -> String {
    let object_id = ObjectId::new();
    let data = object_id.bytes();
    let timestamp = object_id
        .timestamp()
        .try_to_rfc3339_string()
        .expect("ObjectId timestamps always fall within the RFC 3339 range");

    let mut id = String::with_capacity(30);
    id.push_str(&timestamp[0..4]);
    id.push_str(&timestamp[5..7]);
    id.push_str(&timestamp[8..10]);
    id.push_str(&timestamp[11..13]);
    id.push_str(&timestamp[14..16]);
    id.push_str(&timestamp[17..19]);
    id.push_str(&hex::encode(&data[4..]));

    id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_id_with_expected_format() {
        let id = next_id();

        assert_eq!(id.len(), 30);
        assert!(id[..14].chars().all(|character| character.is_ascii_digit()));
        assert!(
            id[14..]
                .chars()
                .all(|character| character.is_ascii_hexdigit())
        );
    }
}
