# diorama

![Screenshot of the museum example](screenshots/museum.webp)

> ⚠️ This project is experimental and shared as-is.
> Expect rough edges, vibe-coded logic, and unreviewed code — use at your own risk!

A Bevy plugin that provides core functionality like a first person controller and setting up physics, along with a few examples using it.

## Controls

| Keys   | Description                | Features Required |
| ------ | -------------------------- | ----------------- |
| WASD   | Movement                   | -                 |
| LShift | Sprint                     | -                 |
| F3+G   | Toggle geometry wireframes | -                 |
| F3+B   | Toggle collider wireframes | `dev`             |
| F7     | Toggle world inspector     | `dev`             |
| F8     | Toggle performance UI      | `dev`             |

## Examples

Running with [just](https://github.com/casey/just) sets the correct `BEVY_ASSET_DIR` for each example.

```shell
just run <example>
just run <example> --features dev
```

- [simple](examples/simple/) - minimal scene with just a plane and a cube
- [platformer](examples/platformer/) - 3D platformer
- [museum](examples/museum/) - generated shaders, complex geometry and dialogue (mostly Sonnet 4.5)
- [ocean_depths](examples/ocean_depths/) - by Opus 4.5
- [alien_planet](examples/alien_planet/) - by Gemini 3 Pro
- [clockwork_observatory](examples/clockwork_observatory/) - floating observatory with a kinetic orrery and lantern swarms (by GPT-Codex-5.3)
- [aurora_forge](examples/aurora_forge/) - volcanic sky-forge with procedural terrain, plasma shaders, aurora sails, and drifting shard halos (by GPT-5.4)
