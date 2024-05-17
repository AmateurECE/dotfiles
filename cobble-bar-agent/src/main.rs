use sink::PrintSink;
use widgets::clock::run_clock;

mod sink;
mod widgets;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mut sink = PrintSink;
    run_clock(&mut sink).await?;
    Ok(())
}
