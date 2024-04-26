use md5::Digest;

pub fn md5(s: &str) -> String {
    let mut hasher = md5::Md5::new();
    hasher.update(s.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn md5_string(s: String) -> String {
    md5(&s)
}
