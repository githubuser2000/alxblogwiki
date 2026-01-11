#include "CSVWriter.h"
#include <fstream>
#include <sstream>

void CSVWriter::writeCSV(const std::string& filename, const std::vector<TokenInfo>& tokens) {
    std::ofstream csv(filename, std::ios::binary);
    const unsigned char bom[] = {0xEF, 0xBB, 0xBF};
    csv.write(reinterpret_cast<const char*>(bom), sizeof(bom));

    csv << "Token,Dateiname,HäufigkeitGesamt,HäufigkeitDatei,Position\n";

    for (const auto& tok : tokens) {
        std::ostringstream pos_stream;
        for (size_t i = 0; i < tok.positions.size(); ++i) {
            pos_stream << tok.positions[i];
            if (i + 1 < tok.positions.size()) pos_stream << ";";
        }

        csv << tok.token << ","
            << tok.filename << ","
            << tok.global_count << ","
            << tok.file_count << ","
            << pos_stream.str()
            << "\n";
    }
}
