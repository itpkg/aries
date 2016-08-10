#ifndef AIRES_UTILS_H
#define AIRES_UTILS_H

#include <cstdint>
#include <cstdlib>
#include <string>
#include <vector>

namespace aries {
std::vector<uint8_t> random_bytes(uint size);
std::string to_base64(const std::string &val);
std::string from_base64(const std::string &val);
}

#endif
