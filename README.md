# gist-rs
[![Crates.io](https://img.shields.io/crates/v/pf.svg?style=plastic)](http://crates.io/crates/gist-rs)
[![Build Status](https://travis-ci.org/robatipoor/pf.svg?branch=master)](https://travis-ci.org/robatipoor/gist-rs)
[![Build status](https://ci.appveyor.com/api/projects/status/d2we8j2c58n6wq7o?svg=true)](https://ci.appveyor.com/project/robatipoor/gist-rs)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

client tool for github gist


**install**

```sh
cargo install gist-rs
```

**Build and install**

```sh
# build and install gist-rs
git clone https://github.com/robatipoor/gist-rs \
&& cd gist-rs \
&& make 
```

**Build dependency**

git, rustc, cargo, gnu make, binutils, upx

**run**

```sh
# login github gist
gist-rs --login "token string"
# sync list gist 
gist-rs -s
# list gist 
gist-rs -l
# sync and list gist 
gist-rs -sl
# post gist
echo 'Hello !' | gist-rs
# post gist
gist-rs post -p file.txt
# get file 
gist-rs get -u url
# delete file by id
gist-rs delete -i id
```