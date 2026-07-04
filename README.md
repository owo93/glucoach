<div align="center">
  <h1>glucoach</h1>
  <p>the platform for nutritionists to manage every team, every athlete, and every meal plan.</p>
</div>

## Overview

This monorepo contains both the
[Cargo Workpace](https://doc.rust-lang.org/cargo/reference/workspaces.html) and
the [Next.js](https://nextjs.org/) frontend for the Glucoach application:

- `glucoach-api`: Axum API server
- `glucoach-lib`: Rust shared library
- `glucoach-web`: Next.js frontend application

## Usage

The root flake exposes two runnable packages, `glucoach-api` & `glucoach-web` ,
alongside a shared devShell. Each package can be built with `nix build` or run
directly with `nix run`

### running

```sh
nix run .#glucoach-api   # start the Axum API server
nix run .#glucoach-web   # start the Next.js frontend
```

### building

```sh
nix build .#glucoach-api   # builds via crane → output at result/
nix build .#glucoach-web   # builds the Next.js app → output at result/
```

### development

enter the shared devShell to access a fully configured development environment:

```sh
nix develop
```
