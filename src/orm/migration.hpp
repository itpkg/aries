#ifndef AIRES_ORM_MODEL_H
#define AIRES_ORM_MODEL_H

#include <string>
#include <vector>

namespace aries{
  namespace orm{
    class Migration{
    public:
      virtual std::vector<std::string> up() = 0;
      virtual std::vector<std::string> down() = 0;
      virtual std::string name() = 0;
    };
  }
}

#endif
