use log::*;

#[tokio::main]
async fn main() -> web3::Result<()> {
    pretty_env_logger::init();

    let transport = web3::transports::Http::new("http://localhost:8545")?;
    let web3 = web3::Web3::new(transport);

    info!("Calling accounts...");    
    let accounts = web3.eth().accounts().await?;
    info!("Accounts: {:?}", accounts);
    
    info!("Calling balances...");
    for account in accounts {
        let balance = web3.eth().balance(account, None).await?;
        info!("Balance of {:?}: {}", account, balance);
    }

    Ok(())
}
