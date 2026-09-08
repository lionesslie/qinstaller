#include "src/library/library.h"
#include "src/choise_delete.cpp"
#include "src/choise_install.cpp"
#include "src/choise_search.cpp"

using namespace std;

int main() {
    string choise;

    cout << "Welcome to QInstaller!" << endl;

    while (true) {
        cout << "[0]Package Install" << endl;
        cout << "[1]Package Delete" << endl;
        cout << "[2]Package Search" << endl;
        cout << "Input: ";
        cin >> choise;

        if (choise == "install" || choise == "Install") {
            choise_install();
        }
        else if (choise == "delete" || choise == "Delete") {
            choise_delete();
        }
        else if (choise == "search" || choise == "Search") {
            choise_search();
        }
        else {
            cout << "Wrong Input!";
        }
    }
    return 0;
}