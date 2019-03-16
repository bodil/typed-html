# TodoMVC

`dodrio` implementation of the popular [TodoMVC](http://todomvc.com/) app. It
correctly and completely fulfills [the
specification](https://github.com/tastejs/todomvc/blob/master/app-spec.md) to
the best of my knowledge.

## Source

There are a number of modules in this `dodrio` implementation of TodoMVC. The
most important are:

* `src/lib.rs`: The entry point to the application.
* `src/todos.rs`: Definition of `Todos` model and its rendering.
* `src/todo.rs`: Definition of `Todo` model and its rendering.
* `src/controller.rs`: The controller handles UI interactions and translates
  them into updates on the model. Finally, it triggers re-rendering after those
  updates.
* `src/router.rs`: A simple URL hash-based router.

## Build

```
wasm-pack build --target no-modules
```

## Serve

Use any HTTP server, for example:

```
python -m SimpleHTTPServer
```
