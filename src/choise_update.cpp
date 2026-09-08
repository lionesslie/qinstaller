#include "library/library.h"

using namespace std;

void choise_update() {
    string pkg;
    string choise_frmt;

    cout << "Pacman or Yay: ";
    cin >> choise_frmt;

    if (choise_frmt == "pacman" || choise_frmt == "Pacman") {
        string command = ("sudo pacman -Syu ");
        system(command.c_str());
    }
    else if (choise_frmt == "yay" || choise_frmt == "Yay") {
        string command = ("yay -Syu ");
        system(command.c_str());
    }
}