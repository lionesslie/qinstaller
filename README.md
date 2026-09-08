# QInstaller

Pacman ve Yay için basit bir terminal menüsü.

## Özellikler

- Paket Kurma (`pacman -S` / `yay -S`)
- Paket Silme (`pacman -Rns` / `yay -Rns`)
- Paket Arama (`pacman -Ss` / `yay -Ss`)
- Pacman veya Yay seçimi

## Derleme

```bash
g++ main.cpp -o qinstaller
```

## Kullanım

```bash
./qinstaller
```

## Gereksinimler

- g++ (C++11 veya üzeri)
- Arch tabanlı bir dağıtım (pacman) ve/veya yay kurulu olmalı
