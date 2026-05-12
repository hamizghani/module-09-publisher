# Publisher

This repository contains the publisher part of the Event-Driven Architecture tutorial. The program publishes five `UserCreatedEventMessage` events to RabbitMQ on the `user_created` queue.

## Reflection 1: Publisher and Message Broker

In one run, the publisher sends five pieces of event data to the message broker. Each call to `publish_event` creates one `UserCreatedEventMessage`, so the five calls in `main` produce five messages. The messages represent five different users with different `user_id` and `user_name` values. RabbitMQ receives those messages and stores them in the queue until a subscriber consumes them. This lets the publisher finish its job without waiting for the subscriber to process every message immediately.

The URL `amqp://guest:guest@localhost:5672` is the same as the one used by the subscriber because both programs must connect to the same RabbitMQ server. The first `guest` is the RabbitMQ username and the second `guest` is the password. The `localhost` host means RabbitMQ runs on the local computer. The port `5672` is RabbitMQ's default AMQP port. When both programs use this same broker address, the publisher can publish events into a queue that the subscriber is listening to.

## Running RabbitMQ as Message Broker

Screenshot: add the RabbitMQ management dashboard screenshot here after running RabbitMQ locally.

RabbitMQ acts as the message broker between the publisher and the subscriber. The publisher does not need to know which subscriber will process the event, because it only sends messages to RabbitMQ. The subscriber also does not need to know when the publisher is executed, because it only waits for messages from the queue. The management dashboard helps verify that RabbitMQ is running and that the AMQP and HTTP management ports are exposed correctly. In the tutorial setup, the AMQP connection uses port `5672`, while the browser dashboard uses port `15672`. This setup makes it easier to observe connections, queues, and message rates while the programs are running.

## Sending and Processing Event

Screenshot: add the terminal screenshot showing publisher execution and subscriber output here.

When the subscriber is running and the publisher is executed, the publisher sends five `UserCreatedEventMessage` events to RabbitMQ. RabbitMQ receives those events on the `user_created` queue and delivers them to the subscriber. The subscriber then prints each received message to the terminal. This proves that the publisher and subscriber are not communicating directly, but are coordinated through the message broker. If the subscriber is already connected before the publisher runs, the messages are usually consumed almost immediately. This behavior demonstrates the basic flow of event-driven architecture: produce an event, enqueue it, consume it, and process it.

## Monitoring Chart Based on Publisher

Screenshot: add the RabbitMQ message-rate chart screenshot here after repeatedly running the publisher.

The RabbitMQ chart shows a spike when the publisher is executed because the publisher sends a burst of five messages in a short time. Each run adds five publish operations to the broker, so repeated runs create repeated increases in the message rate. If the subscriber can process messages quickly, the queue may not stay high for long, but the chart still records the publish activity. If the subscriber is slow or not running, the queue size grows because messages wait in RabbitMQ until a consumer receives them. This chart is useful because it visualizes the difference between producing messages and consuming messages. In an event-driven system, this kind of monitoring helps detect whether consumers are keeping up with incoming events.
