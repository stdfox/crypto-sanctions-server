# Crypto Sanctions Server

To make sure that you are not interacting with a sanctioned person or organization when sending or receiving cryptocurrencies, you need to run a small compliance check for the presence of crypto addresses on the sanctioned lists.

The simplest thing you can do is download the sanctioned persons list file from the [official government website](https://ofac.treasury.gov/specially-designated-nationals-list-data-formats-data-schemas) and find the address you are interested in (for example, using the grep command).

However, in a distributed system, obvious problems such as cache invalidation, outgoing requests via the Internet, and duplication of updater/parser code arise.

This project is a small http server written in Rust intended to solve these problems and allow you to check whether a crypto wallet address is on the sanctions list via simple json api.

It is an open source alternative to tools such as: [Chainalysis Sanctions Screening Tools](https://www.chainalysis.com/free-cryptocurrency-sanctions-screening-tools/) or [TRM Sanctions Screening API](https://www.trmlabs.com/products/sanctions).

At the moment, the server does not support operation over the TLS protocol, and therefore **should not be used for requests over the Internet without a part capable for terminating TLS traffic** (for example, nginx).

## Design Principles

The server is designed to be simple, fast and used behind a reverse proxy or load balancer. However, some of this may change over time.

### Server must...

- Be as fast as possible without turning source into a mess of optimized write-only code
- Be secure and follow best security practices, but no more than is required for safe use
- Follow the HTTP protocol as much as possible to ensure compatibility with any client

### Things to think about before v1.0

- Startup cache and it's invalidation policy
- Retry policy for outgoing requests
- Socks proxy (tor?) for outgoing requests
- Health status support for load balancing
- Signal processing and graceful shutdown
- Expose more command line configuration options
- JSON response format and extended info
- Dump of all database records
- Batch address check (cors support for post req?)
- TLS support
- Code coverage and package release
- Possibility of effective compression

## Installation

### Using Cargo

You can easily install the server locally using the [cargo](https://doc.rust-lang.org/cargo/) package manager command:

```sh
cargo install crypto-sanctions-server
```

### Docker

The server can run inside a docker container. Several steps required:

#### Clone repository

```sh
git clone git@github.com:stdfox/crypto-sanctions-server.git
```

#### Build docker image

```sh
docker build -t crypto-sanctions-server .
```

#### Run container

```sh
docker run --detach --rm --name=crypto-sanctions-server -p 8000:8000 -t crypto-sanctions-server
```

### Manual

#### Clone repository

```sh
git clone git@github.com:stdfox/crypto-sanctions-server.git
```

#### Build and run

```sh
cargo run
```

Or build and run in release mode, with optimizations:

```sh
cargo run --release
```

#### Run on custom port

You can start the http server on a custom host and port using the following command options:

```sh
cargo run -- --host 0.0.0.0 --port 3000
```

## Usage

### Check Address

You can check the crypto address using any available http1 client, `curl` for example:

```sh
curl http://127.0.0.1:8000/api/crypto-sanctions/1EpMiZkQVekM5ij12nMiEwttFPcDK9XhX6
```

This will print the response in JSON format like this:

```json
{"address": "1EpMiZkQVekM5ij12nMiEwttFPcDK9XhX6", "sanctioned": true}
```

It works the same for all blockchains, for example for an Ethereum address:

```sh
curl http://127.0.0.1:8000/api/crypto-sanctions/0xf3701f445b6bdafedbca97d1e477357839e4120d
```

It will print:

```json
{"address": "0xf3701f445b6bdafedbca97d1e477357839e4120d", "sanctioned": true}
```

### Short Command

If you prefer short console commands to use on desktop, you can add something like this to your shell configuration file (.bashrc, .zshrc, etc):

```sh
crypto-sanctions () {
    command curl http://127.0.0.1:8000/api/crypto-sanctions/$@
}
```

After that you can use the command as follows:

```sh
crypto-sanctions 0xf3701f445b6bdafedbca97d1e477357839e4120d
```

## Performance

Since this server works with an in-memory database and does not use a serializer, it is quite performant. See the benchmark result on my good old 2017 MacBook Pro (*3,1 GHz Dual-Core Intel Core i5*) for **release** build:

```
% ab -k -c 100 -n 1000000 -q http://127.0.0.1:8000/api/crypto-sanctions/0xf3701f445b6bdafedbca97d1e477357839e4120d
This is ApacheBench, Version 2.3 <$Revision: 1903618 $>
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking 127.0.0.1 (be patient).....done


Server Software:
Server Hostname:        127.0.0.1
Server Port:            8000

Document Path:          /api/crypto-sanctions/0xf3701f445b6bdafedbca97d1e477357839e4120d
Document Length:        77 bytes

Concurrency Level:      100
Time taken for tests:   14.328 seconds
Complete requests:      1000000
Failed requests:        0
Keep-Alive requests:    1000000
Total transferred:      209000000 bytes
HTML transferred:       77000000 bytes
Requests per second:    69793.78 [#/sec] (mean)
Time per request:       1.433 [ms] (mean)
Time per request:       0.014 [ms] (mean, across all concurrent requests)
Transfer rate:          14245.02 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    0   0.0      0       5
Processing:     0    1   0.4      1      16
Waiting:        0    1   0.4      1      16
Total:          0    1   0.4      1      16

Percentage of the requests served within a certain time (ms)
  50%      1
  66%      1
  75%      1
  80%      2
  90%      2
  95%      2
  98%      3
  99%      3
 100%     16 (longest request)
```

## Contributing

### Contributor License Agreement

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in a work by you, shall be licensed as MIT, without any additional terms or conditions.

### Commit Message Guidelines

This project has a rule on how git commit messages can be formatted. This leads to messages that are more readable and easy to follow when looking through the project history.

#### Commit Message Format

Each commit message consists of mandatory **type** and a **subject**:

```
<type>: <subject>
```

Any line of the commit message cannot be longer 100 characters.

Examples:

```
docs: add contributing guidelines to readme file
```

```
build: remove unused dependencies
```

#### Revert

If the commit reverts a previous commit, it should begin with `revert:`, followed by the header of the reverted commit.

#### Type
Must be one of the following:

* **build**: Changes that affect the build system or external dependencies
* **docs**: Documentation only changes
* **feat**: A new feature
* **fix**: A bug fix
* **perf**: A code change that improves performance
* **refactor**: A code change that neither fixes a bug nor adds a feature
* **style**: Changes that do not affect the meaning of the code
* **test**: Adding missing tests or correcting existing tests

#### Subject

The subject contains a brief description of the change:

* use the imperative, present tense: "change" not "changed" nor "changes"
* don't capitalize the first letter
* no dot (.) at the end

## License

This project is licensed under the [MIT License](LICENSE.md).
