# Aries

A web framework for C++ language.

## Usage(test in archlinux)

### [cpp-netlib](http://cpp-netlib.org/index.html)
patch file to fix this[OPENSSL_NO_SSL3.patch](patchs/cpp-netlib-0.12.0-final/OPENSSL_NO_SSL3.patch):
```
/tmp/cpp-netlib-0.12.0-final/deps/asio/asio/include/asio/ssl/impl/context.ipp:88:31: error: no member named
      'SSLv3_method' in the global namespace
    handle_ = ::SSL_CTX_new(::SSLv3_method());
                            ~~^
/tmp/cpp-netlib-0.12.0-final/deps/asio/asio/include/asio/ssl/impl/context.ipp:91:31: error: no member named
      'SSLv3_client_method' in the global namespace
    handle_ = ::SSL_CTX_new(::SSLv3_client_method());
                            ~~^
/tmp/cpp-netlib-0.12.0-final/deps/asio/asio/include/asio/ssl/impl/context.ipp:94:31: error: no member named
      'SSLv3_server_method' in the global namespace
    handle_ = ::SSL_CTX_new(::SSLv3_server_method());
                            ~~^
3 errors generated.
```

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
