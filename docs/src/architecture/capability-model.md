# Capability Model

Routing in Provena is capability-based, not plugin-addressed. Clients request a capability; the kernel resolves which plugin handles it.

## Priority ordering

Each capability maps to a priority-ordered list of healthy plugin endpoints. Lower priority number means higher authority — this mirrors the DNS delegation model where 0 is highest priority.

## Singleton capabilities

Some capabilities are singletons — only one plugin may register them. Storage backends are the canonical example. The kernel rejects duplicate registrations of a singleton capability as a configuration error, not a routing choice.

Connectivity loss on a singleton capability is a hard stop (503). The kernel never silently reroutes singleton traffic.

## Fallback rules

Fallback between plugins is only valid when the plugins have identical capability and role. The kernel never falls back across semantic boundaries.

## Storage capabilities

Storage capabilities are singleton and role-scoped. The role is encoded in the capability name:

```
storage.azure.blob.standard
storage.azure.blob.sensitive
storage.azure.blob.archival
```

A plugin advertising `storage.azure.blob.standard` is never a fallback for `storage.azure.blob.sensitive`. Storage role boundaries are absolute.
