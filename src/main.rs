mod config;

use clap::Parser;
use config::Config;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::config::{ConfigItem, ValuesMap, value_as_string};

#[derive(Parser, Debug)]
#[command(version, about = "YAML config reader")]
struct Cli {
    #[arg(short, long, default_value = "config.yaml")]
    config: PathBuf,
}

#[derive(Debug, PartialEq, Eq)]
enum WriteStatus {
    Created,
    Updated,
    Skipped,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let content = fs::read_to_string(&cli.config)?;
    let config: Config = serde_yaml_ng::from_str(&content)?;

    for item in &config.configs {
        let render = render_config_item(&config, item)?;
        let target_path = get_target_tath(&config, item);

        match write_if_changed(&target_path, &render)? {
            WriteStatus::Created => {
                println!("created: {}", target_path.display());
            }
            WriteStatus::Updated => {
                println!("updated: {}", target_path.display());
            }
            WriteStatus::Skipped => {
                println!("skip: no changes {}", target_path.display());
            }
        }
    }

    Ok(())
}

fn render_config_item(
    config: &Config,
    item: &ConfigItem,
) -> Result<String, Box<dyn std::error::Error>> {
    let template_name = item.template.as_deref().unwrap_or(&config.template);

    let template_path = Path::new(&config.path_to_template).join(template_name);
    let template_content = fs::read_to_string(&template_path)?;

    let values = get_values(&config.values, item);
    let render = apply_values(&template_content, &values);

    Ok(render)
}

fn get_values(default_values: &ValuesMap, item: &ConfigItem) -> ValuesMap {
    let mut values = default_values.clone();

    values.insert(
        "name".to_string(),
        serde_yaml_ng::Value::String(item.name.clone()),
    );

    for (key, val) in &item.values {
        values.insert(key.clone(), val.clone());
    }

    values
}

fn apply_values(template_content: &str, values: &ValuesMap) -> String {
    let regexp =
        regex::Regex::new(r"\{\{\s*([a-zA-Z0-9_-]+)(?:\s*\|\s*([a-zA-Z0-9_-]+))?\s*\}\}").unwrap();

    regexp
        .replace_all(template_content, |caps: &regex::Captures| {
            let key = &caps[1];
            let filter = caps.get(2).map(|m| m.as_str());

            match values.get(key) {
                Some(v) => {
                    apply_filter(value_as_string(v), filter).unwrap_or_else(|| caps[0].to_string())
                }
                None => caps[0].to_string(),
            }
        })
        .to_string()
}

fn apply_filter(value: String, filter: Option<&str>) -> Option<String> {
    match filter {
        None => Some(value),
        Some("lower") => Some(value.to_lowercase()),
        Some(_) => None,
    }
}

fn get_target_tath(config: &Config, item: &ConfigItem) -> PathBuf {
    let file_name = format!("{}.{}", item.name, config.target_extension);
    Path::new(&config.path_to_target).join(file_name)
}

fn write_if_changed(path: &Path, new_content: &str) -> io::Result<WriteStatus> {
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
