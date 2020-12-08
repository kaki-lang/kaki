# Collections

There are four types of collections built into the language.

## Lists

Lists are an ordered collection of values. Lists are of type `List`, but they
can contain any other number of different types.

```kaki
[] # Empty list
[1, 2, 3]
["one", [2, "3"], none]
```

Lists are implemented like an array, so inserts and removals anywhere other
than the end are expensive, but random accesses are fast.

## Deques

A deque is an ordered collection, but unlike a list, it allows for fast
accesses, insertions, and removals at the beginning and end, but slower random
access. Lists are generally faster than deques, except for when a double ended
queue is needed. Deques have type `Deque`.

```kaki
Deque.new(1, 2, 3)
d = Deque.from([1, 2, 3])
d.push_front(0)
d.push_back(4)
```

## Sets

Sets, of type `Set`, are an unordered collection of unique values.

```kaki
Set.new(1, 2, 3)
Set.new("one", 2, true)
Set.from([1, 2, 3])
```

For a type to be stored in a set, it must implement the `Hash` trait.

## Hash Maps

Hash maps are an unorded collection of type `HashMaps`, which contain unique
keys that are mapped to a value. The values do not need to be unique.

```kaki
{} # Empty hash map
{"one": 1, "two": 2, "three": 3, none: "nothing"}
{true: "This is good!", false: "This is bad..."}
```

For a type to be used as a hash map key, it must implement the `Hash` trait.
Any type can be used as a hash map value, regardless of whether it implements
`Hash` or not.
