use anyhow::Result;
use provena_api::build_router;
use provena_core::{CapabilityName, PluginId};
use provena_kernel::Kernel;
use provena_sdk::{CapabilityDescriptor, PluginManifest};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    let mut kernel = Kernel::default();

    let manifest = PluginManifest::new(
        PluginId::new(),
        "reference-ledger",
        vec![CapabilityDescriptor::new(
            CapabilityName::new("ledger.append")?,
            0,
            true,
        )],
    );

    kernel.register_plugin(manifest)?;

    let router = build_router(kernel.health());
    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    println!("prv listening on http://127.0.0.1:3000");

    axum::serve(listener, router).await?;

    Ok(())
}