## Feeder

Feeder simply reads the events from Pulsar topic
and send it directly to the worker (that the driver of timeline workflow indicated)

The feeder will read the event from the topic and deserialize to almost the same shape as that of the event
expected the leaf workers

## Note
This is only to showcase the various possibilities of the architecture and in real life, the existence of feeder might be in
a different shape, and in-fact user can choose to bring them as part of the Golem Workflow itself.
That said, in most of the situations, a feeder (as well as the event producer) lies outside the Golem workflow
or even outside the network (Example: Another VPC in the same AWS account, or in a different account)

