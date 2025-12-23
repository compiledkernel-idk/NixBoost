# nixboost 

**nixboost** is a fast, no-nonsense frontend for managing packages on **NixOS**.
It makes searching and installing packages from **nixpkgs** and **NUR** feel instant, predictable, and actually enjoyable.

Think *pacman UX*, but for NixOS — without breaking purity :3

---

## ✨ Features

* ⚡ **Blazing-fast search**
  Parallelized queries so you’re not staring at a terminal wondering if Nix froze again.

* 🧩 **NUR integration**
  Seamlessly search and install packages from the Nix User Repository.

* 🧠 **Smart fallback logic**
  If a package isn’t in `nixpkgs`, `nixboost` automatically checks NUR for you.

* 🛠️ **System utilities built-in**
  Quick access to system health checks, generations, news, and garbage collection.

---

## 📦 Installation

### Recommended (NixOS-native)

If you’re on NixOS, this is the clean way:

```bash
nix profile install github:NacreousDawn596/nixboost
```

> Reproducible, rebuild-safe, zero `/usr/local` crimes.

---

### From source

If you want to build it yourself:

```bash
nix-shell
cargo build --release # or download the latest release directly from GitHub
cp target/release/nixboost ~/.local/bin/
```

Make sure `~/.local/bin` is in your `PATH`.

---

## 🧑‍💻 Usage

### Package management

```bash
nixboost -S  <pkg>     # Install (checks nixpkgs, then NUR)
nixboost -R  <pkg>     # Remove with confirmation
nixboost -Ss <query>   # Search nixpkgs
nixboost -A  <query>   # Search NUR (full index)
nixboost -l            # List installed packages
```

---

### System utilities

```bash
nixboost --news        # Latest NixOS news & blog posts
nixboost --history     # View system generations
nixboost --health      # System & store health check
nixboost --clean       # Garbage collect & free space
```

---

## 🧠 Philosophy

* Fast feedback
* Minimal friction
* No magic state
* Nix-first, always

If it doesn’t respect NixOS semantics, it doesn’t belong here.

---

## 📜 License

**GPL-3.0**

Built with too much caffeine ☕, exam pressure, zero sleep, and ❤️ for the NixOS community.

---