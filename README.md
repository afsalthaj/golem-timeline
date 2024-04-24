## TimeLine-Golem (A work in progress)

Forget imperative style and extremely complex and hefty data piplelines and streaming (probably with Spark and python/df, SQL queries) with the help of Timeline paper from Conviva backed by durable execution of Golem.

https://www.cidrdb.org/cidr2023/papers/p22-milner.pdf

The project begins with building the library to get the following DSL working

<img width="675" alt="image" src="https://github.com/afsalthaj/golem-timeline/assets/7448613/f31587dd-ec03-4298-8dfe-9f436ac03345">


### Events Structure

Input

```hocon
[
  {
    "event": "play",
    "timestamp": 1672531200,
    "playback_session_id": "dd365258-57b9-4c17-a6ae-7d482e9ffde1",
    "video": "avengers",
    "duration": 60,
    "attributes": {     
      "device": "mobile",
      "os": "android",
      "browser": "chrome",
      "location": "US"
    }
  },
  {
    "event": "pause",
    "timestamp": 1672531260,
    "playback_session_id": "dd365258-57b9-4c17-a6ae-7d482e9ffde1",
    "video": "avengers",
    "duration": 60,
    "attributes": {
      "device": "mobile",
      "os": "android",
      "browser": "chrome",
      "location": "US"
    }
  },
  {
    "event": "seek",
    "timestamp": 1672531320,
    "playback_session_id": "dd365258-57b9-4c17-a6ae-7d482e9ffde1",
    "video": "avengers",
    "duration": 60,
    "attributes": {
      "device": "mobile",
      "os": "android",
      "browser": "chrome",
      "location": "US"
    }
  },
  {
    "event": "buffer",
    "timestamp": 1672531380,
    "playback_session_id": "dd365258-57b9-4c17-a6ae-7d482e9ffde1",
    "video": "avengers",
    "duration": 60,
    "attributes": {
      "device": "mobile",
      "os": "android",
      "browser": "chrome",
      "location": "US"
    }
  },
  {
    "event": "play",
    "timestamp": 1672531440,
    "playback_session_id": "dd365258-57b9-4c17-a6ae-7d482e9ffde1",
    "video": "avengers",
    "duration": 180,
    "attributes": {
      "device": "mobile",
      "os": "android",
      "browser": "chrome",
      "location": "US"
    }
  }
  {
    "event": "buffer",
    "timestamp": 1672531620,
    "playback_session_id": "dd365258-57b9-4c17-a6ae-7d482e9ffde1",
    "video": "avengers",
    "duration": 60,
    "attributes": {
      "device": "mobile",
      "os": "android",
      "browser": "chrome",
      "location": "US"
    }
  },
  {
    "event": "play",
    "timestamp": 1672531680,
    "playback_session_id": "dd365258-57b9-4c17-a6ae-7d482e9ffde1",
    "video": "avengers",
    "duration": 120,
    "attributes": {
      "device": "mobile",
      "os": "android",
      "browser": "chrome",
      "location": "US"
    }
  },
  {
    "event": "seek",
    "timestamp": 1672531800,
    "playback_session_id": "dd365258-57b9-4c17-a6ae-7d482e9ffde1",
    "video": "avengers",
    "duration": 60,
    "attributes": {
      "device": "mobile",
      "os": "android",
      "browser": "chrome",
      "location": "US"
    }
  },
  {
    "event": "buffer",
    "timestamp": 1672531860,
    "playback_session_id": "dd365258-57b9-4c17-a6ae-7d482e9ffde1",
    "video": "avengers",
    "duration": 60,
    "attributes": {
      "device": "mobile",
      "os": "android",
      "browser": "chrome",
      "location": "US"
    }
  }
]

```

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

## Quick Start to spin up Golem Timeline with Golem OSS

Mostly all you need is:

### Spin up golem
curl -O https://raw.githubusercontent.com/golemcloud/golem/main/docker-examples/docker-compose-sqlite.yaml -O  https://raw.githubusercontent.com/golemcloud/golem/main/docker-examples/.env
docker-compose -f docker-compose-sqlite.yaml up

### Generate all required code and build timeline project

```bash
cargo make build-flow
```

### Run a quick test

```bash
./quick-test.sh
```

This is all still a work in progress, in terms of designing and implementing a well defined workflow with implementation of every DSL nodes in TimeLine.