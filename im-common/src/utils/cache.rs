#[inline]
pub fn token2key(token: &str) -> String {
    format!("user:{}", token)
}