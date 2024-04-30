#include <iostream>
#include <string>

extern "C" int sum(int a, int b) {
  return a + b;
}

extern "C" const char* concatenateStrings(const char* str1, const char* str2) {
        std::string result = std::string(str1) + std::string(str2);
        return result.c_str();
    }
