#!/bin/bash

current_epoch=$(date +%s)

echo $current_epoch

driver_with_core=$(golem-cli -F json component add --component-name driver_with_core${current_epoch} target/wasm32-wasi/debug/driver_with_core.wasm| jq .componentUrn)
echo "Successfully added driver with core"
core_with_event_with_timeline=$(golem-cli -F json component add --component-name core_with_event_with_timeline${current_epoch} target/wasm32-wasi/debug/core_with_event_with_timeline.wasm | jq .componentUrn)
echo "Successfully added core with event with timeline"
event_processor=$(golem-cli -F json component add --component-name event_processor${current_epoch} target/wasm32-wasi/debug/event_processor.wasm | jq .componentUrn)
echo "Successfully added event processor"
timeline_with_event_with_timeline=$(golem-cli -F json component add --component-name timeline_with_event${current_epoch} target/wasm32-wasi/debug/timeline_with_event_with_timeline.wasm | jq .componentUrn)

echo "Template IDs:"
echo "Core Composed: $core_with_event_with_timeline"
echo "Raw Events: $event_processor"
echo "Driver: $driver_with_core"

driver_with_core_raw="${driver_with_core/urn:component:/}"
core_with_event_with_timeline_raw="${core_with_event_with_timeline/urn:component:/}"
event_processor_raw="${event_processor/urn:component:/}"
timeline_with_event_with_timeline_raw="${timeline_with_event_with_timeline/urn:component:/}"


# Construct the command with properly formatted parameters
command="golem-cli worker invoke-and-await --component \"$driver_with_core\" --worker-name first-try --function timeline:driver/api.{run} --parameters '[$core_with_event_with_timeline_raw, $event_processor_raw, $timeline_with_event_with_timeline_raw]'"

echo "Invoking Driver with a TimeLineOp to initialise the whole workflow"
echo "$command"
eval $command
echo "A sample invocation succeeded!"
