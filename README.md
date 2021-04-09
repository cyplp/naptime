# naptime

This tool parse and play scenario based on the [restclient.el](https://github.com/pashky/restclient.el) format.

## example

 ```
 # some comment
 GET http://some.url/some/path?q=1
 First-Header: some header
 Seconde-Header: another one
 {"some": "body",
  "here": "in json"}
 ```

see  [restclient.el](https://github.com/pashky/restclient.el) for more samples.


## usage

```
USAGE:
    naptime [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <file>                 File containing requests with restclient.el format
    -i, --interval <interval>         interval between two requests in milliseconds
    -p, --parameter <parameter>...
    -s, --select <select>             select only some requests as <index1,index2,index3...> start at 1

```


## roadmap

* [ ] cli:
   * [X] parameter from cli
   * [ ] format output
   * [ ] fail
   * [ ] select on title
* [ ] parsing
  * [X] parse static parameter
  * [ ] parse dynamic parameter
    * [ ] read header
	* [ ] read jsonpath response
	* [ ] read xpath response
	* [ ] lisp parameters ???
  * [ ] parse title
* [ ] formating
  * [ ] html output
  * [ ] xml output
* [ ] assert on response
* [ ] coverage 100%

## name

because I mostly write it during naps of my baby.

![Rust](https://github.com/bdejean/cmd_cache/workflows/Rust/badge.svg)

