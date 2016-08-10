#ifndef AIRES_DB_H
#define AIRES_DB_H

namespace aries {
class DB {
public:
  virtual void execute(const char *sql) = 0;
};
}

#endif
