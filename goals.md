# Interface
* Using this library should be 3 steps
    - Create the scene
    - Render the scene
    - Display the render on a window and/or save the render


## Creating the Scene

### Problems

Materials are owned by their object. Thus multiple objects cannot share the same material -- which is very inefficient for Rect3ds, for example

**Solution**: Create a `MaterialLibrary`, indexed by a `MaterialIndex`

Translate, Rotate, and Scale, and FlipNormals are convoluted. They should not be a wrapper object creating an unnecessary layer of indirection.

**Solution**: Create `Render` trait to replace `Hitable`. A global 

Camera and World are specified awkwardly.

**Solution**: Wrap everything in a `Scene` struct.

## Render the Scene

```rust
let renderer: Renderer = RenderParams::new(scene)
                .use_bvh(true)
                .multithreaded(true)
                .with_progress_bar(true)
                .show_time(true)
                .gamma(0.5)
                .build();

let render: Vec<Vec3> = renderer.render();
```

## Display the Render

```rust
// This should assume sensible defaults for everything. 60 FPS, Non-resizable, etc. The user should be able to change those with `render_display.window_mut()`, which should return a mutable reference to the window.
let render_display = RenderDisplay::new("Name", WIDTH, HEIGHT);

render_display.show(render); // This should handle the eventloop, menu, F3 for saving, everything
```
