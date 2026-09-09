Use `just run <example>` instead of `cargo run` when running an example.

## Coding guidelines

- Keep `main.rs` files minimal
- Format using `cargo +nightly fmt`
- Use `format!("{var}")` over `format!("{}", var)`
- Only use `#[allow(dead_code)]` when truly needed
- Favour `just` commands over `cargo`
- Guard against numeric over/underflow (use saturating ops)

## Dependencies

- Use `cargo add` when adding new dependencies, to ensure we're using the latest compatible version
- Prefer using features that will be easier to build (e.g. rustls over openssl)
- Run `just dep-check` when changing dependencies and fix any issues

## When finishing a task

- Run `just clippy` - fix issues
- Finally, run `just fmt`
- Update docs as needed
- Add to the "Learnings" section of AGENTS.md as appropriate - revise/update existing learnings if necessary
- Propose next steps

## Learnings

- Bevy `0.19` migration notes:
  - Light shadow-map control uses `shadow_maps_enabled` (not `shadows_enabled`).
  - `TextFont.font_size` uses a `FontSize` value, such as `FontSize::Px(20.0)`.
  - `Hdr` is in `bevy::camera`, and `OcclusionCulling` is in `bevy::render::occlusion_culling`.
  - Physics gizmos need the `bevy_gizmos_render` feature in addition to `bevy_gizmos`.
  - The world inspector needs `GizmoConfigStore` registered before `WorldInspectorPlugin` starts.
  - Bevy's `FpsOverlayPlugin` provides a small FPS and frame-time view without a separate performance UI dependency.
- Bevy `0.18` migration notes:
  - Ambient lighting as a global resource uses `GlobalAmbientLight` (not `AmbientLight`).
  - `ShaderType` field size attributes use `#[shader(size(...))]` (not `#[size(...)]`).
  - `BorderRadius` is configured on `Node.border_radius` rather than spawned as a standalone component.
- Cargo example discovery supports directory targets at `examples/<name>/main.rs`, so `just run <name>` works without explicit `[[example]]` entries.
- For large translucent custom-material set pieces that need to read well from both sides, using very thin `Cuboid` meshes is simpler than `Plane3d` because it avoids extra pipeline work for culling.
- Decorative "sun"/"moon"/lamp meshes placed along the directional light's axis must carry `NotShadowCaster` (`bevy::light::NotShadowCaster`), or their shadow eclipses a big disk of the scene — this looks like "random objects render black" far away from the prop itself.
- The camera now adopts the player's facing direction once at `PostStartup`, so `spawn_player` can aim the opening view with `Transform::looking_at`; scenes are no longer limited to composing their opening shot looking along -Z.
- `just screenshot-and-exit` sets `DIORAMA_FIXED_LOOK=1`, which disables mouse look and cursor grab for the run — screenshots are deterministic and the run doesn't capture the user's pointer. Don't rely on mouse input in CI-style runs.
- In systems that continuously enforce one `Transform` field, use `Mut::map_unchanged` with `set_if_neq`. An identical direct assignment marks the full component as changed and starts unnecessary propagation.
- A transform-only parent of visible children must include `Visibility`. Otherwise, Bevy reports B0004 and inherited visibility can be inconsistent.
- Inspect the PNG from `just screenshot-and-exit`. Bevy can exit successfully with a black image when a macOS window has no acquired render surface. An offscreen `RenderTarget::Image` can still validate the same scene and camera.
