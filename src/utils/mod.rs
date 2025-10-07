use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

use anyhow::Result;
use digest::Digest;
use md5::Md5;
use rand::random;
use sha1::Sha1;
use sha2::Sha256;
use sha2::Sha512;

/// 生成指定长度的数字验证码
/// Generates a numeric verification code of specified length
///
/// # Arguments
/// * `len` - 验证码长度 / Code length
///
/// # Returns
/// * `String` - 生成的验证码字符串 / Generated verification code string
pub fn gen_valid_code(len: usize) -> String {
    let mut code = String::with_capacity(len);

    for i in 0..len {
        // 生成随机数字 (0-9)
        // Generate random digit (0-9)
        let mut digit = (random::<u8>() % 10).to_string();

        // 如果第一位是0，重新生成以避免以0开头
        // If first digit is 0, regenerate to avoid starting with 0
        if i == 0 && digit.contains("0") {
            digit = (random::<u8>() % 10).to_string()
        }

        // 将数字添加到验证码字符串
        // Add digit to verification code string
        code.push_str(&digit);
    }

    code
}

#[derive(Debug, Clone, Copy)]
pub enum HashAlgorithm {
    MD5,
    SHA1,
    SHA256,
    SHA512,
}

pub enum Hasher {
    MD5(Md5),
    SHA1(Sha1),
    SHA256(Sha256),
    SHA512(Sha512),
}

impl Hasher {
    pub fn update(&mut self, data: &[u8]) {
        match self {
            Hasher::MD5(hasher) => hasher.update(data),
            Hasher::SHA1(hasher) => hasher.update(data),
            Hasher::SHA256(hasher) => hasher.update(data),
            Hasher::SHA512(hasher) => hasher.update(data),
        }
    }

    pub fn finalize(self) -> String {
        match self {
            Hasher::MD5(hasher) => format!("{:x}", hasher.finalize()),
            Hasher::SHA1(hasher) => format!("{:x}", hasher.finalize()),
            Hasher::SHA256(hasher) => format!("{:x}", hasher.finalize()),
            Hasher::SHA512(hasher) => format!("{:x}", hasher.finalize()),
        }
    }
}

impl HashAlgorithm {
    pub fn hasher(&self) -> Hasher {
        match self {
            HashAlgorithm::MD5 => Hasher::MD5(Md5::new()),
            HashAlgorithm::SHA1 => Hasher::SHA1(Sha1::new()),
            HashAlgorithm::SHA256 => Hasher::SHA256(Sha256::new()),
            HashAlgorithm::SHA512 => Hasher::SHA512(Sha512::new()),
        }
    }
}

/// 计算文件的哈希摘要
pub fn file_digest(path: &Path, algorithm: HashAlgorithm) -> Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut buffer = [0; 8192]; // 8KB 缓冲区

    match algorithm {
        HashAlgorithm::MD5 => {
            let mut hasher = Md5::new();
            copy_to_hasher(&mut reader, &mut hasher, &mut buffer)?;
            Ok(format!("{:x}", hasher.finalize()))
        }
        HashAlgorithm::SHA1 => {
            let mut hasher = Sha1::new();
            copy_to_hasher(&mut reader, &mut hasher, &mut buffer)?;
            Ok(format!("{:x}", hasher.finalize()))
        }
        HashAlgorithm::SHA256 => {
            let mut hasher = Sha256::new();
            copy_to_hasher(&mut reader, &mut hasher, &mut buffer)?;
            Ok(format!("{:x}", hasher.finalize()))
        }
        HashAlgorithm::SHA512 => {
            let mut hasher = Sha512::new();
            copy_to_hasher(&mut reader, &mut hasher, &mut buffer)?;
            Ok(format!("{:x}", hasher.finalize()))
        }
    }
}

/// 通用的哈希计算辅助函数
fn copy_to_hasher<R, H>(reader: &mut R, hasher: &mut H, buffer: &mut [u8]) -> Result<()>
where
    R: Read,
    H: Digest,
{
    loop {
        let bytes_read = reader.read(buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    Ok(())
}

#[tokio::test]
async fn test_gen_valid_code() {
    let valid_code = gen_valid_code(4);
    assert_eq!(valid_code.len(), 4);
    println!("{valid_code:?}");
}

#[tokio::test]
async fn test_file_digest() {
    use manifest_dir_macros::file_path;
    let path = Path::new(file_path!("tests/index.txt"));
    let digest = file_digest(&path, HashAlgorithm::SHA1).unwrap();
    println!("sha1:{}", digest);
    assert_eq!(digest, "65aed31e2af181f131e7091301b97d0b8379bdf3");
    let digest = file_digest(&path, HashAlgorithm::SHA512).unwrap();
    println!("sha256:{}", digest);
    assert_eq!(
        digest,
        "d074479576ba7b335c9178bfdd85a9f0f32084faa823b83922d638c3fe3dcca7ac836783fbf6f14cf253c915b46ce7d9974814c28063cad74e987131652dcf7f"
    );
    let digest = file_digest(&path, HashAlgorithm::MD5).unwrap();
    println!("md5:{}", digest);
    assert_eq!(digest, "deab55458837fbe4ec2a6e61690fe998");
}

#[tokio::test]
async fn test_hasher() {
    let algorithm = HashAlgorithm::SHA256;
    let mut hasher = algorithm.hasher();
    hasher.update(b"hello world");
    let result = hasher.finalize();
    println!("hasher result: {}", result);
    assert_eq!(result, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
}
