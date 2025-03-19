#!/bin/bash

golem component deploy

echo "Template IDs:"
echo "Core Composed: $core_with_event_with_timeline"
echo "Raw Events: $event_processor"
echo "Driver: $driver_with_core"

driver_with_core_raw="${driver_with_core/urn:component:/}"
core_with_event_with_timeline_raw="${core_with_event_with_timeline/urn:component:/}"
event_processor_raw="${event_processor/urn:component:/}"
timeline_with_event_with_timeline_raw="${timeline_with_event_with_timeline/urn:component:/}"


# Construct the command with properly formatted parameters
command="golem worker invoke-and-await --component \"$driver_with_core\" --worker-name first-try --function timeline:driver/api.{run} --parameters '[$core_with_event_with_timeline_raw, $event_processor_raw, $timeline_with_event_with_timeline_raw]'"

echo "Invoking Driver with a TimeLineOp to initialise the whole workflow"
echo "$command"
eval $command
echo "A sample invocation succeeded!"