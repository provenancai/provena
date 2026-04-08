# Plugin Model

Plugins are out-of-process. Each plugin is a separate binary or container that communicates with the kernel over HTTP.

## Manifests

Plugins declare their capabilities via a `plugin.toml` manifest on disk. This is the authoritative source of truth. The kernel discovers plugins by reading these manifests - there is no hardcoded configuration.

`plugin.toml` is deserialised at runtime into `PluginManifest`. The on-disk format and the in-memory type are the same data at different layers of the system.

## Plugin SDK

Third-party plugin authors use the `provena-sdk` crate. It defines:

- `Plugin` - the trait a plugin binary implements
- `PluginManifest` - the capabilities a plugin declares
- `CapabilityDescriptor` - a single capability and its metadata
- Provider traits for specific capability categories

No dependency on `provena-kernel` is required to build a plugin.

## Lifecycle

1. Plugin binary starts and writes its `plugin.toml` to the agreed discovery path.
2. The kernel reads the manifest and registers the plugin's capabilities.
3. The kernel polls the plugin's health endpoint to track liveness.
4. Routing requests for a capability are forwarded to the plugin's HTTP endpoint.
5. If a plugin becomes unhealthy, it is removed from the routing table until it recovers.
