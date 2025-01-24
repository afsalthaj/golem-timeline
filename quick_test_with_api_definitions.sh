#!/bin/bash

current_epoch=$(date +%s)

echo $current_epoch

asmdriver_with_core=$(golem-cli -F json component add --component-name driver_with_core${current_epoch} target/wasm32-wasip1/debug/driver_with_core.wasm| jq .componentUrn)
echo "Successfully added driver with core"
core_with_event_with_timeline=$(golem-cli -F json component add --component-name core_with_event_with_timeline${current_epoch} target/wasm32-wasip1/debug/core_with_event_with_timeline.wasm | jq .componentUrn)
echo "Successfully added core with event with timeline"
event_processor=$(golem-cli -F json component add --component-name event_processor${current_epoch} target/wasm32-wasip1/debug/event_processor.wasm | jq .componentUrn)
echo "Successfully added event processor"
# timeline_with_event_with_timeline=$(golem-cli -F json component add --component-name timeline_with_event_with_timeline${current_epoch} target/wasm32-wasip1/debug/timeline_with_event_with_timeline.wasm | jq .componentUrn)

echo "Template IDs:"
echo "Core Composed: $core_with_event_with_timeline"
echo "Raw Events: $event_processor"
echo "Driver: $driver_with_core"

driver_with_core_raw="${driver_with_core/urn:component:/}"
core_with_event_with_timeline_raw="${core_with_event_with_timeline/urn:component:/}"
event_processor_raw="${event_processor/urn:component:/}"
#timeline_with_event_with_timeline_raw="${timeline_with_event_with_timeline/urn:component:/}"


# Construct the command with properly formatted parameters
command="golem-cli worker invoke-and-await --component \"$driver_with_core\" --worker-name first-try --function timeline:driver/api.{run} --parameters '[$core_with_event_with_timeline_raw, $event_processor_raw, $timeline_with_event_with_timeline_raw]'"

echo "Invoking Driver with a TimeLineOp to initialise the whole workflow"
echo "$command"
eval $command
echo "A sample invocation succeeded!"

echo "Exposing Timeline as API for users..."

program='let result = timeline:driver/api.{run}(REPLACE_CORE_WITH_EVENT_WITH_TIMELINE, REPLACE_EVENT_PROCESSOR, REPLACE_TIMELINE_WITH_EVENT_WITH_TIMELINE); match result { ok(value) => {body : value, status: 200u16}, err(msg) => {body: {error: msg}, status: 500u16 } }'
program="${program/REPLACE_CORE_WITH_EVENT_WITH_TIMELINE/$(echo $core_with_event_with_timeline_raw | sed 's/"/\\"/g')}"
program="${program/REPLACE_EVENT_PROCESSOR/$(echo $event_processor_raw | sed 's/"/\\"/g')}"
program="${program/REPLACE_TIMELINE_WITH_EVENT_WITH_TIMELINE/$(echo $timeline_with_event_with_timeline_raw | sed 's/"/\\"/g')}"

echo $program

api_definition='{
  "id": "golem-timeline",
  "draft": true,
  "version": REPLACE_VERSION,
  "routes": [
    {
      "method": "Get",
      "path": "/{user-id}/instantiate-timeline",
      "binding": {
        "type": "wit-worker",
        "componentId": {
           "componentId": REPLACE_DRIVER_WITH_CORE,
           "version" : 0
        },
        "workerName": "\"first-try\"",
        "response" : "${EXPRESSION}"
      }
    }
  ]
}'


# Replace placeholders with actual values
api_definition="${api_definition/REPLACE_VERSION/$current_epoch}"
api_definition="${api_definition/REPLACE_DRIVER_WITH_CORE/$driver_with_core_raw}"
api_definition="${api_definition/EXPRESSION/$program}"

echo $api_definition

echo "Registering API definition with Golem..."
curl -X POST http://localhost:9881/v1/api/definitions -H "Content-Type: application/json" -d "$api_definition"

echo ""
echo "API definition registered!"

echo "Deploying the API against localhost:9006..."
deployment='{
   "apiDefinitions": [{"id": "golem-timeline", "version": "REPLACE_VERSION"}],
   "site": {
      "host" : "localhost:9006"
  }
}'

deployment="${deployment/REPLACE_VERSION/$current_epoch}"

curl -X POST http://localhost:9881/v1/api/deployments/deploy -H "Content-Type: application/json"  -d "$deployment"

echo ""
echo "Deployment succeeded!"
echo "Testing with 'http://localhost:9006/afsal/instantiate-timeline"
curl -H "Accept: application/json" -X GET http://localhost:9006/afsal/instantiate-timeline | jq -r . | jq .
