# Aries

A web framework for C++ language.

## Usage(test in archlinux)

### [fruit](https://github.com/google/fruit.git).
```
git clone https://github.com/google/fruit.git
cd fruit
mkdir build
cd build
cmake -DBUILD_SHARED_LIBS:BOOL=OFF -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/usr ..
make -j
sudo make install
```

### [mstch](https://github.com/no1msd/mstch).
```
git clone https://github.com/no1msd/mstch.git
cd mstch
mkdir build
cd build
cmake -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/usr ..
make -j
sudo make install
```


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

- language-cmake
- language-cpp14
- language-mustache
- linter-clang
- autocomplete-clang
- git-plus
- autosave(enable)
- atom-beautify(beautify on save, clang-format)

## Thanks

- <https://google.github.io/styleguide/cppguide.html>
- <https://github.com/google/fruit>
- <https://github.com/jbeder/yaml-cpp/>
- <https://mustache.github.io/>
- <https://github.com/google/googletest>

- <https://github.com/gflags/gflags>
- <https://github.com/google/re2>
- <https://github.com/google/flatbuffers>
- <https://github.com/google/glog>

- <https://github.com/dropbox/json11>
- <https://github.com/facebook/proxygen>
- <https://github.com/facebook/folly>
