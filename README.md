## TimeLine-Golem

Forget imperative style and extremely complex and hefty complex data piplelines and streaming (that probably using Spark and python/df, SQL queries) with the help of Timeline paper from Conviva backed by durable execution of Golem.


### Events Structure

Input

https://www.cidrdb.org/cidr2023/papers/p22-milner.pdf

```hocon

   {
    "event": "play",
    "timestamp": 1234567890,
    "user": "user1",
    # suggest a good movie video copilot
    "video": "avengers",
    "duration": 60,
    "attributes": {
        "device": "mobile",
        "os": "android",
        "browser": "chrome",
        "location": "US"
    }
   }

   {
    "event": "pause",
    "timestamp": 1234567890,
    "user": "user1",
    "video": "avengers",
    "duration": 60,
    "attributes": {
        "device": "mobile",
        "os": "android",
        "browser": "chrome",
        "location": "US"
    }
   }
    
   {
    "event": "seek",
    "timestamp": 1234567890,
    "user": "user1",
    "video": "avengers",
    "duration": 60,
    "attributes" : {
        "device": "mobile",
        "os": "android",
        "browser": "chrome",
        "location": "US"
    }
   }
    
   {
    "event": "buffer",
    "timestamp": 1234567890,
    "user": "user1",
    "video": "avengers",
    "duration": 60,
    "attributes" : {
        "device": "mobile",
        "os": "android",
        "browser": "chrome",
        "location": "US"
    }
    
   }
    
   {
    "event": "play",
    "timestamp": 1234567890,
    "user": "user1",
    "video": "avengers",
    "duration": 60,
    "attributes" : {
        "device": "mobile",
        "os": "android",
        "browser": "chrome",
        "location": "US"
    }
   }
    
   {
    "event": "pause",
    "timestamp": 1234567890,
    "user": "user1",
    "video": "avengers",
    "duration": 60,
    "attributes" : {
        "device": "mobile",
        "os": "android",
        "browser": "chrome",
        "location": "US"
    }
   }
    
   {
    "event": "seek",
    "timestamp": 1234567890,
    "user": "user1",
    "video": "avengers",
    "duration": 60,
    "attributes" : {
        "device": "mobile",
        "os": "android",
        "browser": "chrome",
        "location": "US"
    }
   }
    
   {
    "event": "buffer",
    "timestamp": 1234567890,
    "user": "user1",
    "video": "avengers",
    "duration": 60,
    "attributes" : {
        "device": "mobile",
        "os": "android",
        "browser": "chrome",
        "location": "US"
    }
   }
    
   {
    "event": "play",
    "timestamp": 1234567890,
    "user": "user1",
    "video": "avengers",
    "duration": 60,
    "attributes" : {
        "device": "mobile",
        "os": "android",
        "browser": "chrome",
        "location": "US"
    }
   }
    
    {
    "event": "pause",
    "timestamp": 1234567890,
    "user": "user1",
    "video": "avengers",
    "duration": 60
    }
    
    {
    "event": "seek",
    "timestamp": 1234567890,
    "user": "user1",
    "video": "avengers",
    "duration": 60
    }
    
    {
    "event": "buffer",
    "timestamp": 1234567890,
    "user": "user1",
    "video": "avengers",
    "duration": 60
    }
    
    {
    "event": "play",
    "timestamp": 1234567890,
    "user": "user1",
    "video": "avengers",
    "duration":
}
```

```