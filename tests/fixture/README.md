# Fixtures

To add a new fixture test, create a directory with two files:

- `input.ts` (or `input.tsx`) – This should contain the file _before_ the transform is applied.
- `output.js` – This should contain the JavaScript result _after_ the transform has been applied.

Run `cargo test` or `pnpm test` to run the tests.
