use serde::Deserialize;
use std::collections::HashMap;

pub type ValuesMap = HashMap<String, serde_yaml_ng::Value>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub path_to_target: String,
    pub path_to_template: String,
    pub template: String,
    pub target_extension: String,
    pub values: ValuesMap,
    pub configs: Vec<ConfigItem>,
}

#[derive(Debug, Deserialize)]
pub struct ConfigItem {
    pub name: String,

    #[serde(default)]
    pub template: Option<String>,

    #[serde(default)]
    pub values: ValuesMap,
}

pub fn value_as_string(value: &serde_yaml_ng::Value) -> String {
    match value {
        serde_yaml_ng::Value::String(v) => v.clone(),
        serde_yaml_ng::Value::Bool(v) => v.to_string(),
        serde_yaml_ng::Value::Number(v) => v.to_string(),
        serde_yaml_ng::Value::Null => String::new(),
        other => serde_yaml_ng::to_string(other)
            .unwrap_or_default()
            .trim()
            .to_string(),
    }
}
