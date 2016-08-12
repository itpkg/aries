# Aries

A web framework for C++ language.

## Usage(test in archlinux)

### Clone code and build

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

## Atom editor plugins
- language-gitignore
- language-cmake
- language-cpp14
- language-mustache
- linter-clang
- autocomplete-clang
- git-plus
- autosave(enable)
- atom-beautify(beautify on save, clang-format)

## Dependencies

- <https://google.github.io/styleguide/cppguide.html>
- <https://github.com/google/fruit>
- <https://github.com/jbeder/yaml-cpp/>
- <https://mustache.github.io/>
- <https://github.com/google/googletest>
- <https://github.com/google/fruit.git>
- <https://github.com/no1msd/mstch>
