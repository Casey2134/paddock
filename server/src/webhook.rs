use sha2::Sha256;
use hmac::{
    Hmac,
    KeyInit,
    Mac,
};


pub fn verify_webhook(secret: &[u8], body: &[u8], s_header: &str) -> bool {
    let Some(rest) = s_header.strip_prefix("sha256=") else {
        return false;
    };
    let Ok(expected) = hex::decode(rest) else {
        return false;
    };
    let  Ok(mut mac) = Hmac::<Sha256>::new_from_slice(secret) else {
        return false;
    };
    mac.update(body);
    return mac.verify_slice(&expected).is_ok();
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn valid_github_signature_passes() {
        dotenvy::dotenv();
        let body = std::fs::read("tests/fixtures/b0f01e10-b7c6-11f1-8015-5bb7cda013f7-workflow_job.json").expect("file fixture missing");
        let secret = std::env::var("WEBHOOK_SECRET").expect("set WEBHOOK_SECRET");
        let signature = "sha256=69c9a778db104ee3d6d2c09432818a45d4e835ad989ab26cf30042c3a8152165";
        assert!(verify_webhook(secret.as_bytes(), &body, signature));
    }
}
