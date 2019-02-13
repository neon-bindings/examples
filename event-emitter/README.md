# event-emitter

An example of creating an `EventEmitter` with [Neon][neon].

[neon]: https://github.com/neon-bindings/neon

## Why

[Currently][rfc] it is not possible to schedule code to execute on the main
JS thread from a Rust thread. This feature would be ideal for implementing
patterns where data originates in Rust, for example, from a network socket.

Instead, we can invert the control flow and expose a polling interface from
Neon.

[rfc]: https://github.com/neon-bindings/rfcs/pull/25

## Design

At a high level, data is sourced from a Rust thread. That Rust thread sends
the data on an unbounded [`Channel`][mpsc].

The receiving end of the channel--as well as a shutdown `Sender`--is wrapped
in a Neon [`JsClass`][classes].

The class exposes a `poll` method that can be used to read data from the
channel. In the case of an [`EventEmitter`][events], the `poll` method should
be called in a loop and the data emitted.

[mpsc]: https://doc.rust-lang.org/stable/std/sync/mpsc/index.html
[classes]: https://neon-bindings.com/docs/classes
[events]: https://nodejs.org/api/events.html

## Limitations

Reading from channel is a blocking operation. To prevent this from stalling
the JS event loop, it is performed asynchronously on a [libuv][libuv] thread.

By default, the number of threads available is `4`. This puts a hard limit on
the number of calls to `poll` across instances that can run concurrently. In
addition, the thread pool is shared with several internal Node.js functions.

If you expect more than a single instance, it may be a good idea to increase
the threadpool size to match your use case. The channel is read from with a
[timeout][timeout] to yield periodically to the event loop.

[libuv]: http://libuv.org/
[threadpool-size]: https://nodejs.org/api/cli.html#cli_uv_threadpool_size_size
[timeout]: https://doc.rust-lang.org/beta/std/sync/mpsc/struct.Receiver.html#recv_timeout.v
