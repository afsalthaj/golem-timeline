#!/bin/bash

echo "Welcome to that BAD shell script! Keep being sad until it disappears"

current_epoch=$(date +%s)

driver_with_core_template_name="driver_with_core${current_epoch}"

core_composed_template_id=$(golem-cli -F json template add --template-name core_composed_all${current_epoch} target/wasm32-wasi/debug/core_composed_leaf.wasm | jq .templateId)
event_processor_template_id=$(golem-cli -F json template add --template-name raw_event${current_epoch} target/wasm32-wasi/debug/event_processor.wasm | jq .templateId)
driver_template_id=$(golem-cli -F json template add --template-name "$driver_with_core_template_name" target/wasm32-wasi/debug/driver_composed.wasm| jq .templateId)

echo "Template IDs:"
echo "Core Composed: $core_composed_template_id"
echo "Raw Events: $event_processor_template_id"
echo "Driver: $driver_template_id"

# Construct the command with properly formatted parameters
command="golem-cli worker invoke-and-await --template-id \"$driver_template_id\" --worker-name first-try --function timeline:driver/api/run --parameters '[$core_composed_template_id, $event_processor_template_id, \"dummy\"]'"

# Output the constructed command
echo "A dry run on deployed timeline..."
echo "$command"
# Execute the command
eval $command
echo "A sample invocation succeeded!"

echo "Exposing Timeline as API for users..."

api_definition='{
  "id": "golem-timeline",
  "version": REPLACE_VERSION,
  "routes": [
    {
      "method": "Get",
      "path": "/{user-id}/instantiate-timeline",
      "binding": {
        "type": "wit-worker",
        "template": REPLACE_DRIVER_TEMPLATE_ID,
        "workerId": "first-try",
        "functionName": "timeline:driver/api/run",
        "functionParams": [REPLACE_CORE_COMPOSED, REPLACE_event_processor, "dummy"],
        "response" : "${ { body: worker.response, status: 200 } }"
      }
    }
  ]
}'

# Replace placeholders with actual values
api_definition="${api_definition/REPLACE_VERSION/$current_epoch}"
api_definition="${api_definition/REPLACE_DRIVER_TEMPLATE_ID/$driver_template_id}"
api_definition="${api_definition/REPLACE_CORE_COMPOSED/$core_composed_template_id}"
api_definition="${api_definition/REPLACE_event_processor/$event_processor_template_id}"

echo $api_definition

echo "Registering API definition with Golem..."
curl -X POST http://localhost:9881/v1/api/definitions -H "Content-Type: application/json" -d "$api_definition"

echo ""
echo "API definition registered!"

echo "Deploying the API against localhost:9006..."
deployment='{
   "apiDefinitionId": "golem-timeline",
   "version": REPLACE_VERSION,
   "site": "localhost:9006"
}'

deployment="${deployment/REPLACE_VERSION/$current_epoch}"

curl -X PUT http://localhost:9881/v1/api/deployments -H "Content-Type: application/json"  -d "$deployment"

echo ""
echo "Deployment succeeded!"
echo "Now you can try curl -X GET http://localhost:9006/afsal/instantiate-timeline"
