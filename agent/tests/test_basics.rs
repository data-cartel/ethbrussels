use serde_json::json;
use nearb_agent::*;

#[tokio::test]
async fn test_contract_is_operational() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;

    const BLOCK_HEIGHT = 123306141;

    let testnet = near_workspaces::testnet_archival();
    let ref_fi_id: AccountId = REF_FI_ACCOUNT_ID.parse()?;
    let ref_fi = worker
        .import_contract(&ref_fi_id, &testnet)
        .initial_balance(parse_near!("1000 N"))
        .block_height(BLOCK_HEIGHT)
        .transact()
        .await?;

    // let ref_fi = std::fs::read("./ref_exchange_release.wasm")?;
    // let ref_fi = sandbox.dev_deploy(&ref_fi).await?;

    let owner = sandbox.dev_create_account().await?;

    owner
        .call(&worker, contract.id(), "init")
        .args_json(serde_json::json!({
            "ref_fi_id": value1,
            "arg2": value2,
        }))?
        .transact()
        .await?;

    let contract_wasm = near_workspaces::compile_project("./").await?;
    let contract = sandbox.dev_deploy(&contract_wasm).await?;

    Ok(())
}
