import { test, expect } from 'vitest';
import swc from '@swc/core';
import { resolve } from 'node:path';

test('Ensure that the plugin integrates with swc', async () => {
  const code = await swc.transform(`export default { identifier: 'value', 'string.key': 'value', 1234: 'value' }`, {
    jsc: {
      experimental: {
        plugins: [
          [
            require.resolve('../target/wasm32-wasi/release/swc_plugin_object_property_reverse.wasm'),
            { displayName: true }
          ]
        ]
      }
    }
  });

  expect(code).toMatchInlineSnapshot();
});
