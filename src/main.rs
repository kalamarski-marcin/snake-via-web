use genesismobo::snake::http_server;
use genesismobo::snake::movement_scheduler;
use tide::Result;

const SERVER_ADDR: &str = "127.0.0.1:8080";

#[async_std::main]
async fn main() -> Result<()> {
    let app = http_server::init();
    movement_scheduler::run(&app);

    app.listen(SERVER_ADDR).await?;
    Ok(())
}
