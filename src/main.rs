use {anyhow::Result, std::env::var, warp::Filter};

#[tokio::main]
async fn main() -> Result<()> {
    simple_logger::init_with_level(log::Level::Info)?;

    let filter = warp::any()
        .and(warp::get().map(|| format!("You've hit {}", var("HOSTNAME").unwrap())))
        .or(warp::post()
            .and(warp::body::bytes())
            .map(|body: bytes::Bytes| {
                let body = String::from_utf8_lossy(&body);
                log::info!("Posted: {}", body);
                format!("You've posted:\n{}", body)
            }))
        .unify()
        .with(warp::log::log(env!("CARGO_PKG_NAME")));
    warp::serve(filter).run(([0, 0, 0, 0], 8080)).await;

    Ok(())
}
