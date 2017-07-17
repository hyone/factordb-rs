# factordb

[![Build Status](https://travis-ci.org/hyone/factordb-rs.svg?branch=master)](https://travis-ci.org/hyone/factordb-rs)
[![Build status](https://ci.appveyor.com/api/projects/status/27794ypbvorl8257?svg=true)](https://ci.appveyor.com/project/hyone/factordb-rs)

`factordb` get the factorization of the number from factordb.com

[Documentation](http://hyone.github.io/factordb-rs).

### Usage

```shell
$ factordb 336
2 2 2 2 3 7

$ factordb -r 336
{"id":"336","status":"FF","factors":[["2",4],["3",1],["7",1]]}
```

    USAGE:
        factordb [FLAGS] <NUM>

    FLAGS:
        -h, --help       Prints help information
        -r, --raw        Display the answer as raw json format
        -V, --version    Prints version information

    ARGS:
        <NUM>    Number to be factored
