## TimeLine-Golem (A work in progress)

Forget imperative style and extremely complex and hefty data piplelines and streaming (probably with Spark and python/df, SQL queries) with the help of Timeline paper from Conviva backed by durable execution of Golem.

https://www.cidrdb.org/cidr2023/papers/p22-milner.pdf

The project begins with building the library to get the following DSL working

<img width="675" alt="image" src="https://github.com/afsalthaj/golem-timeline/assets/7448613/f31587dd-ec03-4298-8dfe-9f436ac03345">



## CIRR


### Actual timeline
```
      |
seek  |                   seek
      |               
buffer|               \                ---(buffer)---
play  |         ---(play)--             
t ---------------------------------------------->  
                t1        t2          t3          t10
```

### TimeLine DSL semantics   

#### TLHas_Existed(play)

```
     (play)--------------------------------------
-----t1
```

#### Not TLHas_Existed_Within(seek, 5sec)

```
                         
t1----------             t5-------------             
                         
           t2---(seek)---t5

```

#### Latest state is buffer (TL_LatestEventToState)

```
            t3-------------(bufer)

-------------  
t1          t3

```

#### And all of it

```

                    t7--------t10
       
t1------t2----------t7
```

#### TL_duration_where: 

```

3sec                            /
2sec                          /
1sec                        /
0sec----------------------/
                          t7  t8 t9 t10

```

The summary of the above timeline is as follows,
user did start playing at some point. Although the user
action was seek at some point, even after giving an extension of 5 seconds
for seek, there still exists 3 seconds of buffering,
contributing to the connection induced rebuffering.


### A simple credit card transaction outlier detection

```rust
TL_HasExistedWithin(TL_DurationInCurState(TL_LatestEventToState(col("lat_long")), col(duration) < 10)
```

## Setup

```sh
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup install stable && rustup default stable
rustup target add wasm32-wasi

# Install wasm tools 
cargo install --force --locked  wasm-tools@1.0.57

# Install Cargo Component
cargo install --force --locked cargo-component@0.7.0

# Install golem-cli (OSS, for enterprise edition, golem-cloud-cli)
cargo install golem-cli
```


## Golem Version Tested

```sh
golem-cli --version
golem-cli 0.0.96
```

## Quick Start to spin up Golem Timeline with Golem OSS

Mostly all you need is:

### Spin up golem

```sh
curl -O https://raw.githubusercontent.com/golemcloud/golem/main/docker-examples/docker-compose-sqlite.yaml -O  https://raw.githubusercontent.com/golemcloud/golem/main/docker-examples/.env
docker-compose -f docker-compose-sqlite.yaml up
```

The docker version should correspond to 0.0.96 similar to CLI.


### Generate all required code and build timeline project

```bash
cargo make build-flow
```

### Run a quick test

```bash
./quick-test.sh
```

This is all still a work in progress, in terms of designing and implementing a well defined workflow with implementation of every DSL nodes in TimeLine.