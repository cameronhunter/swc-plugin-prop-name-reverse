# swc-plugin-prop-name-reverse

[![Rust](https://github.com/cameronhunter/swc-plugin-prop-name-reverse/actions/workflows/rust.yml/badge.svg)](https://github.com/cameronhunter/swc-plugin-prop-name-reverse/actions/workflows/rust.yml)

This is an [`swc`](https://swc.rs/) plugin that reverses the names of object properties. For example:

```ts
export default {
  identifier: 'value'
};

// ↓↓↓↓↓↓↓↓↓↓

export default {
  reifitnedi: 'value'
};
```

For more examples, see the directories in `tests/fixture`. These fixtures are run as part of the `rust` test suite.
