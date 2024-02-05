use std::sync::Arc;
use timeline::backend::{BackEnd};
use timeline::worker_sink::{InMemoryWorkerInvoke};
use timeline::event_record::RawEventRecord;
use timeline::event_stream::EventStream;
use timeline::event_type::EventType;
use timeline::timeline_execution::TimeLineExecution;
use chrono::{DateTime, NaiveDateTime, Utc};
fn main() {
    print!("Golem TimeLine");

    let json_data = r#"
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
  },
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
"#;

    let json_data: Vec<serde_json::Value> = serde_json::from_str(json_data).unwrap();

    let mut records: Vec<RawEventRecord> = Vec::new();
    for json_record in json_data {
        let event = json_record["event"].as_str().unwrap();

        let event_type = if event == "play" || event == "pause" || event == "seek" {
            EventType::Event(event.to_string())
        } else {
            EventType::StateDynamic(event.to_string())
        };

        let key = json_record["playback_session_id"].as_str().unwrap();
        let timestamp = json_record["timestamp"].as_u64().unwrap();
        let video = json_record["video"].as_str().unwrap();

        let record = RawEventRecord::new(key.to_string(), timestamp, event_type);
        records.push(record);
    }

    let event_stream = EventStream::InMemoryEvents(timeline::event_stream::InMemoryEventStream {
        events: records
    });

    let time_line_op = timeline::timeline_op::TimeLineOp::Leaf(event_stream);

    let mut in_memory_workers = std::sync::Arc::new(std::sync::Mutex::new(InMemoryWorkerInvoke::new()));
    let in_memory_backend = BackEnd::InMemory(Arc::clone(&in_memory_workers));
    time_line_op.run(in_memory_backend);

    let locked_workers = in_memory_workers.lock().unwrap();

    // 2024-02-05T08:14:22Z 2023-01-01T00:00:00Z "play"
    // 2023-01-01T00:00:00Z 2023-01-01T00:01:00Z "pause"
    // 2023-01-01T00:01:00Z 2023-01-01T00:02:00Z "seek"
    // 2023-01-01T00:02:00Z 2023-01-01T00:03:00Z "buffer"
    // 2023-01-01T00:03:00Z 2023-01-01T00:04:00Z "play"
    // 2023-01-01T00:04:00Z 2023-01-01T00:07:00Z "buffer"
    // 2023-01-01T00:07:00Z 2023-01-01T00:08:00Z "play"
    // 2023-01-01T00:08:00Z 2023-01-01T00:10:00Z "seek"
    // 2023-01-01T00:10:00Z 2023-01-01T00:11:00Z "buffer"
    for worker in locked_workers.workers() {
        for i in worker.timeline.points.iter() {
            println!("{:?} {:?} {:?}", timestamp_to_datetime(i.t1 as i64), timestamp_to_datetime(i.t2 as i64), i.value.to_string());
        }
    }
}

fn timestamp_to_datetime(timestamp: i64) -> DateTime<Utc> {
    // Convert the timestamp to a NaiveDateTime (seconds since Unix epoch)
    let naive_datetime = NaiveDateTime::from_timestamp_opt(timestamp, 0);
    // Convert the NaiveDateTime to a DateTime<Utc>
    DateTime::<Utc>::from_naive_utc_and_offset(naive_datetime.unwrap(), Utc)
}