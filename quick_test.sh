#!/bin/bash

echo "Welcome to that BAD shell script! Keep being sad until it disappears"

current_epoch=$(date +%s)

driver_with_core=$(golem-cli -F json component add --component-name driver_with_core${current_epoch} target/wasm32-wasi/debug/driver_with_core.wasm| jq .componentId)
core_with_event_with_timeline=$(golem-cli -F json component add --component-name core_with_event_with_timeline${current_epoch} target/wasm32-wasi/debug/core_with_event_with_timeline.wasm | jq .componentId)
event_processor=$(golem-cli -F json component add --component-name event_processor${current_epoch} target/wasm32-wasi/debug/event_processor.wasm | jq .componentId)
timeline_with_event_with_timeline=$(golem-cli -F json component add --component-name timeline_with_event_with_timeline${current_epoch} target/wasm32-wasi/debug/timeline_with_event_with_timeline.wasm | jq .componentId)

echo "Template IDs:"
echo "Core Composed: $core_with_event_with_timeline"
echo "Raw Events: $event_processor"
echo "Driver: $driver_with_core"

# Construct the command with properly formatted parameters
command="golem-cli worker invoke-and-await --component-id \"$driver_with_core\" --worker-name first-try --function timeline:driver/api/run --parameters '[$core_with_event_with_timeline, $event_processor, $timeline_with_event_with_timeline]'"

# Output the constructed command
echo "A dry run on deployed timeline..."
echo "$command"
# Execute the command
eval $command
echo "A sample invocation succeeded!"

echo "Exposing Timeline as API for users..."

program='let result = timeline:driver/api/run(REPLACE_CORE_WITH_EVENT_WITH_TIMELINE, REPLACE_EVENT_PROCESSOR, REPLACE_TIMELINE_WITH_EVENT_WITH_TIMELINE); { body: match result { ok(value) => value, err(msg) => msg }, status: match result { ok(value) => 200, err(msg) => 500 }, headers : {} }'
program="${program/REPLACE_CORE_WITH_EVENT_WITH_TIMELINE/$(echo $core_with_event_with_timeline | sed 's/"/\\"/g')}"
program="${program/REPLACE_EVENT_PROCESSOR/$(echo $event_processor | sed 's/"/\\"/g')}"
program="${program/REPLACE_TIMELINE_WITH_EVENT_WITH_TIMELINE/$(echo $timeline_with_event_with_timeline | sed 's/"/\\"/g')}"

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
        "componentId": REPLACE_DRIVER_WITH_CORE,
        "workerName": "first-try",
        "response" : "${EXPRESSION}"
      }
    }
  ]
}'

# Replace placeholders with actual values
api_definition="${api_definition/REPLACE_VERSION/$current_epoch}"
api_definition="${api_definition/REPLACE_DRIVER_WITH_CORE/$driver_with_core}"
api_definition="${api_definition/EXPRESSION/$program}"

echo $api_definition

echo "Registering API definition with Golem..."
curl -X POST http://localhost:9881/v1/api/definitions -H "Content-Type: application/json" -d "$api_definition"

echo ""
echo "API definition registered!"

echo "Deploying the API against localhost:9006..."
deployment='{
   "apiDefinitionId": "golem-timeline",
   "version": REPLACE_VERSION,
   "site": {
      "host" : "localhost:9006"
  }
}'

deployment="${deployment/REPLACE_VERSION/$current_epoch}"

curl -X POST http://localhost:9881/v1/api/deployments/deploy -H "Content-Type: application/json"  -d "$deployment"

echo ""
echo "Deployment succeeded!"
echo "Testing with 'curl -X GET http://localhost:9006/afsal/instantiate-timeline| jq -r . | jq .'"
curl -H "Accept: application/json" -X GET http://localhost:9006/afsal/instantiate-timeline | jq -r . | jq .