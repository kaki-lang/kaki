# Concurrency

Concurrency exists in two ways:

- Cooperative multitasking using fibers
- Preemptive multitasking using threads

## Fibers

Often times cooperative multitasking is a nice way to achieve concurrency. The
main advantage of cooperative multitasking is that it is very lightweight and
processes can be switched very quickly. Even though only one processes ever
runs  at a time, it can be faster than other types of multitasking in many
scenarios, especially when a portion of the time is spent waiting (on I/O, for
example).

Cooperative multitasking is achieved using fibers, which are like functions
that can be paused and then restarted where they left off.

```kaki
fiber = Fiber.new { |x|
  y = Fiber.yield("first time yielding")
  z = Fiber.yield("second time yielding")
  Fiber.yield("sum = {}".fmt(x + y + z))
}

fiber(1) #=> "first time yielding"
fiber(2) #=> "second time yielding"
fiber(3) #=> "sum = 6"
```

When a `Fiber` yields, it returns whatever is supplied as its argument.
Multiple values can be returned by using a collection like a `List`. The next
time the fiber is called, it picks up exactly where it left off, which is the
call to `yield`. The `yield` also returns whatever argument was supplied
to it, so the fiber can get more data (should it be needed). In this way,
fibers can be used for things like _coroutines_ and _generators_.

```kaki
# A generator which produces perfect squares
squares = Fiber.new {
  n = 1
  loop {
    Fiber.yield(n ** 2)
    n += 1
  }
}

squares() #=> 1
squares() #=> 4
squares() #=> 9
squares() #=> 16
```

# Threads

!> Not yet specified
