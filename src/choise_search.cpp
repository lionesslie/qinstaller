#include "library/library.h"

using namespace std;

void choise_search() {
    string pkg;
    int choise_frmt;

    cout << "Pacman or Yay (0 / 1): ";
    cin >> choise_frmt;

    if (choise_frmt == 0) {
        cout << "Package Name: ";
        cin >> pkg;

        string command = ("sudo pacman -Ss " + pkg);
        system(command.c_str());
    }
    else if (choise_frmt == 1) {
        cout << "Package Name: ";
        cin >> pkg;

        string command = ("yay -Ss " + pkg);
        system(command.c_str());
    }
}