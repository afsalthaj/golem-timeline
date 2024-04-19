#!/bin/bash

echo "Welcome to that BAD shell script! Keep being sad until it disappears"

current_epoch=$(date +%s)

driver_with_core_template_name="driver_with_core${current_epoch}"

output1=$(golem-cli template add --template-name core_composed_all${current_epoch} target/wasm32-wasi/debug/core_composed_leaf.wasm)
core_composed_template_id=$(echo "$output1" | awk '/templateId:/ {print $2}')

output2=$(golem-cli template add --template-name raw_event${current_epoch} target/wasm32-wasi/debug/raw_events.wasm)
raw_events_template_id=$(echo "$output2" | awk '/templateId:/ {print $2}')

output3=$(golem-cli template add --template-name "$driver_with_core_template_name" target/wasm32-wasi/debug/driver_composed.wasm)
driver_template_id=$(echo "$output3" | awk '/templateId:/ {print $2}')

echo "Template IDs:"
echo "Core Composed: $core_composed"
echo "Raw Events: $raw_events"
echo "Driver: $driver"

# Construct the command with properly formatted parameters
command="golem-cli worker invoke-and-await --template-id \"$driver_template_id\" --worker-name first-try --function timeline:driver/api/run --parameters '[\"$core_composed_template_id\", \"$raw_events_template_id\", \"dummy\"]'"

# Output the constructed command
echo "Command:"
echo "$command"

# Execute the command
eval $command

echo "A sample invocation succeeded!"

json_template='{
  "id": "golem-timeline",
  "version": "0.0.1",
  "routes": [
    {
      "method": "Get",
      "path": "/{user-id}/instantiate-timeline",
      "binding": {
        "type": "wit-worker",
        "template": "REPLACE_DRIVER_TEMPLATE_ID",
        "workerId": "first-try",
        "functionName": "timeline:driver/api/run",
        "functionParams": ["REPLACE_CORE_COMPOSED", "REPLACE_RAW_EVENTS", "dummy"],
        "response" : "${worker.response}"
      }
    }
  ]
}'

# Replace placeholders with actual values
json_template="${json_template/REPLACE_DRIVER_TEMPLATE_ID/$driver_template_id}"
json_template="${json_template/REPLACE_CORE_COMPOSED/$core_composed_template_id}"
json_template="${json_template/REPLACE_RAW_EVENTS/$raw_events_template_id}"

echo $json_template

echo "Registering API definition with Golem..."
curl -X POST http://localhost:9881/v1/api/definitions -H "Content-Type: application/json" -d "$json_template"

