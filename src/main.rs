use crate::cmd::root_cmd::{root_cmd, root_exec, CommandsImpl};
use crate::config::{EncItConfig, EncItConfigImpl, EncItPEM};
use crate::errors::EncItError;
use log::debug;
use std::env;
use std::fs::create_dir;
use std::path::{Path, PathBuf};
use std::rc::Rc;

mod cmd;
mod config;
mod enc;
mod errors;

fn main() -> Result<(), EncItError> {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    debug!("args {:?}", args);

    let matches = root_cmd().get_matches();

    let config_file = if let Some(config_file_arg) = matches.value_of("config") {
        PathBuf::new().join(config_file_arg)
    } else {
        dirs::home_dir()
            .expect("cannot find home directory")
            .join(".encit")
            .join("config.yml")
    };
    let config: Rc<dyn EncItConfig> = get_config(config_file.as_path())?;
    let commands = Rc::new(CommandsImpl::new(config));

    root_exec(commands, &matches)
}

fn get_config(config_file: &Path) -> Result<Rc<dyn EncItConfig>, EncItError> {
    let config = if !&config_file.exists() {
        let config_dir = config_file.parent().unwrap();
        if !config_dir.exists() {
            create_dir(config_dir)?;
        }
        EncItConfigImpl::create(config_file)?
    } else {
        EncItConfigImpl::load(config_file)?
    };
    Ok(Rc::new(config))
}

#[cfg(test)]
mod tests {

    struct AutoDeleteFile<'a>(pub &'a Path);
    impl<'a> Drop for AutoDeleteFile<'a> {
        fn drop(&mut self) {
            let _ = remove_file(self.0);
        }
    }

    use super::*;
    use crate::config::tests::VALID_CFG_CNT;
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    use std::env::temp_dir;
    use std::fs::remove_file;
    use std::io::Write;
    use std::path::PathBuf;

    #[test]
    fn get_config_not_exist() -> Result<(), EncItError> {
        let cfg_file_name: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();
        let mut cfg_file_path = PathBuf::new();
        cfg_file_path.push(temp_dir());
        cfg_file_path.push(cfg_file_name);
        let _auto_delete = AutoDeleteFile(cfg_file_path.as_path());
        get_config(cfg_file_path.as_path())?;
        assert!(cfg_file_path.exists());
        Ok(())
    }

    #[test]
    fn get_config_from_existent_file() -> Result<(), EncItError> {
        let mut cfg_file = tempfile::Builder::new().suffix(".yml").tempfile()?;
        cfg_file.write_all(VALID_CFG_CNT.as_bytes())?;
        let cfg_file_path = cfg_file.path();
        let cfg = get_config(cfg_file_path)?;
        assert_eq!(cfg.identities().len(), 1);
        Ok(())
    }
}
