import subprocess

def question():
    while True:
        secim = str(input("Yüklemekmi Silmekmi (I/D): ").strip().upper())

        if secim == "I":
            sec = str(input("Pacman yada Yay: (P/Y): ").strip().upper())
            if sec == "P":
                install_pkg()
            elif sec == "Y":
                install_yay()

        elif secim == "D":
            sec = str(input("Pacman yada Yay: (P/Y): ").strip().upper())
            if sec == "P":
                delete_pkg()
            elif sec == "Y":
                delete_yay()
        else:
            print("Yanlış Seçim: (I/D)")

def install_pkg():
    package = str(input("Paket Adı Gir: "))
    subprocess.run(f"sudo pacman -S {package}", shell=True)

def delete_pkg():
    delpackage = str(input("Paket Adı Gir: "))
    subprocess.run(f"sudo pacman -Rns {delpackage}", shell=True)

def install_yay():
    package = str(input("Paket Adı Gir: "))
    subprocess.run(f" yay -S {package}", shell=True)

def delete_yay():
    delpackage = str(input("Paket Adı Gir: "))
    subprocess.run(f"yay -Rns {delpackage}", shell=True)