# Individual-Project3

## Project Description
This is a serverless application through which you can get time or date of some countries.

## Usage

### Installation

use `pip3 install cargo-lambda` to install cargo-lambda

### Steps to run

* `make format` to format code
* `make lint` to lint
* `make release-arm` to build for arm which is: `cargo lambda build --release --arm64`
* `make deploy` which is this`cargo lambda deploy`

### Available commands

* command:  
    * `date`  
    * `time`
* place:
    * `beijing`
    * `london`
    * `new york`
    * `tokyo`
    * `sydney`
    * `san francisco`

### Example

```Event json
{
  "command": "date",
  "place": "beijing"
}
```

```Result
{
  "req_id": "c791d742-785c-45bc-8500-8c9d55bb5cce",
  "msg": "The date in beijing is 2023-03-23"
}
```

## Reference

[Rust Template by Noah Gift on Github](https://github.com/noahgift/rust-new-project-template)
