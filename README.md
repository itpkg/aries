# Aries

A web framework for C++ language.

## Build

- Need install [fruit](https://github.com/google/fruit.git) first.

```
git clone https://github.com/itpkg/aries.git
cd aries

mkdir build
cd build

# gcc
cmake -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/usr ..
# clang
cmake -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_COMPILER=clang++ -DCMAKE_INSTALL_PREFIX=/usr ..

make -j
make install/strip

./demo
```

## Editor

### Atom

- language-cmake
- language-cpp14
- linter-clang
- autocomplete-clang
- git-plus
- autosave(enable)
- atom-beautify(beautify on save, clang-format)

## Thanks

- <https://google.github.io/styleguide/cppguide.html>
- <https://github.com/google/fruit>
- <https://github.com/jbeder/yaml-cpp/>
- <https://github.com/google/googletest>

- <https://github.com/gflags/gflags>
- <https://github.com/google/re2>
- <https://github.com/google/flatbuffers>
- <https://github.com/google/glog>

- <https://github.com/dropbox/json11>
- <https://github.com/facebook/proxygen>
- <https://github.com/facebook/folly>
