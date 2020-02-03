# Collections

There are four types of collections built into the language.

## Lists

Lists are an ordered collection of values.

```kaki
[1, 2, 3]
["one", [2, "3"], none]
```

Lists are implemented like an array, so inserts and removals anywhere other
than the end are expensive, but random accesses are fast.

## Sets

Sets are an unordered collection of unique values.

```kaki
Set.new(1, 2, 3)
Set.new("one", 2, true)
Set.from([1, 2, 3])
```

## Dictionaries

Dictionaries are an unorded collection of unique keys, which each are
associated with a value.

```kaki
@{} # Empty dictionary
@{"one": 1, "two": 2, "three": 3, none: "nothing"}
@{true: "This is good!", false: "This is bad..."}
```

## Deque

A deque is an ordered collection, but unlike a list it uses linked list
semantics. This means that is supports fast insertions and removals on either
end, but slow random accesses.

```kaki
Deque.new(1, 2, 3)
Deque.new([1, 2, 3])
```
