#include "models.hpp"

ORM_EXPORT_CLASS(aries::platform::User)

namespace aries {
namespace orm {}
}

namespace aries {
namespace platform {
template <class T> void User::hibernate(T &t) {}
}
}
