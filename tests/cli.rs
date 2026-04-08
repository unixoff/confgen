use std::{
    fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

fn temp_dir_path(test_name: &str) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    std::env::temp_dir().join(format!("genconf-{test_name}-{timestamp}"))
}

#[test]
fn generates_config_file_from_yaml_template() {
    let root = temp_dir_path("integration");
    let template_dir = root.join("template");
    let target_dir = root.join("out");
    let config_path = root.join("values.yaml");
    let output_path = target_dir.join("worker-default.conf");

    fs::create_dir_all(&template_dir).unwrap();
    fs::create_dir_all(&target_dir).unwrap();

    fs::write(
        template_dir.join("worker.conf"),
        "\
[program:{{ name }}]
command={{ php_bin }} {{ console }} {{ command_args }}
autostart={{ autostart | lower }}
user={{ user }}
startsecs={{ startsecs }}
",
    )
    .unwrap();

    fs::write(
        &config_path,
        format!(
            "\
pathToTarget: {}
pathToTemplate: {}
template: worker.conf
targetExtension: conf

values:
  php_bin: /usr/bin/php
  console: /srv/app/bin/console
  user: app
  autostart: true
  startsecs: 0

configs:
  - name: worker-default
    values:
      command_args: queue:consume default
      startsecs: 5
",
            target_dir.display(),
            template_dir.display()
        ),
    )
    .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_genconf"))
        .arg("--config")
        .arg(&config_path)
        .output()
        .unwrap();

    assert!(output.status.success());
    assert_eq!(
        fs::read_to_string(&output_path).unwrap(),
        "\
[program:worker-default]
command=/usr/bin/php /srv/app/bin/console queue:consume default
autostart=true
user=app
startsecs=5
"
    );

    fs::remove_dir_all(root).unwrap();
}
