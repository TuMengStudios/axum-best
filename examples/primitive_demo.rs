use axum_best::models::primitive::Primitive;

fn main() {
    println!("=== Primitive 枚举使用示例 ===");

    // 创建不同类型的 Primitive 值
    let values = vec![
        Primitive::I32(42),
        Primitive::from(3.14f64),
        Primitive::from(true),
        Primitive::from("Hello, World!"),
        Primitive::from('A'),
        Primitive::U8(255),
        Primitive::Null,
    ];

    // 显示每个值的信息
    for value in values {
        println!("值: {:?}", value);
        println!("类型: {}", value.type_name());
        println!("字符串表示: {}", value.to_string());
        println!("是否为数值: {}", value.is_numeric());
        println!("是否为整数: {}", value.is_integer());
        println!("是否为浮点数: {}", value.is_float());
        println!("---");
    }

    // 演示 From trait 的使用
    println!("=== From trait 演示 ===");
    let from_i32: Primitive = 100.into();
    let from_bool: Primitive = false.into();
    let from_str: Primitive = "test string".into();

    println!("From i32: {:?}", from_i32);
    println!("From bool: {:?}", from_bool);
    println!("From &str: {:?}", from_str);
}
