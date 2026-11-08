# QInstaller

Pacman ve Yay için basit bir terminal menüsü.

## Özellikler

- Paket Kurma
- Paket Silme
- Sistemi Güncelleme
- Pacman veya Yay seçimi

## Derleme

```bash
cargo build
```

## Kullanım

```bash
cargo run yada ./qinstaller
```bash

## Gereksinimler

- Rust
- Archlinux

## Dosya Ağacı

```bash

qinstaller
 ┣ src
 ┃ ┣ module
 ┃ ┃ ┣ backend
 ┃ ┃ ┃ ┣ pacman
 ┃ ┃ ┃ ┃ ┣ delete_pacman.rs
 ┃ ┃ ┃ ┃ ┣ install_pacman.rs
 ┃ ┃ ┃ ┃ ┣ search_pacman.rs
 ┃ ┃ ┃ ┃ ┗ update_pacman.rs
 ┃ ┃ ┃ ┗ yay
 ┃ ┃ ┃ ┃ ┣ delete_yay.rs
 ┃ ┃ ┃ ┃ ┣ install_yay.rs
 ┃ ┃ ┃ ┃ ┣ search_yay.rs
 ┃ ┃ ┃ ┃ ┗ update_yay.rs
 ┃ ┃ ┣ delete.rs
 ┃ ┃ ┣ install.rs
 ┃ ┃ ┣ search.rs
 ┃ ┃ ┗ update.rs
 ┃ ┗ main.rs
 ┣ .gitignore
 ┣ Cargo.lock
 ┗ Cargo.toml

```
