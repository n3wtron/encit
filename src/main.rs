use crate::cmd::root_cmd::{root_cmd, root_exec, CommandsImpl};
use crate::config::{EncItConfig, EncItConfigImpl, EncItPEM};
use crate::errors::EncItError;
use std::fs::create_dir;
use std::path::Path;
use std::rc::Rc;

mod cmd;
mod config;
mod enc;
mod errors;

fn main() -> Result<(), EncItError> {
    env_logger::init();

    let config_file = dirs::home_dir()
        .expect("cannot find home directory")
        .join(".encit")
        .join("config.yml");

    let config: Rc<dyn EncItConfig> = get_config(config_file.as_path())?;
    let commands = Rc::new(CommandsImpl::new(config));

    let matches = root_cmd().get_matches();
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
    use super::*;
    use crate::config::tests::VALID_CFG_CNT;
    use std::io::Write;

    #[test]
    fn get_config_not_exist() -> Result<(), EncItError> {
        let cfg_file = tempfile::Builder::new().suffix(".yml").tempfile()?;
        let cfg_file_path = cfg_file.path();
        get_config(cfg_file_path)?;
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
