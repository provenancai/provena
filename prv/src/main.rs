use std::sync::{Arc, Mutex};

use anyhow::{Context, Result};
use provena_api::build_router;
use provena_kernel::Kernel;
use provena_sdk::PluginManifest;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    let kernel = Arc::new(Mutex::new(Kernel::default()));

    // Discover and register plugins from the `plugins/` directory.
    // Each `*.toml` file in that directory is treated as a plugin manifest.
    // Startup continues even if no plugins directory exists.
    load_plugins_from_disk(&kernel)?;

    let router = build_router(Arc::clone(&kernel));
    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    println!("prv listening on http://127.0.0.1:3000");

    axum::serve(listener, router).await?;

    Ok(())
}

/// Scan `./plugins/` for `*.toml` files and register each as a plugin manifest.
///
/// Files that fail to parse are logged and skipped — a malformed manifest
/// should not prevent other plugins from loading. Missing directory is silently
/// ignored so the binary can start in a zero-plugin state for development.
fn load_plugins_from_disk(kernel: &Arc<Mutex<Kernel>>) -> Result<()> {
    let plugins_dir = std::path::Path::new("plugins");

    if !plugins_dir.exists() {
        return Ok(());
    }

    let entries = std::fs::read_dir(plugins_dir)
        .with_context(|| format!("failed to read plugins directory: {}", plugins_dir.display()))?;

    for entry in entries {
        let entry = entry.context("failed to read directory entry")?;
        let path = entry.path();

        if path.extension().and_then(|e| e.to_str()) != Some("toml") {
            continue;
        }

        let content = match std::fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("prv: skipping {}: {e}", path.display());
                continue;
            }
        };

        let manifest = match PluginManifest::from_toml(&content) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("prv: skipping {}: {e}", path.display());
                continue;
            }
        };

        let display_name = manifest.display_name.clone();

        let mut k = kernel.lock().map_err(|_| anyhow::anyhow!("kernel mutex poisoned"))?;
        match k.register_plugin(manifest) {
            Ok(()) => println!("prv: registered plugin '{display_name}' from {}", path.display()),
            Err(e) => eprintln!("prv: failed to register '{}': {e}", path.display()),
        }
    }

    Ok(())
}
