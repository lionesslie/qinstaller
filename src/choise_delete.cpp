#include "library/library.h"

using namespace std;

void choise_delete() {
    string pkg;
    string choise_frmt;

    cout << "Pacman or Yay: ";
    cin >> choise_frmt;

    if (choise_frmt == "pacman" || choise_frmt == "Pacman") {
        cout << "Package Name: ";
        cin >> pkg;

        string command = ("sudo pacman -Rns " + pkg);
        system(command.c_str());
    }
    else if (choise_frmt == "yay" || choise_frmt == "Yay") {
        cout << "Package Name: ";
        cin >> pkg;

        string command = ("yay -Rns " + pkg);
        system(command.c_str());
    }
}