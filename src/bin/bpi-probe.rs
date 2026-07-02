use std::env;
use std::fs;

use bpi_rs::BpiError;
use bpi_rs::probe::account::RawProbeConfig;
use bpi_rs::probe::contract::ApiContract;
use bpi_rs::probe::flow::{ProbeFlow, execute_flow};
use bpi_rs::probe::run::execute_contract;

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), BpiError> {
    if env::var("BPI_PROBE").as_deref() != Ok("1") {
        return Err(BpiError::invalid_parameter(
            "BPI_PROBE",
            "set BPI_PROBE=1 to run network probes",
        ));
    }

    let mut args = env::args().skip(1);
    let contract_path = args.next().ok_or_else(|| {
        BpiError::invalid_parameter(
            "contract",
            "usage: bpi-probe <contract.json> [account.toml]",
        )
    })?;
    let account_path = args.next().unwrap_or_else(|| "account.toml".to_string());
    let output_path = args.next();

    if args.next().is_some() {
        return Err(BpiError::invalid_parameter(
            "args",
            "usage: bpi-probe <contract.json> [account.toml] [output.json]",
        ));
    }

    let contract_bytes = fs::read(&contract_path).map_err(|err| {
        BpiError::parse(format!(
            "failed to read contract file {contract_path}: {err}"
        ))
    })?;
    let accounts = RawProbeConfig::load(&account_path)?;
    let input: serde_json::Value = serde_json::from_slice(&contract_bytes)?;
    let output = if input.get("steps").is_some() {
        let flow = ProbeFlow::from_slice(&contract_bytes)?;
        let result = execute_flow(&flow, &accounts).await?;
        serde_json::to_string_pretty(&result)?
    } else {
        let contract = ApiContract::from_value(input)?;
        let result = execute_contract(&contract, &accounts).await?;
        serde_json::to_string_pretty(&result)?
    };

    if let Some(output_path) = output_path {
        fs::write(&output_path, format!("{output}\n")).map_err(|err| {
            BpiError::parse(format!(
                "failed to write probe output file {output_path}: {err}"
            ))
        })?;
    } else {
        println!("{output}");
    }
    Ok(())
}
