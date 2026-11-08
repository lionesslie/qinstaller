# QInstaller

Pacman ve Yay paket yöneticileri için geliştirilmiş basit bir terminal aracı.

## Özellikler

- Paket kurma
- Paket silme
- Paket arama
- Sistem güncelleme
- Pacman veya Yay seçimi

## Gereksinimler

- Arch Linux
- Rust
- Cargo
- Yay (AUR desteği kullanılacaksa)

## Derleme

Projeyi klonladıktan sonra proje dizinine girin:

```bash
git clone https://github.com/lionesslie/qinstaller.git
cd qinstaller
```

Projeyi derlemek için:

```bash
cargo build --release
```

## Kullanım

Cargo ile çalıştırmak için:

```bash
cargo run
```

Derlenmiş uygulamayı çalıştırmak için:

```bash
./target/release/qinstaller
```

## Dosya Yapısı

```text
qinstaller/
┣ src/
┃ ┣ module/
┃ ┃ ┣ backend/
┃ ┃ ┃ ┣ pacman/
┃ ┃ ┃ ┃ ┣ delete_pacman.rs
┃ ┃ ┃ ┃ ┣ install_pacman.rs
┃ ┃ ┃ ┃ ┣ search_pacman.rs
┃ ┃ ┃ ┃ ┗ update_pacman.rs
┃ ┃ ┃ ┗ yay/
┃ ┃ ┃   ┣ delete_yay.rs
┃ ┃ ┃   ┣ install_yay.rs
┃ ┃ ┃   ┣ search_yay.rs
┃ ┃ ┃   ┗ update_yay.rs
┃ ┃ ┣ delete.rs
┃ ┃ ┣ install.rs
┃ ┃ ┣ search.rs
┃ ┃ ┗ update.rs
┃ ┗ main.rs
┣ .gitignore
┣ Cargo.lock
┗ Cargo.toml
```
