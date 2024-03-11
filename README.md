# crypto-sanctions

A console command and http server written in Rust that allows you to check whether a crypto wallet address is on the sanctions list.

## Installation

### Clone Repository

```sh
git clone git@github.com:stdfox/crypto-sanctions.git
```

### Build and Run

```sh
cargo build
```

Or build and run in release mode, with optimizations:

```sh
cargo run --release
```

## Usage

### Check Address

You can verify the crypto address using the check command:

```sh
crypto-sanctions check 1EpMiZkQVekM5ij12nMiEwttFPcDK9XhX6
```

This will print the response in JSON format like this:

```json
{"address": "1EpMiZkQVekM5ij12nMiEwttFPcDK9XhX6", "sanctioned": true}
```

It works the same for all blockchains, for example for an Ethereum address:

```sh
crypto-sanctions check 0xf3701f445b6bdafedbca97d1e477357839e4120d
```

It will print:

```json
{"address": "0xf3701f445b6bdafedbca97d1e477357839e4120d", "sanctioned": true}
```

### Run HTTP Server

You can start the http server using the following command:

```sh
crypto-sanctions serve
```

This command also allows you to set a custom host and port:

```sh
crypto-sanctions serve --host 0.0.0.0 --port 3000
```

After starting the server (this may take some time, since the server is waiting for the initial database update), you can use any HTTP/1 client to check the address:

```sh
% curl -v http://127.0.0.1:3000/api/crypto-sanctions/0xf3701f445b6bdafedbca97d1e477357839e4120d
*   Trying 127.0.0.1:3000...
* Connected to 127.0.0.1 (127.0.0.1) port 3000
> GET /api/crypto-sanctions/0xf3701f445b6bdafedbca97d1e477357839e4120d HTTP/1.1
> Host: 127.0.0.1:3000
> User-Agent: curl/8.6.0
> Accept: */*
>
< HTTP/1.1 200 OK
< Content-Type: application/json
< Content-Length: 77
< Date: Mon, 11 Mar 2024 01:32:44 GMT
<
* Connection #0 to host 127.0.0.1 left intact
{"address": "0xf3701f445b6bdafedbca97d1e477357839e4120d", "sanctioned": true}
```

## Server Performance

Since this server works with an in-memory database and does not use a serializer, it is quite performant.

See the benchmark result on my 2017 MacBook Pro (3,1 GHz Dual-Core Intel Core i5):

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
Time taken for tests:   35.827 seconds
Complete requests:      1000000
Failed requests:        0
Keep-Alive requests:    1000000
Total transferred:      209000000 bytes
HTML transferred:       77000000 bytes
Requests per second:    27911.68 [#/sec] (mean)
Time per request:       3.583 [ms] (mean)
Time per request:       0.036 [ms] (mean, across all concurrent requests)
Transfer rate:          5696.82 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    0   0.0      0       5
Processing:     0    4   1.4      4      54
Waiting:        0    4   1.4      3      54
Total:          0    4   1.4      4      54

Percentage of the requests served within a certain time (ms)
  50%      4
  66%      4
  75%      4
  80%      5
  90%      5
  95%      6
  98%      7
  99%      7
 100%     54 (longest request)
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
