# Reactive State Management

Rupaui uses a signal-based reactivity pattern similar to modern frameworks like SolidJS or Preact.

## Signals
`Signal<T>` is a container for data that can change. When its value is updated via `.set()` or `.update()`, Rupaui automatically triggers a redraw request to the OS event loop.

```rust
let count = Signal::new(0);

// In an event handler
count.update(|v| *v += 1); // Automatically triggers Redraw
```

## Memos
`Memo<T>` is used for derived state that requires calculations. A Memo takes a closure that computes its value.

```rust
let is_even = Memo::new({
    let count = count.clone();
    move || count.get() % 2 == 0
});
```

## Reactivity in Components
Components like `Button` or `Modal` receive Signals for their dynamic properties.

```rust
let is_loading = Signal::new(false);

Button::new("Save")
    .loading(is_loading.clone())
```
