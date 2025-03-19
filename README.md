# Golem Timeline

[TimeLine Analytics](https://www.cidrdb.org/cidr2023/papers/p22-milner.pdf) backed by Durable Execution Engine provided by [Golem](https://learn.golem.cloud)

Watch the talk from Afsal at [LambdaConf:2024:Estes-Park:Colorado](https://www.youtube.com/watch?v=9WjUBOfgriY)

The project begins with building the library to get the basic TimeLine DSL working

Also, here is an over simplified version of initialisation phase. Every box below is a golem worker. There is a separate phase of querying but that requires more indepth explanations.

<img width="754" alt="image" src="https://github.com/user-attachments/assets/4016368e-c5e2-4799-abcb-8d08f7439bc9">


# Summary of examples (mentioned in the talk)
## Connection Induced Rebuffering Ratio


### Actual timeline
```
      |
seek  |                   seek
      |               
buffer|                               ---(buffer)---
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

The summary of the above timeline is as follows:
> User did start playing at some point. After playing user did perform a seek event
> at some point. We extend this event to a configurable 5 seconds. Even after
> extending the seek event to 5 seconds, we can see there still exists 3 seconds
> of buffering, indicating this buffering may not be the direct outcome of seek -
> contributing to the connection induced rebuffering!


## A simple credit card transaction outlier detection

```rust
TL_HasExistedWithin(TL_DurationInCurState(TL_LatestEventToState(col("lat_long")), col(duration) < 10)
```

#### Golem QuickStart
https://learn.golem.cloud/docs/quickstart

This should allow you to run a golem server locally.

```shell

golem server run

```

## Run a quick test

```bash
./quick-test.sh
```

`quick-test.sh` essentially registers the workflow and makes a dry run to see if everything is set up correctly.

This should give some output like this,

```bash
Invocation results in WAVE format:
  - ok({event-processor-workers: [leaf-timeline(tl-latest-event-to-state({worker-id: "cirr-le2s-playerStateChange", template-id: "timeline:event-processor"}))], result-worker: derived-timeline(equal-to({worker-id: "cirr-tleq-30e0f4b7-3c05-42da-8392-dcd0cc3b2096", template-id: "timeline:timeline-processor"}))})
A sample invocation succeeded!
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

## Contributing

RustRover and IntelliJ users should enable rustfmt instead of built-in formatter as described here: https://www.jetbrains.com/help/rust/rustfmt.html
