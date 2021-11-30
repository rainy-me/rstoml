import fs from 'fs/promises'
import path from 'path'

import b from 'benny'

import toml from '../index'

async function run() {
  const file = path.join(__dirname, '../.cache/toml-rs/test-suite/tests/valid/example-v0.4.0.toml')
  const content = await fs.readFile(file, 'utf-8')
  await b.suite(
    file,

    b.add('parseBuffer', () => {
      toml.parseBuffer(Buffer.from(content))
    }),

    b.add('toml parse', () => {
      toml.parse(content)
    }),

    b.cycle(),
    b.complete(),
  )
}

run().catch((e) => {
  console.error(e)
})
