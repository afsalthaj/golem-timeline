#!/bin/bash

golem component deploy

# Construct the command with properly formatted parameters
command="golem worker invoke timeline:driver/first-try timeline:driver/api.{run}"

echo "Invoking Driver with a TimeLineOp to initialise the whole workflow"
echo "$command"
eval $command
echo "A sample invocation succeeded!"