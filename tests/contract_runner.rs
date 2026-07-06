use std::fs;
use std::path::{Path, PathBuf};

use bpi_rs::probe::audit::audit_contracts;
use bpi_rs::probe::endpoint_contract::{ApiRisk, EndpointContract};
use bpi_rs::probe::flow::ProbeFlow;
use bpi_rs::probe::model::parse_registered_model;
use bpi_rs::probe::sanitize::audit_value;
use bpi_rs::{BpiError, BpiResult};

#[test]
fn contract_runner_validates_committed_contracts() -> Result<(), Box<dyn std::error::Error>> {
    let root = Path::new("tests/contracts");
    let report = audit_contracts(root)?;
    assert_eq!(
        report.errors, 0,
        "契约结构审计发现 {} 个错误: {:?}",
        report.errors, report.findings
    );

    let mut checked_fixtures = 0usize;
    let mut parsed_models = 0usize;
    for path in contract_files(root)? {
        let bytes = fs::read(&path)?;
        let value: serde_json::Value = serde_json::from_slice(&bytes)?;
        if value.get("steps").is_some() {
            ProbeFlow::from_slice(&bytes)?;
            continue;
        }

        let contract = EndpointContract::from_slice(&bytes)?;
        for case in &contract.cases {
            let Some(fixture) = &case.response.fixture else {
                continue;
            };
            let fixture_path = path.parent().unwrap_or_else(|| Path::new("")).join(fixture);
            let fixture_bytes = fs::read(&fixture_path)?;
            let fixture_value: serde_json::Value = serde_json::from_slice(&fixture_bytes)?;

            assert_fixture_api_code_matches(&fixture_path, &fixture_value, case.response.api_code);

            let findings = audit_value(&fixture_value, Some(ApiRisk::PublicRead));
            assert!(
                findings.is_empty(),
                "fixture 脱敏审计失败: {} {:?}",
                fixture_path.display(),
                findings
            );

            if let Some(model) = case.response.rust_model.as_deref()
                && parse_registered_model(model, &fixture_bytes)?
            {
                parsed_models += 1;
            }

            checked_fixtures += 1;
        }
    }

    assert!(checked_fixtures > 0, "没有检查到任何 fixture");
    assert!(parsed_models > 0, "没有解析到任何已注册模型");
    Ok(())
}

fn assert_fixture_api_code_matches(path: &Path, value: &serde_json::Value, expected: Option<i32>) {
    let Some(expected) = expected else {
        return;
    };
    let actual = value
        .get("code")
        .or_else(|| value.get("errno"))
        .and_then(serde_json::Value::as_i64);

    if let Some(actual) = actual {
        assert_eq!(
            actual,
            expected as i64,
            "fixture code 和契约 api_code 不一致: {}",
            path.display()
        );
    }
}

fn contract_files(root: &Path) -> BpiResult<Vec<PathBuf>> {
    let mut files = Vec::new();
    collect_contract_files(root, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_contract_files(path: &Path, files: &mut Vec<PathBuf>) -> BpiResult<()> {
    if path.is_file() {
        if path.file_name().and_then(|value| value.to_str()) == Some("contract.json") {
            files.push(path.to_path_buf());
        }
        return Ok(());
    }

    if !path.is_dir() {
        return Err(BpiError::invalid_parameter(
            "path",
            "contract runner path must be a contract.json file or directory",
        ));
    }

    for entry in fs::read_dir(path).map_err(|err| BpiError::parse(err.to_string()))? {
        let entry = entry.map_err(|err| BpiError::parse(err.to_string()))?;
        collect_contract_files(&entry.path(), files)?;
    }
    Ok(())
}
