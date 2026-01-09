#include "html.h"
#include <iostream>

std::string html_escape(const std::string& s) {
    std::string r;
    for (char c : s) {
        switch (c) {
            case '&': r += "&amp;"; break;
            case '<': r += "&lt;"; break;
            case '>': r += "&gt;"; break;
            case '"': r += "&quot;"; break;
            default:  r += c;
        }
    }
    return r;
}

void print_http_headers() {
    std::cout <<
        "Content-Type: text/html; charset=utf-8\n"
        "X-Content-Type-Options: nosniff\n"
        "Referrer-Policy: no-referrer\n"
        "Content-Security-Policy: default-src 'none'; style-src 'unsafe-inline'\n\n";
}
