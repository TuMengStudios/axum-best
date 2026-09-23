use redis::{FromRedisValue as _, ToRedisArgs as _, Value};
use redis_derive::{RedisArgs, RedisValue};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, RedisArgs, RedisValue)]
struct Student {
    id: u64,
    name: String,
    age: u8,
    grade: String,
    scores: Vec<u32>,
}

#[derive(Serialize, Deserialize, RedisArgs, RedisValue)]
struct Record<T> {
    value: T,
}

#[derive(Serialize, RedisArgs)]
struct SerializeOnly {
    value: u32,
}

#[derive(Deserialize, RedisValue)]
struct DeserializeOnly {
    value: u32,
}

#[test]
fn serializes_struct_as_one_json_redis_argument() {
    let student = Student {
        id: 7,
        name: "Ada".to_owned(),
        age: 12,
        grade: "A".to_owned(),
        scores: vec![95, 99],
    };

    let args = student.to_redis_args();

    assert_eq!(
        args,
        vec![br#"{"id":7,"name":"Ada","age":12,"grade":"A","scores":[95,99]}"#.to_vec()]
    );
}

#[test]
fn supports_generic_serializable_structs() {
    let record = Record { value: 42_u32 };

    assert_eq!(record.to_redis_args(), vec![br#"{"value":42}"#.to_vec()]);
}

#[test]
fn deserializes_struct_from_redis_json_value() {
    let value = Value::BulkString(
        br#"{"id":7,"name":"Ada","age":12,"grade":"A","scores":[95,99]}"#.to_vec(),
    );

    let student = Student::from_redis_value(&value).unwrap();

    assert_eq!(student.id, 7);
    assert_eq!(student.name, "Ada");
    assert_eq!(student.age, 12);
    assert_eq!(student.grade, "A");
    assert_eq!(student.scores, vec![95, 99]);
}

#[test]
fn invalid_json_returns_redis_type_error() {
    let value = Value::BulkString(b"not json".to_vec());

    let error = Student::from_redis_value(&value).unwrap_err();

    assert_eq!(error.kind(), redis::ErrorKind::TypeError);
    assert!(error.to_string().contains("JSON parse error"));
}

#[test]
fn deserializes_generic_struct_from_redis_json_value() {
    let value = Value::BulkString(br#"{"value":42}"#.to_vec());

    let record = Record::<u32>::from_redis_value(&value).unwrap();

    assert_eq!(record.value, 42);
}

#[test]
fn serialization_and_deserialization_derives_are_independent() {
    let serialized = SerializeOnly { value: 7 };
    assert_eq!(serialized.to_redis_args(), vec![br#"{"value":7}"#.to_vec()]);

    let value = Value::BulkString(br#"{"value":42}"#.to_vec());
    let deserialized = DeserializeOnly::from_redis_value(&value).unwrap();
    assert_eq!(deserialized.value, 42);
}
