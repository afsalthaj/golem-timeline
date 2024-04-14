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


```json
result = heartbeats . toTimeline (eventTime = col ( " t "
. select (TL_DurationWhere(
    (TL_LatestEventToState( col ( "playerStateChange" )) == " buffer " ) &
    TL_HasExisted( col ( "playerStateChange") == "play" ) &
     ( ~ TL_HasExistedWithin( col ( "userAction" ) == "seek" , 5s )) &
     (TL_LatestEventToState( col ( "cdnChange")) == "CDN1")
   ). as ( "cirDuration" ))

```

```json
TLHasExistedWithin(debit, 10 sec)
               ---------------------------t11
--------------t1

TL_LatestEventToState(lat_long)
       --------t2         --------------t4
------t1         --------t3

(TL_LatestEventToState( col ( "transaction_type")) == "CDN1")
        
               -----------t3
--------------t1           ---------------t4

select(TLDurationWhere(
    (TL_LatestEventToState(col("transaction_type")) == "debit") &
    
)

```

### A simple credit card transaction outlier detection

```rust
TL_HasExistedWithin(TL_DurationInCurState(TL_LatestEventToState(col("lat_long")), col(duration) < 10)
```


## Basic structure

A driver exposing a mere run function for golem to initiate the execution of the timeline.
It internally instantiate the timeline workflow by invoking the exported function in core component.
core component further on various leaf and other composed components responsible for each node in the execution tree, such as `raw-event` component.

For now, as a POC, we are focussing on the workflow and reiterating and getting it right, to further optimise as we go.


### How to use?

```bash
cargo component build

golem-cli stubgen compose --source-wasm target/wasm32-wasi/debug/driver.wasm --stub-wasm target/wasm32-wasi/debug/core_stub.wasm --dest-wasm target/wasm32-wasi/debug/driver_composed.wasm
Writing composed component to "target/wasm32-wasi/debug/driver_composed.wasm"

## Spin up golem
curl -O https://raw.githubusercontent.com/golemcloud/golem/main/docker-examples/docker-compose-sqlite.yaml -O  https://raw.githubusercontent.com/golemcloud/golem/main/docker-examples/.env
docker-compose -f docker-compose-sqlite.yaml up


## Upload Templates

### The timeline engine, to say, keep a note of the template id, which we will use for the time being to initiate the function in the driver (below)
golem-cli template add --template-name core target/wasm32-wasi/debug/core.wasm

### The raw-events processor component - will be initiated for most of the queries as it forms the base
golem-cli template add --template-name raw-event target/wasm32-wasi/debug/raw_events.wasm

### The composed driver component - which is the a composite of the core-stub and the driver, to invoke the core functionality from the driver in a typesafe way
golem-cli template add --template-name driver target/wasm32-wasi/debug/driver_composed.wasm

### Invoke the function in the driver






```