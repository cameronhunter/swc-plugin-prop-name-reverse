# swc-plugin-object-property-reverse

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
