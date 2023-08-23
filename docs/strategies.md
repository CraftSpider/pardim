
Pardim is built to take 2D arrays of tokens, such as the following:

```
>11+v
^ p<
```

And convert them into meaningful syntax graphs. They cannot be trees as many 2D languages contain lots of
self-reference.

To do this, it relies on parsing regions and paths, which consume the input, and are mapped into result
graphs.
