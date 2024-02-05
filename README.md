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
