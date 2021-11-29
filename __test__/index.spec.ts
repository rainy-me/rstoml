import test from 'ava'

import { parse, parseBuffer, stringify } from '../index'

test('parse function from native code', (t) => {
  t.snapshot(parse("foo = 'bar'"))
  t.snapshot(
    parse(`
  ip = '127.0.0.1'

  [keys]
  github = 'xxxxxxxxxxxxxxxxx'
  travis = 'yyyyyyyyyyyyyyyyy'
  `),
  )
})

test('parseBuffer function from native code', (t) => {
  t.snapshot(parseBuffer(Buffer.from("foo = 'bar'", 'utf-8')))
})

test('stringify function from native code', (t) => {
  t.snapshot(stringify({ foo: 'a\nb\nc' }))
})

test('stringify function from native code with pretty', (t) => {
  t.snapshot(stringify({ foo: 'a\nb\nc' }, { pretty: true }))
})
