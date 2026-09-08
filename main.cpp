#include "src/library/library.h"
#include "src/choise_delete.cpp"
#include "src/choise_install.cpp"
#include "src/choise_search.cpp"
#include "src/choise_update.cpp"

using namespace std;

int main() {
    string choise;

    cout << "Welcome to QInstaller!" << endl;

    while (true) {
        cout << "[Install] Package Install" << endl;
        cout << "[Delete] Package Delete" << endl;
        cout << "[Search] Package Search" << endl;
        cout << "[Update] Package Update" << endl;
        cout << "[Quit] Package Quit" << endl;
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
        else if (choise == "update" || choise == "Update") {
            choise_update();
        }
        else if (choise == "quit" || choise == "Quit") {
            choise_update();
        }
        else {
            cout << "Wrong Input!" << endl;
        }
    }
    return 0;
}