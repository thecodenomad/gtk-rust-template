# Gtk Rust Template

A GNOME/GTK application template written in Rust, using Blueprint for UI definition.

## Overview

This is a minimal template for creating GNOME/GTK applications using Rust with libadwaita and Blueprint. It demonstrates idiomatic handling of:

- GTK4 with libadwaita
- Blueprint templates
- GSettings for preferences
- Proper GNOME application structure

# Building

There are 2 ways to build from source: Using Nix or using Flatpak.

### Why Build a Binary via Nix?

- Local development and testing - Faster iteration cycle, no sandbox restrictions
- Quick debugging - Easier to attach debuggers, inspect logs, and test changes
- System integration - Runs directly on your host, easier to integrate with system tools
- Smaller file size - No bundled runtimes, just the binary (~5-10 MB vs ~100+ MB for Flatpak)
- No Flatpak dependencies - Doesn't require Flatpak runtime installed
- Development workflow - cargo run gives instant feedback without building containers

### Why Build a Flatpak?

- Deployment - The standard format for distributing GNOME applications
- Flathub submission - Required if you want to publish to Flathub
- Isolation - Runs in a sandbox with controlled access to system resources
- Consistency - Guarantees the same runtime environment regardless of user's distro
- Dependencies - Bundles GTK4, libadwaita, and other dependencies - works on any Linux distro with Flatpak
- Security - Sandboxed access to files, devices, and network
- Version management - Users can run multiple versions side-by-side
- Updates - Automatic updates through Flatpak runtime or Flathub

## Binary Build (Nix)

To build a standalone binary for local testing:

```bash
nix develop -c cargo build --release

# Then to run the application:
nix develop ./target/release/gtk-rust-template
```

## Flatpak Build

To build a Flatpak for deployment to Flathub:

```bash
mkdir -p build && cp build-aux/com.example.gtk_rust_template.json build/
flatpak-builder build build/com.example.gtk_rust_template.json --force-clean --install --user
```

To run the installed Flatpak:

```bash
flatpak run com.example.gtk_rust_template
```

To uninstall:

```bash
flatpak uninstall com.example.gtk_rust_template
```

### Development Build

To build and run the development version:

```bash
mkdir -p build && cp build-aux/com.example.gtk_rust_template.Devel.json build/
flatpak-builder build build/com.example.gtk_rust_template.Devel.json --force-clean --install --user
flatpak run com.example.gtk_rust_template.Devel
```

To uninstall the development version:

```bash
flatpak uninstall com.example.gtk_rust_template.Devel
```

### Creating a .flatpak File

To create a distributable `.flatpak` file:

```bash
mkdir -p repo export
flatpak-builder build build/com.example.gtk_rust_template.json --force-clean --repo=repo
flatpak build-bundle repo export/com.example.gtk_rust_template.flatpak com.example.gtk_rust_template
```

To install the `.flatpak`:

```bash
flatpak install ./export/com.example.gtk_rust_template.flatpak
```

## Project Structure

- `src/main.rs` - Application entry point
- `src/application.rs` - GtkApplication subclass
- `src/widgets/window.rs` - Main window implementation
- `src/widgets/window.blp` - Blueprint UI template
- `src/config.rs` - Application configuration
- `data/` - Desktop entry, metainfo, GSettings schema
- `flake.nix` - Nix development environment

## Customization

To customize this template for your own application:

1. Update `Cargo.toml` with your package name
2. Update `meson.build` with your project name and app ID
3. Update `data/com.example.gtk_rust_template.*` files with your app details
4. Update `src/config.rs` with your app ID
5. Update the GSettings schema in `data/*.gschema.xml`

## Attribution

This project's Meson build files, Flatpak manifest, and overall structure draw inspiration from Fretboard by Brage Fuglseth (originally under GPL-3.0). All code and configurations have been independently written and adapted with the intention of creating a template for GTK Rust-based applications. See the original project at https://github.com/bragefuglseth/fretboard for details.
