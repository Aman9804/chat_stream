# Leptos Specification for Chat Stream Project

## 1. Project Overview

This project is a full-stack web application built with **Leptos 0.8.x** (Rust) using **Actix-Web** for the backend server. It leverages Server-Side Rendering (SSR) with client-side hydration for optimal performance and SEO.

- **Framework**: Leptos (Rust)
- **Backend**: Actix-Web
- **Build Tool**: `cargo-leptos`
- **Styling**: SCSS (compiled to CSS)

## 2. Architecture & File Structure

The project follows the standard Leptos SSR architecture:

- **`src/main.rs`**: The server entry point. Configures Actix-Web, defines server routes, and serves the static assets (`/pkg`, `/assets`).
- **`src/lib.rs`**: The client-side entry point (WASM). Contains the `hydrate` function used by the browser to attach interactivity to the SSR HTML.
- **`src/app.rs`**: The main application logic. Contains the root `App` component, the `Router`, and global providers (Meta, etc.).
- **`style/main.scss`**: The global stylesheet. `cargo-leptos` watches this file and compiles it to `pkg/chat_stream.css`.
- **`assets/`**: Public assets (images, fonts) served directly by the server.
- **`end2end/`**: End-to-end tests using Playwright.

## 3. Core Development Concepts

### 3.1 Components
- Define components using the `#[component]` macro.
- Components must return `impl IntoView`.
- Use **PascalCase** for component names (e.g., `ChatBox`, `UserProfile`).

```rust
#[component]
pub fn MyComponent(title: String) -> impl IntoView {
    view! { <div>{title}</div> }
}
```

### 3.2 State Management (Signals)
Reactivity is handled via **Signals**.
- **`RwSignal`**: Use for local state that needs read/write access.
- **`ReadSignal`/`WriteSignal`**: Use `create_signal` for simple separated read/write handles.
- **Global State**: Use `provide_context` in a parent component and `expect_context` in children to share state without prop drilling.

### 3.3 Data Fetching & Server Functions
- **Server Functions (`#[server]`)**: Use these to execute code on the server (e.g., database queries) and return data to the client.
- **Resources (`create_resource`)**: Use these to handle async data loading. They integrate with `<Suspense>` and SSR streaming.

```rust
#[server]
pub async fn fetch_messages() -> Result<Vec<String>, ServerFnError> {
    // Database logic here
    Ok(vec!["Hello".to_string()])
}
```

### 3.4 Routing
- Routes are defined in `src/app.rs` within the `<Routes>` component.
- To add a new page:
    1. Create a new component.
    2. Add a `<Route path="..." view=MyNewPage />` entry in `App`.

### 3.5 Styling
- Write styles in `style/main.scss`.
- Use standard SCSS nesting and variables.
- Styles are globally injected via `<Stylesheet id="leptos" href="/pkg/chat_stream.css"/>`.

## 4. Development Workflow

### Prerequisites
- Rust (stable or nightly).
- `cargo-leptos`: Install with `cargo install cargo-leptos`.

### Running the Project
1. **Development Server**:
   ```bash
   cargo leptos watch
   ```
   This compiles both the server (Rust) and client (WASM) and starts the app at `http://127.0.0.1:3000`.

2. **Production Build**:
   ```bash
   cargo leptos build --release
   ```
   Generates optimized binaries and WASM bundles.

## 5. Testing
- **Unit Tests**:
   - Server logic: `cargo test`
   - Client logic: `wasm-pack test --headless --firefox`
- **End-to-End Tests**:
   - Located in `end2end/`.
   - Run via `npx playwright test` (configured in `Cargo.toml`).
