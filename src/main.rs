
use pumpfun_pumpswap_copytrader::common::{config::Config,config::notify_token_swap, constants::RUN_MSG};
use pumpfun_pumpswap_copytrader::engine::monitor::pumpswap_trader;
use std::env;
use dotenv::dotenv;


#[tokio::main]
async fn main() {
    dotenv().ok();
    notify_token_swap();
    let config = Config::new().await;
    let config = config.lock().await;
       

    /* Running Bot */
    let run_msg = RUN_MSG;
    println!("{}", run_msg);

    let _ = pumpswap_trader(
        config.yellowstone_grpc_http.clone(),
        config.yellowstone_grpc_token.clone(),
        config.app_state.clone(),
        config.swap_config.clone(),
        config.targetlist.clone(),
    )
    .await;


}
