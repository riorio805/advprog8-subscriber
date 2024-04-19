# advprog8-subscriber

bnuuy


## relfeceton

> 7a. what is amqp?

AMQP or Advanced Message Queuing Protocol, is an application layer protocol for Message-oriented middleware.
AMQP is used for communication between systems using messages, like between a Publisher and a Subscriber.

The protocol defines a message broker, which facilitates the connection between the Publisher and the Subscriber.
The Publisher sends a message to the exchange of the message broker.
Then, the message is routed based on bindings to the specified queues.
Finally, if the Subscriber subscribes to a queue, then the message is automatically delivered to the Subscriber.
They can also poll directly to a queue to receive a message, although this is discouraged. 

RabbitMQ uses AMQP 0.9.1 as its basis.

> 7b. what it means? **guest:guest@localhost:5672**, what is the first quest, and what is the second guest, and what is localhost:5672 is for?

- The first and second "guest" refers to the username and password respectively,
which is used to authenticate clients to the RabbitMQ service.
- `localhost:5672` is the IP address and port that RabbitMQ listens to by default.