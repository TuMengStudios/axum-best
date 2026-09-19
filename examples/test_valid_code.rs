use axum_best::utils::gen_valid_code;

fn main() {
    // Test generating verification codes of different lengths
    for len in [4, 6, 8] {
        let code = gen_valid_code(len);
        println!("生成长度为 {} 的验证码: {}", len, code);

        // Verify the length is correct
        assert_eq!(code.len(), len, "验证码长度不正确");

        // Verify it contains only digits
        assert!(code.chars().all(|c| c.is_ascii_digit()), "验证码包含非数字字符");

        println!("✅ 验证通过");
    }

    println!("所有测试通过！");
}
