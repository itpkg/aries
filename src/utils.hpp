#pragma once

#include <boost/log/trivial.hpp>
#include <cstdint>
#include <cstdlib>
#include <string>
#include <vector>

namespace aries {
namespace utils {
std::vector<uint8_t> random_bytes(uint size);
std::string to_base64(const std::string &val);
std::string from_base64(const std::string &val);
void shell(std::string cmd);
}
}
