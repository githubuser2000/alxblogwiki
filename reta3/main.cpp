#include <iostream>
#include <vector>
#include <string>
#include <cstdlib>
#include "program.hpp"
#include "error.hpp"

void print_help() {
    std::cout << "RETA - Tabellenverarbeitungsprogramm\n\n";
    std::cout << "Verwendung: reta [OPTIONEN]\n\n";
    std::cout << "Hauptoptionen:\n";
    std::cout << "  -zeilen          Zeilenparameter festlegen\n";
    std::cout << "  -spalten         Spaltenparameter festlegen\n";
    std::cout << "  -ausgabe         Ausgabeparameter festlegen\n";
    std::cout << "  -kombination     Tabellenkombinationen\n";
    std::cout << "  -h, --help       Diese Hilfe anzeigen\n\n";
    std::cout << "Beispiele:\n";
    std::cout << "  reta -zeilen --alles --typ=sonne,mond\n";
    std::cout << "  reta -spalten --multiplikationen=2,3,5\n";
    std::cout << "  reta -ausgabe --art=html --breite=80\n\n";
    std::cout << "Ausgabeformate:\n";
    std::cout << "  --art=shell      Standard-Shell-Ausgabe (default)\n";
    std::cout << "  --art=html       HTML-Ausgabe\n";
    std::cout << "  --art=bbcode     BBCode-Ausgabe\n";
    std::cout << "  --art=markdown   Markdown-Ausgabe\n";
    std::cout << "  --art=csv        CSV-Ausgabe\n";
    std::cout << "  --art=emacs      Emacs-org-mode Ausgabe\n";
}

int main(int argc, char* argv[]) {
    try {
        std::vector<std::string> args;
        for (int i = 0; i < argc; ++i) {
            args.push_back(argv[i]);
        }
        
        if (args.size() == 1 || 
            std::find(args.begin(), args.end(), "-h") != args.end() ||
            std::find(args.begin(), args.end(), "--help") != args.end()) {
            print_help();
            return 0;
        }
        
        Program program(args, "", true);
        
        for (const auto& line : program.resulting_table()) {
            std::cout << line << "\n";
        }
        
        return 0;
    } catch (const RETAException& e) {
        std::cerr << "Fehler: " << e.what() << "\n";
        return 1;
    } catch (const std::exception& e) {
        std::cerr << "Unerwarteter Fehler: " << e.what() << "\n";
        return 2;
    }
}
