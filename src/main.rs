use {anyhow::Result, std::env::var, warp::Filter};

#[tokio::main(core_threads = 8)]
async fn main() -> Result<()> {
    simple_logger::init_with_level(log::Level::Info)?;

    let filter = warp::get()
        .map(|| format!("You've hit {}", var("HOSTNAME").unwrap()))
        .with(warp::log::log(env!("CARGO_PKG_NAME")));
    warp::serve(filter).run(([0, 0, 0, 0], 8080)).await;

    Ok(())
}
