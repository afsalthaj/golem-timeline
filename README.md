## TimeLine-Golem

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
                         
t1----------               t7-------------             
                         
           t2---(seek+5)---t7

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

# Install cargo make
cargo install cargo-make
```


## Golem Version Tested

```sh
golem-cli --version
golem-cli 0.0.96
```

## Quick Start to spin up Golem Timeline with Golem OSS



#### Spin up golem and Pulsar streaming 

```sh
docker-compose up -d
```

This wil deploy the OSS version of Golem , along with Pulsar (which will be used later). Pulsar sort of simulates
the existence of events in streaming platforms employed in various companies.

The docker version should correspond to 0.0.96 similar to CLI.
It's good to download the latest dockeer-compose from golem website or repository, to avoid any issues.


#### Generate all required code and build timeline project

```bash
cargo make build-flow
./build.sh
```

Please note that timeline-processor-stub project isn't listed in `Makefile.toml` due to: https://github.com/golemcloud/wasm-rpc/issues/40
This implies, **only if you are making changes to timeline-processor related wit file**s, you will run into various issues. One hack you could do is, 
after making changes the timeline=processor/wit file, generate the stub again using

```sh

golem-cli stubgen generate -s timeline-processor/wit -d timeline-processor-stub

```

And then explicitly delete the new wit directories that was created, followed by `cargo component build`, which will direct you further on what to do!
But be aware that this can be slightly tricky.

## Run a quick test

```bash
./quick-test.sh
```

`quick-test.sh` essentially registers the workflow and makes a dry run to see if everything is set up correctly.

This should give some output like this,

```bash
...
Core Composed: "16809bce-95df-4607-9697-55edb2dfea71"
Raw Events: "17e0839e-9e9b-4e3f-bcd0-26de49aefa98"
Driver: "0a3072c5-b7d7-489b-8ee8-c3add4fa093e"
A dry run on deployed timeline...

...
Invocation results in WAVE format:
- ok("cirr-le2s-playerStateChange")

...
```

## Streaming with Pulsar

Now for demo purpose we use `pulsar` for streaming. We have a sample `producer` and a `feeder` (which reads the events and feeds it to the worker that handles the events directly).
More explanations on different types of workers (processing events, processing timelines itself etc) will be given later.

You can test this workflow by first building the producer and feeder, and run them separately. 

```bash
cd sample-event-feeder
cargo build
export WORKER_NAME=cirr-le2s-playerStateChange
export COMPONENT_ID=17e0839e-9e9b-4e3f-bcd0-26de49aefa98
RUST_LOG=DEBUG target/debug/sample-event-feeder
```

Now the consumer is running, ready to accept the events produced by `sample-event-producer`. 

```bash
cd sample-event-producer
cargo build
RUST_LOG=DEBUG target/debug/sample-event-producer

```

The consumer (feeder) essentially reads the events, and feed it to the worker that is acting as leaf node (basically, reading the events directly) to compute
leaf timeline nodes. For now, our leaf node corresponds to tl_latest_event_to_state tracking playerStateChange field in the event.


This is all still a work in progress, in terms of designing and implementing a well defined workflow with implementation of every DSL nodes in TimeLine.
