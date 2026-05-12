# Publisher

This repository contains the publisher part of the Event-Driven Architecture tutorial. The program publishes five `UserCreatedEventMessage` events to RabbitMQ on the `user_created` queue.

## Reflection 1: Publisher and Message Broker

In one run, the publisher sends five pieces of event data to the message broker. Each call to `publish_event` creates one `UserCreatedEventMessage`, so the five calls in `main` produce five messages. The messages represent five different users with different `user_id` and `user_name` values. RabbitMQ receives those messages and stores them in the queue until a subscriber consumes them. This lets the publisher finish its job without waiting for the subscriber to process every message immediately.

The URL `amqp://guest:guest@localhost:5672` is the same as the one used by the subscriber because both programs must connect to the same RabbitMQ server. The first `guest` is the RabbitMQ username and the second `guest` is the password. The `localhost` host means RabbitMQ runs on the local computer. The port `5672` is RabbitMQ's default AMQP port. When both programs use this same broker address, the publisher can publish events into a queue that the subscriber is listening to.
