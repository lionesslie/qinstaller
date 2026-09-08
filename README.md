# QInstaller

Pacman ve Yay için basit bir terminal menüsü. Paket kurma/silme işlemlerini interaktif olarak yapmanızı sağlar.

## Özellikler

- Paket kurma (`pacman -S` / `yay -S`)
- Paket silme (`pacman -Rns` / `yay -Rns`)
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
