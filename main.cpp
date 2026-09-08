#include "src/library/library.h"
#include "src/choise_delete.cpp"
#include "src/choise_install.cpp"

using namespace std;

int main() {
    int choise;
    cout << "Welcome to QInstaller!" << endl;

    cout << "Package Install or Delete (0 / 1): ";
    cin >> choise;

    if (choise == 0) {
        choise_install();
    }
    else if (choise == 1) {
        choise_delete();
    }

    return 0;
}