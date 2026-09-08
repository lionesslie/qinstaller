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

Program sırayla şunları soracak:

1. **Kurulum mu, silme mi?** (0 / 1)
2. **Pacman mı, Yay mı?** (0 / 1)
3. **Paket adı**

## Gereksinimler

- g++ (C++11 veya üzeri)
- Arch tabanlı bir dağıtım (pacman) ve/veya yay kurulu olmalı

## Bilinen Sınırlamalar

- Paket adı girişi doğrulanmıyor; yalnızca güvendiğiniz girişlerle kullanın.
- Geçersiz menü seçimlerinde (0/1 dışı) herhangi bir hata mesajı gösterilmiyor.

## Lisans

Belirtilmemiş.
