use crate::config::{Config, ConfigItem};

use std::{
    fs, io,
    path::{Path, PathBuf},
};

#[derive(Debug, PartialEq, Eq)]
pub enum WriteStatus {
    Created,
    Updated,
    Skipped,
}

pub fn get_target_path(config: &Config, item: &ConfigItem) -> PathBuf {
    let file_name = format!("{}.{}", item.name, config.target_extension);
    Path::new(&config.path_to_target).join(file_name)
}

pub fn write_if_changed(path: &Path, new_content: &str) -> io::Result<WriteStatus> {
    match fs::read_to_string(path) {
        Ok(existing_content) => {
            if existing_content == new_content {
                Ok(WriteStatus::Skipped)
            } else {
                fs::write(path, new_content)?;
                Ok(WriteStatus::Updated)
            }
        }
        Err(err) if err.kind() == io::ErrorKind::NotFound => {
            fs::write(path, new_content)?;
            Ok(WriteStatus::Created)
        }
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use super::{WriteStatus, get_target_path, write_if_changed};
    use crate::config::{Config, ConfigItem, ValuesMap};
    use std::{
        fs,
        path::PathBuf,
        time::{SystemTime, UNIX_EPOCH},
    };

    fn make_config(path_to_target: &str, target_extension: &str) -> Config {
        Config {
            path_to_target: path_to_target.to_string(),
            path_to_template: String::new(),
            template: String::new(),
            target_extension: target_extension.to_string(),
            values: ValuesMap::new(),
            configs: Vec::new(),
        }
    }

    fn make_item(name: &str) -> ConfigItem {
        ConfigItem {
            name: name.to_string(),
            template: None,
            values: ValuesMap::new(),
        }
    }

    fn temp_file_path(test_name: &str) -> PathBuf {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        std::env::temp_dir().join(format!("genconf-{test_name}-{timestamp}.conf"))
    }

    #[test]
    fn builds_target_path_from_name_and_extension() {
        let config = make_config("/tmp/genconf", "conf");
        let item = make_item("worker-default");

        let path = get_target_path(&config, &item);

        assert_eq!(path, PathBuf::from("/tmp/genconf/worker-default.conf"));
    }

    #[test]
    fn returns_created_when_file_does_not_exist() {
        let path = temp_file_path("created");

        let status = write_if_changed(&path, "first content").unwrap();
        let written = fs::read_to_string(&path).unwrap();

        assert_eq!(status, WriteStatus::Created);
        assert_eq!(written, "first content");

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn returns_skipped_for_same_content_and_updated_for_new_content() {
        let path = temp_file_path("updated");
        fs::write(&path, "initial content").unwrap();

        let skipped = write_if_changed(&path, "initial content").unwrap();
        let updated = write_if_changed(&path, "new content").unwrap();
        let written = fs::read_to_string(&path).unwrap();

        assert_eq!(skipped, WriteStatus::Skipped);
        assert_eq!(updated, WriteStatus::Updated);
        assert_eq!(written, "new content");

        fs::remove_file(path).unwrap();
    }
}
