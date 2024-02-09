Rust Mocking Solutions
======================

Though Rust may be a fantastic language, that doesn't mean it is without difficult problems that need (potentially) complicated solutions.

One such problem comes in the form of dependency injection and mocking. The [mockall][1] crate does a great job of providing most of a solution, but we are still on our own to figure out how to share and inject those mockable dependencies.

This project lays out three examples of dependencies that need to be mocked such that the parent object can be unit tested without concern for the business logic in the dependency.

Simple Example
--------------

In `src/simple/main.rs`, we have a simple project with a dependency `MyStruct`. This dependency has a trait `MyTrait` covering all publicly accessible methods, allowing `mockall` to be used by unit tests and the concrete `MyStruct` in production code.

In the simplest of programs, like this one, passing immutable dependencies by reference is low overhead and requires the smallest amount of changes. See `src/simple/main.rs` for an example of basic dependency injection and mocking. This solution falls apart, however, when threading or an asynchronous runtime (like [Tokio][2]) gets involved.

Threadsafe Example
------------------

In `src/threadsafe/main.rs`, the [dyn-clone][3] crate is used to show how the `Box<dyn Middleman>` dependency can be passed across threads and asynchronous tasks without the overhead of reference counters or mutexes. This is predicated on the assumption that the dependencies do not consume a significant amount of memory at runtime - that each dependency contains only other "logic" dependencies and no "data" dependencies. For many object-inspired programs, objects contain _either_ data _or_ logic, and not both, so this assumption holds true. When this is not true - when there is data or state that needs to be shared across threads or objects - see the third and final solution below.

Shared Resources Example
------------------------

Finally, the most complicated scenario is a shared resource that must be passed across threads and/or held and used by multiple dependencies. In this scenario, the shared resource (such as a physical communication medium like a serial port), must:

* Have its memory managed at runtime with a reference counter 
* Have the reference counter operations be atomic in order to stay threadsafe
* Have the object itself guarded by a mutex in order to prevent simultaneous calls across different threads

The `Arc<Mutex<T>>` pattern is used to accomplish all of these. Though it works, it is verbose and cumbersome; prefer the simple `dyn-clone` example when possible.


[1]: https://docs.rs/mockall/latest/mockall/
[2]: https://docs.rs/tokio/latest/tokio/
[3]: https://docs.rs/dyn-clone/latest/dyn_clone/
