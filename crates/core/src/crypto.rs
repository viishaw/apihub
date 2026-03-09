//! 加密模块

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use sha2::{Digest, Sha256};

/// 加密器
pub struct Encryptor {
    cipher: Aes256Gcm,
}

impl Encryptor {
    /// 从主密钥创建加密器
    pub fn new(master_key: &str) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(master_key.as_bytes());
        let key = hasher.finalize();
        
        let cipher = Aes256Gcm::new_from_slice(&key)
            .expect("Invalid key length");
        
        Self { cipher }
    }
    
    /// 加密 API Key
    pub fn encrypt(&self, plaintext: &str) -> crate::error::Result<Vec<u8>> {
        // 生成随机 nonce（12 字节）
        let nonce_bytes: [u8; 12] = {
            use rand::RngCore;
            let mut rng = rand::thread_rng();
            let mut bytes = [0u8; 12];
            rng.fill_bytes(&mut bytes);
            bytes
        };
        
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = self.cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| crate::error::Error::Internal(anyhow::anyhow!("Encryption failed: {}", e)))?;
        
        // 将 nonce 和 ciphertext 组合
        let mut result = nonce_bytes.to_vec();
        result.extend(ciphertext);
        Ok(result)
    }
    
    /// 解密 API Key
    pub fn decrypt(&self, encrypted: &[u8]) -> crate::error::Result<String> {
        if encrypted.len() < 12 {
            return Err(crate::error::Error::Internal(anyhow::anyhow!("Invalid encrypted data")));
        }
        
        let (nonce_bytes, ciphertext) = encrypted.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        
        let plaintext = self.cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| crate::error::Error::Internal(anyhow::anyhow!("Decryption failed: {}", e)))?;
        
        String::from_utf8(plaintext)
            .map_err(|e| crate::error::Error::Internal(anyhow::anyhow!("Invalid UTF-8: {}", e)))
    }
    
    /// 计算 Key 的哈希（用于去重）
    pub fn hash_key(api_key: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(api_key.as_bytes());
        let hash = hasher.finalize();
        hex::encode(hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_encrypt_decrypt() {
        let encryptor = Encryptor::new("test-master-key-32-chars-long!!");
        let original = "sk-test123456";
        
        let encrypted = encryptor.encrypt(original).unwrap();
        let decrypted = encryptor.decrypt(&encrypted).unwrap();
        
        assert_eq!(original, decrypted);
    }
    
    #[test]
    fn test_hash_key() {
        let key1 = "sk-test123";
        let key2 = "sk-test123";
        let key3 = "sk-test456";
        
        assert_eq!(Encryptor::hash_key(key1), Encryptor::hash_key(key2));
        assert_ne!(Encryptor::hash_key(key1), Encryptor::hash_key(key3));
    }
}
