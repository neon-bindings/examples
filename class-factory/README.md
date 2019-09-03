# class-factory

> Class factories and resource pools.

Often rust types will be created from a long-lived type. For example,
getting a database connection from a connection pool. The idiomatic way
to safely pass rust types to javascript is by wrapping them in a class.
However, it can be somewhat unintuitive to create a new class from
rust data in an existing class. This example provides two useful patterns.

## Patterns

### Eager Initialization

Eager initialization is the preferred method. It is simpler and requires
less boilerplate to use.

When implementing the factory/pool pattern in Rust, typically child instances
would be created directly from the parent/factory. However, since there is
no way to create a js class directly from the rust type that it wraps, this
must be slightly inverted.

The factory js class is sent as the first argument to the instance constructor.
The instance constructor accesses the internal factory to create the internal
instance object.

### Lazy Initialization

Lazy initialization is more complicated, but can also be more powerful. Instead
of passing the factory into the constructor for instances, instances are created
empty. I.e., inner values are wrapped with `Option<T>` and default to `None`.

After the instance is created, the class can be unwrapped and inner values
filled in. However, this requires all methods to handle the case where
the instance was not initialized.

## Warning

Rust destructors (`Drop`) are deterministic. Many pooling libraries rely on
this; however, the JS garbage collector is not. Data/connections borrowed
from pools may outlive their utility by a wide margin.

In order to work around this limitation, it may be useful to combine methods.
For example, if an object needs a connection, instead of embedding a connection,
embed a reference to the pool itself. Methods can grab and release connections
as part of the deterministic rust lifecycle.
