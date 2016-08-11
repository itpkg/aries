#pragma once

#include "dialect.hpp"

#include <algorithm>
#include <boost/algorithm/string.hpp>
#include <boost/typeof/typeof.hpp>
#include <cstdint>
#include <cstdlib>
#include <vector>

#define ORM_EXPORT_CLASS(Clazz)                                                \
  namespace aries {                                                            \
  namespace orm {                                                              \
  template <> std::string Dialect::tableName<Clazz>() {                        \
    std::vector<std::string> ss;                                               \
    boost::split(ss, #Clazz, boost::is_any_of("::"),                           \
                 boost::token_compress_on);                                    \
    std::string buf = boost::algorithm::join(ss, "_");                         \
    std::transform(buf.begin(), buf.end(), buf.begin(), ::tolower);            \
    return buf;                                                                \
  }                                                                            \
  }                                                                            \
  }

namespace aries {
namespace orm {}
}
