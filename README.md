# aries

A web framework for C++ language.

## Build
```
git clone https://github.com/itpkg/aries.git
cd aries

mkdir build
cd build

# gcc
cmake -DCMAKE_BUILD_TYPE=Release ..
# clang
cmake -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_COMPILER=clang++ ..

make
make install

./demo
```

## Editor
### Atom
* language-cmake
* linter-c++
* autocomplete-clang
* git-plus

## Thanks

- <https://google.github.io/styleguide/cppguide.html>
- <https://github.com/google/fruit>
- <https://github.com/gflags/gflags>

- <https://github.com/google/re2>
- <https://github.com/google/flatbuffers>
- <https://github.com/google/glog>
- <https://github.com/google/googletest>
- <https://github.com/dropbox/json11>
- <https://github.com/facebook/proxygen>
- <https://github.com/facebook/folly>
