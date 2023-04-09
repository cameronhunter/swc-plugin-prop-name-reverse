import { test, expect } from 'vitest';
import swc from '@swc/core';

test('Ensure that the plugin integrates with swc', async () => {
  const { code } = await swc.transform(`export default { identifier: 'value', 'string.key': 'value', 1234: 'value' }`, {
    jsc: {
      target: 'esnext',
      parser: { syntax: 'typescript' },
      experimental: {
        plugins: [[require.resolve('..'), {}]]
      }
    }
  });

  expect(code).toMatchInlineSnapshot(`
    "export default {
        reifitnedi: \\"value\\",
        \\"yek.gnirts\\": \\"value\\",
        4321: \\"value\\"
    };
    "
  `);
});
