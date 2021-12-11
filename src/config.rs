use std::fs;
use std::fs::read_to_string;
use std::io::Write;
use std::path::Path;

use config::{Config, File};
#[cfg(test)]
use mockall::{automock, predicate::*};
use openssl::pkey::{Private, Public};
use openssl::rsa::Rsa;
use openssl::sha::Sha256;
use serde::{Deserialize, Serialize};

use crate::errors::EncItError;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EncItPEM {
    Path(String),
    Base64(String),
    Hex(String),
    Pem(String),
}

impl EncItPEM {
    pub fn pem(&self) -> Result<Vec<u8>, EncItError> {
        match self {
            EncItPEM::Path(path) => {
                let cnt = read_to_string(path)?;
                Ok(cnt.into_bytes())
            }
            EncItPEM::Base64(b64) => base64::decode(b64).map_err(|e| e.into()),
            EncItPEM::Hex(hex_pem) => {
                let cleaned_hex = if hex_pem.starts_with("0x") {
                    hex_pem.clone().split_off(2)
                } else {
                    hex_pem.clone()
                };
                hex::decode(cleaned_hex.trim()).map_err(|e| e.into())
            }
            EncItPEM::Pem(pem) => Ok(pem.clone().into_bytes()),
        }
    }

    pub fn public_key(&self) -> Result<Rsa<Public>, EncItError> {
        Rsa::public_key_from_pem(self.pem()?.as_slice()).map_err(|e| e.into())
    }

    pub fn private_key(&self, passphrase: Option<&str>) -> Result<Rsa<Private>, EncItError> {
        if let Some(passphrase) = passphrase {
            Rsa::private_key_from_pem_passphrase(self.pem()?.as_slice(), passphrase.as_bytes())
        } else {
            Rsa::private_key_from_pem(self.pem()?.as_slice())
        }
        .map_err(|e| e.into())
    }

    pub fn hex_pem(&self) -> Result<String, EncItError> {
        Ok(hex::encode(self.pem()?))
    }

    pub fn sha_pem(&self) -> Result<String, EncItError> {
        let mut sha = Sha256::new();
        sha.update(self.pem()?.as_slice());
        Ok(hex::encode(sha.finish()))
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EncItPrivateKey {
    #[serde(flatten)]
    key: EncItPEM,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<String>,
}

#[allow(dead_code)]
impl EncItPrivateKey {
    pub fn new(key: EncItPEM, password: Option<String>) -> Self {
        EncItPrivateKey { key, password }
    }

    pub fn rsa_key(&self) -> Result<Rsa<Private>, EncItError> {
        let rsa_key = if let Some(password) = &self.password {
            Rsa::private_key_from_pem_passphrase(self.key.pem()?.as_slice(), password.as_bytes())?
        } else {
            Rsa::private_key_from_pem(self.key.pem()?.as_slice())?
        };
        Ok(rsa_key)
    }

    pub fn pem(&self) -> Result<Vec<u8>, EncItError> {
        self.rsa_key()?.private_key_to_pem().map_err(|e| e.into())
    }

    pub fn hex(&self) -> Result<String, EncItError> {
        self.rsa_key()?
            .private_key_to_pem()
            .map(hex::encode)
            .map_err(|e| e.into())
    }

    pub fn public_key_pem(&self) -> Result<Vec<u8>, EncItError> {
        self.rsa_key()?.public_key_to_pem().map_err(|e| e.into())
    }

    pub fn public_key_pem_hex(&self) -> Result<String, EncItError> {
        Ok(hex::encode(self.public_key_pem()?))
    }

    pub fn public_key_pem_sha(&self) -> Result<String, EncItError> {
        let mut sha = Sha256::new();
        sha.update(self.public_key_pem()?.as_slice());
        Ok(hex::encode(sha.finish()))
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EncItIdentity {
    name: String,
    #[serde(rename = "privateKey")]
    private_key: EncItPrivateKey,
}

#[allow(dead_code)]
impl EncItIdentity {
    pub fn new(name: String, private_key: EncItPrivateKey) -> Self {
        EncItIdentity { name, private_key }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn private_key(&self) -> &EncItPrivateKey {
        &self.private_key
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EncItFriend {
    name: String,
    #[serde(rename = "publicKey")]
    public_key: EncItPEM,
}

#[allow(dead_code)]
impl EncItFriend {
    pub fn new(name: String, public_key: EncItPEM) -> Self {
        EncItFriend { name, public_key }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn public_key(&self) -> &EncItPEM {
        &self.public_key
    }
}

#[cfg_attr(test, automock)]
pub trait EncItConfig {
    fn identity<'a>(&'a self, identity: &str) -> Option<&'a EncItIdentity>;
    fn identity_by_public_key_sha<'a>(
        &'a self,
        identity_public_key_sha: &str,
    ) -> Option<&'a EncItIdentity>;
    fn friend<'a>(&'a self, friend: &str) -> Option<&'a EncItFriend>;
    fn friend_by_public_key_sha<'a>(
        &'a self,
        identity_public_key_hex: &str,
    ) -> Option<&'a EncItFriend>;
    fn friends(&self) -> &Vec<EncItFriend>;
    fn add_friend(
        &self,
        friend_name: &str,
        public_key: &EncItPEM,
    ) -> Result<Box<dyn EncItConfig>, EncItError>;
    fn add_identity<'a>(
        &self,
        identity_name: &'a str,
        private_key: &EncItPEM,
        passphrase: Option<&'a str>,
    ) -> Result<Box<dyn EncItConfig>, EncItError>;
    fn identities(&self) -> &Vec<EncItIdentity>;
    fn save(&self) -> Result<(), EncItError>;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EncItConfigImpl {
    #[serde(skip)]
    path: String,
    #[serde(default)]
    identities: Vec<EncItIdentity>,
    #[serde(default)]
    friends: Vec<EncItFriend>,
}

impl EncItConfigImpl {
    pub fn create(config_file: &Path) -> Result<Self, EncItError> {
        let cfg = EncItConfigImpl {
            path: config_file.to_str().unwrap().to_string(),
            identities: vec![],
            friends: vec![],
        };
        cfg.save()?;
        Ok(cfg)
    }

    pub fn load(config_file: &Path) -> Result<Self, EncItError> {
        if !config_file.exists() {
            return Err(EncItError::ConfigurationNotFound(
                config_file.to_str().unwrap().to_string(),
            ));
        }
        let mut cfg = Config::default();
        cfg.merge(File::from(config_file))?;
        cfg.try_into::<EncItConfigImpl>()
            .map(|cfg| EncItConfigImpl {
                path: config_file.to_str().unwrap().to_string(),
                ..cfg
            })
            .map_err(|e| e.into())
    }
}

impl EncItConfig for EncItConfigImpl {
    fn identity(&self, identity: &str) -> Option<&EncItIdentity> {
        self.identities.iter().find(|i| i.name == identity)
    }

    fn identity_by_public_key_sha(&self, identity_public_key_sha: &str) -> Option<&EncItIdentity> {
        self.identities.iter().find(|identity| {
            let pub_key_sha = identity
                .private_key
                .public_key_pem_sha()
                .unwrap_or_else(|_| panic!("cannot get public key for identity {}", identity.name));
            println!("checking {} with {}", pub_key_sha, identity_public_key_sha);
            pub_key_sha == identity_public_key_sha
        })
    }

    fn friend(&self, friend: &str) -> Option<&EncItFriend> {
        self.friends.iter().find(|f| f.name == friend)
    }

    fn friend_by_public_key_sha(&self, identity_public_key_sha: &str) -> Option<&EncItFriend> {
        self.friends.iter().find(|friend| {
            let pub_key_sha = friend
                .public_key
                .sha_pem()
                .unwrap_or_else(|_| panic!("cannot get public key for identity {}", friend.name));
            pub_key_sha == identity_public_key_sha
        })
    }

    fn friends(&self) -> &Vec<EncItFriend> {
        &self.friends
    }

    fn add_friend<'a>(
        &self,
        friend_name: &str,
        public_key: &EncItPEM,
    ) -> Result<Box<dyn EncItConfig>, EncItError> {
        if self.friend(friend_name).is_some() {
            return Err(EncItError::FriendAlreadyExist());
        }
        let friend = EncItFriend {
            name: friend_name.to_string(),
            public_key: public_key.clone(),
        };
        let mut new_friends: Vec<EncItFriend> = Vec::with_capacity(self.friends.len() + 1);
        new_friends.extend(self.friends.to_vec());
        new_friends.extend(vec![friend]);

        Ok(Box::new(EncItConfigImpl {
            path: self.path.clone(),
            identities: self.identities.to_vec(),
            friends: new_friends,
        }))
    }

    fn add_identity<'a>(
        &self,
        identity_name: &'a str,
        private_key: &EncItPEM,
        passphrase: Option<&'a str>,
    ) -> Result<Box<dyn EncItConfig>, EncItError> {
        if self.identity(identity_name).is_some() {
            return Err(EncItError::IdentityAlreadyExist());
        }
        let identity = EncItIdentity {
            name: identity_name.to_string(),
            private_key: EncItPrivateKey {
                key: private_key.clone(),
                password: passphrase.map(|p| p.to_string()),
            },
        };
        let mut new_identities = Vec::with_capacity(self.identities.len() + 1);
        new_identities.extend(self.identities.to_vec());
        new_identities.extend(vec![identity]);

        Ok(Box::new(EncItConfigImpl {
            path: self.path.clone(),
            identities: new_identities,
            friends: self.friends.to_vec(),
        }))
    }

    fn identities(&self) -> &Vec<EncItIdentity> {
        &self.identities
    }

    fn save(&self) -> Result<(), EncItError> {
        let yaml = serde_yaml::to_string(self)?;
        let mut config_file = fs::File::create(&self.path)?;
        config_file.write_all(yaml.as_bytes())?;
        config_file.flush().map_err(|e| e.into())
    }
}

#[cfg(test)]
pub mod tests {
    use std::io::{Read, Seek, Write};

    use indoc::indoc;
    use openssl::symm::Cipher;
    use std::string::String;
    use tempfile::NamedTempFile;

    use super::*;

    pub static VALID_CFG_CNT: &str = indoc! {"\
        identities:
          - name: identity-1
            privateKey:
              base64: LS0tLS1CRUdJTiBFTkNSWVBURUQgUFJJVkFURSBLRVktLS0tLQpNSUlGSERCT0Jna3Foa2lHOXcwQkJRMHdRVEFwQmdrcWhraUc5dzBCQlF3d0hBUUk4ODJNZ1dZell3TUNBZ2dBCk1Bd0dDQ3FHU0liM0RRSUpCUUF3RkFZSUtvWklodmNOQXdjRUNOSDJONVl6MFpaZ0JJSUV5SzhVUkdFVGdtV1cKekZUU2lWeUdOZWZJZWMxWHY1Qjc3S1lXbFBadXZXU0dPZnhkUWljdzlDeGlmU0ltRGN6UzhOakhLU3NVNDBEaQozTjFBQnFVaFZSdytDWkJVYjFxQXB5WDNtNzFZZEZ3b2w4dFNZUXVmWGJvbE9FMXY5SzhLTS9Qbjh2RFVuaGdhCk5zZXIvZGZlbkhkOEZyRzRkNWZibEVkZFJHSWpnYVd3RGpVVm5BMHYyVHUxdkhoOCtMY3BmalhYNWtiZkFjUk8KK3JoSS9HV1pzcTAwbkEzbm1EUnA1TlN3YUVOYTA2WUI0VDU4VlpoalBXemxmNHdvVGppczYwcnpha0txbk1RZApPVTU2ejBINHZLbCt6bVIvcExUc0dnMUNCQXJIZDNKSFdZbjBJWTg5UjArZ3ZlRlJZMXVZNi91MjQ2SWJxSkpjCnQ1V2lMckRFMnkrbUsrWHM2WUQ5ZkdPeEJlSkFPUmgzelJaaURXYW05OS9TZUVkYnF1T1pzWFdWOUc0TGhham0KSWJ0YXF3U2lGd1R6L2dRU09GTFdLVTdmUTNwZ1p6WUJ1dytjRkswRUhXdzhvVndobzN2WjN4UVYwWDJZUSttNgpncHVXVHJGM3dGT3lXQ1ZEOFNEaHhtazBOL1F2RUcyTnpiV1NKaVlMYkRRVU14WHg3Z2NuMEpQeUE4dmxzRlpBCk15cGE3MmNxSjd1aVN4d2tOSHNScGh0MVB5amZ6UWRKeE9VMHNVU0wvWGRSQTU0ais4WTliUVo1cEpLVmdnRU8KcFFQNGdIMHBIMm5lMWpJcllSdURmMzFJVmkwcm9MVU1KSm9oa2xIaEpaWlA1c2ZkN1RIU25RbzF0ME5PbytiMApLbVlBc1lOb045ck1XOHJGZnJVVkhMMkgzZmNYczRKbjZsWW9jR2pyaENvRitxZklqakhjemhIT3RRSXZ1dWw2CjlkcmZhd0xUNGJKVDI0RUxybWFWVWhvcWUxVzAyaGxsQlBkQXZ3SGlva0RiR1M4ZHZuZWdhVUhyb29Nbzc5YmEKQ0tycXBIaGhGaWVNZGpEUXR3eDVlakJlMktWaSthZWRPUzRpUHExYm5nOGFiM3Jkck9NTDk5NzhkR216SzZsNApLUlpZanAvTDBRcHFpQ29YT3lORHdqYTd3RjhCU0d0ZlUzOGJWZDAyNVdqcElDVHVBYXdTcUpKZmxTNlI4Q21kCk5obWs3bU10K0VuZDZpa1FWMkZDYTNTamYwWHlpdC9uMzhqVTRMekVFaEpvNERqZW0wSGtxeHVkdFVBbkNsZncKODkvQjVSWnNTZG8vdkdIZCtpMEM3Z09CYkdDYXNwM0w0dmtrcnN4aDhjZ2RSeVdUQng3Y01lR3BXVzNxcE8xSwpCRkdHa0FDMGh5Z2lzdk5rZEQ4OVN5azZibzZBczBDZWR6RGJFUmpSWFhmTk1tUU5hOGpNTjNNVWw2MjVreGc0CmZxSVJJMTZrUXZFMUdGTnIwMFJBRW5TdHRUMGMxbjE3V0grZGFEUzk2R2ZwMTJzMGVuMTlLMDNxRnNIRGttVWgKS0J3U1pqaXNwanRwb2dZWXNJVThha2hNQWgyc3I5QjVuVXl1SzhCaERBM3h0eVJFaE8vSzRxN2FTYkp3Umh1bgp5SDh5YUVBR3Y5WlZKVFhPZlRKbXArMlJ6eXNNWWh3KzYyUW04M2l6TW8rNXQvdndYWkN1dk9LUEJJVUNXbFJ4Clp1cXZxeDhBaTZPK3FnV0o2VWxQYWxEb0F3K3NycldHQ2hpNTVuRTFHU2lXVDd0M0R2ZVBMWEVWdDVpL3lLNzgKb054eW5iNnZadk8wVG52azVoSWZDcjI0TDl1VEVCVU91ZmE0T052QlpMUTRWeDk3cURrT3BvOTIwcWM2dkJxcgpMS3NLUTZBa09OajNGM1JiMFhsTUhOYW94NnMvTFBaNWpyZDZpNi9LVFlvUDlQQWhLRVdZZnNMSXhwL2tyWFZWCndyK2lzTXZPTjllS01DczVRZVJ5MGF2aUlhcEE1akNETkgxWnV3MFZtdTJJSE9ZcjN4V0d0M2wxNVgyeGJtS3gKMkY4VXIrdll4UWxzQ21jK3hRQitpZTRMdEFnTHBsSUdRa0lQaVlQc1ZLVnVLdnhwZFVGa3p3bW9MTHFOdGNXawpLeWE4cGlCRkdkaHNZUU9lS24zZ3RnPT0KLS0tLS1FTkQgRU5DUllQVEVEIFBSSVZBVEUgS0VZLS0tLS0K
              password: test
        friends:
          - name: friend-1
            publicKey:
                base64: LS0tLS1CRUdJTiBQVUJMSUMgS0VZLS0tLS0KTUlJQklqQU5CZ2txaGtpRzl3MEJBUUVGQUFPQ0FROEFNSUlCQ2dLQ0FRRUEzcHFKZ2JJYXQzSUMvMkVsejcvMQpranhvbGZRaXlmSlZDY3N1Q0Y3cS9JQXRGRjhUbnhnbFJRNVF2MCtWUnRpMWR6MnNQeDRiN25xaFFVbytoRmd3CnVRclRvYWFndGNrc0RuUjdKblB5bE9WajJIOG85VW5aRmtvN2NsbDByVFZuSFI4Y3BsT1dkV1IvMzBYRm1uazYKck1hdUpIN2U3V0hhYW5DaFpSak01NitMT0d0MFNCWmR6SlVocHlmaFhXNDhhN0RpNFdlVEgzczNaSkh1djRjbwp1ZmdZU1pROHhsNklveHhmSmRETG1sR3p4Rkl1ZzBwYXhIK2VRNnJCY0p2UUROUlFCSm8vZDIxS05TcU1iT0plClU3SCtDZERmRXFVYlRZK3R1WVltVGxlVTBVK0RQOVN6ckd0Nmw1S1VqTGZyWXRBeTNlRzA2Ujg1MlZTWlpVdFAKMndJREFRQUIKLS0tLS1FTkQgUFVCTElDIEtFWS0tLS0tCg==
        "};

    #[test]
    fn load() -> Result<(), EncItError> {
        let (_, cfg) = get_valid_config()?;
        assert_eq!(cfg.identities().len(), 1);
        assert_eq!(cfg.friends().len(), 1);
        Ok(())
    }

    #[test]
    fn add_friend() -> Result<(), EncItError> {
        let (_, cfg) = get_valid_config()?;
        let new_friend_pub_key_hex = hex::encode(Rsa::generate(2048)?.public_key_to_pem()?);

        let new_cfg =
            cfg.add_friend("new-friend", &EncItPEM::Hex(new_friend_pub_key_hex.clone()))?;
        let new_friend = new_cfg.friend("new-friend");
        assert!(new_friend.is_some());
        assert_eq!(
            new_friend.unwrap().public_key.hex_pem()?,
            new_friend_pub_key_hex
        );
        assert!(new_cfg.friend("friend-1").is_some());
        Ok(())
    }

    #[test]
    fn add_identity() -> Result<(), EncItError> {
        let (_, cfg) = get_valid_config()?;
        let pem_pwd = "identity-pwd";
        let new_identity_private_key = hex::encode(
            Rsa::generate(2048)?
                .private_key_to_pem_passphrase(Cipher::aes_128_cbc(), pem_pwd.as_bytes())?,
        );
        let new_cfg = cfg.add_identity(
            "new-identity",
            &EncItPEM::Hex(new_identity_private_key.clone()),
            Some(pem_pwd),
        )?;
        let new_identity = new_cfg.identity("new-identity");
        assert!(new_identity.is_some());
        let new_identity = new_identity.unwrap();
        assert_eq!(
            new_identity.private_key.key.hex_pem()?,
            new_identity_private_key
        );
        assert_eq!(new_identity.private_key.password.as_ref().unwrap(), pem_pwd);
        assert!(new_cfg.identity("identity-1").is_some());
        Ok(())
    }

    #[test]
    fn save() -> Result<(), EncItError> {
        let (mut cfg_file, cfg) = get_valid_config()?;
        let new_friend_pub_key_hex = hex::encode(Rsa::generate(2048)?.public_key_to_pem()?);
        let identity_passphrase = "new-identity-pass";
        let new_identity_private_key_hex =
            hex::encode(Rsa::generate(2048)?.private_key_to_pem_passphrase(
                Cipher::aes_128_cbc(),
                identity_passphrase.as_bytes(),
            )?);
        cfg.add_friend("new-friend", &EncItPEM::Hex(new_friend_pub_key_hex))?
            .add_identity(
                "new-identity",
                &EncItPEM::Hex(new_identity_private_key_hex),
                Some(identity_passphrase),
            )?
            .save()?;

        cfg_file.rewind()?;
        let mut new_cfg_content = String::new();
        cfg_file.read_to_string(&mut new_cfg_content)?;
        println!("new cfg :{}", new_cfg_content);

        let new_cfg = EncItConfigImpl::load(cfg_file.path())?;
        assert!(new_cfg.friend("new-friend").is_some());
        assert!(new_cfg.identity("new-identity").is_some());
        Ok(())
    }

    #[test]
    fn identity_found() -> Result<(), EncItError> {
        let (_, cfg) = get_valid_config()?;
        cfg.identity("identity-1").expect("identity-1 not found");
        Ok(())
    }

    #[test]
    fn identity_not_found() -> Result<(), EncItError> {
        let (_, cfg) = get_valid_config()?;
        let non_existent_identity_opt = cfg.identity("non-existent-identity");
        assert!(non_existent_identity_opt.is_none());
        Ok(())
    }

    #[test]
    fn identity_by_public_key_sha_found() -> Result<(), EncItError> {
        let (_, cfg) = get_valid_config()?;
        cfg.identity_by_public_key_sha(
            "23b59f9973066dbfd3c69a714055cfd87391938c685a3062580343e2e3f2d6e0",
        )
        .expect("identity-1 not found");
        Ok(())
    }

    #[test]
    fn friend_found() -> Result<(), EncItError> {
        let (_, cfg) = get_valid_config()?;
        cfg.friend("friend-1").expect("friend-1 not found");
        Ok(())
    }

    #[test]
    fn friend_not_found() -> Result<(), EncItError> {
        let (_, cfg) = get_valid_config()?;
        let non_existent_friend_opt = cfg.friend("non-existent-friend");
        assert!(non_existent_friend_opt.is_none());
        Ok(())
    }

    #[test]
    fn friend_by_public_key_sha_found() -> Result<(), EncItError> {
        let (_, cfg) = get_valid_config()?;
        cfg.friend_by_public_key_sha(
            "dbb90347dcf9f816ef522e94e69bf2964de87f966175e7e15d27c34ae0e9fbbc",
        )
        .expect("friend-1 not found");
        Ok(())
    }

    fn get_valid_config() -> Result<(NamedTempFile, EncItConfigImpl), EncItError> {
        let mut cfg_file = tempfile::Builder::new().suffix(".yml").tempfile().unwrap();
        write!(cfg_file, "{}", VALID_CFG_CNT).expect("error writing cfg file");
        let cfg = EncItConfigImpl::load(cfg_file.path())?;
        Ok((cfg_file, cfg))
    }
}
