#!/bin/bash

current_epoch=$(date +%s)

driver_with_core_template_name="driver_with_core${current_epoch}"

output1=$(golem-cli template add --template-name core_composed_all${current_epoch} target/wasm32-wasi/debug/core_composed_leaf.wasm)
core_composed=$(echo "$output1" | awk '/templateId:/ {print $2}')

output2=$(golem-cli template add --template-name raw_event${current_epoch} target/wasm32-wasi/debug/raw_events.wasm)
raw_events=$(echo "$output2" | awk '/templateId:/ {print $2}')

output3=$(golem-cli template add --template-name "$driver_with_core_template_name" target/wasm32-wasi/debug/driver_composed.wasm)
driver=$(echo "$output3" | awk '/templateId:/ {print $2}')

echo "Template IDs:"
echo "Core Composed: $core_composed"
echo "Raw Events: $raw_events"
echo "Driver: $driver"

# Construct the command with properly formatted parameters
command="golem-cli worker invoke-and-await --template-name \"$driver_with_core_template_name\" --worker-name first-try --function timeline:driver/api/run --parameters '[\"$core_composed\", \"$raw_events\", \"dummy\"]'"

# Output the constructed command
echo "Command:"
echo "$command"

# Execute the command
eval $command
