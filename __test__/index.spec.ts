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

test('parse date', (t) => {
  // offset datetime
  const odt1 = '1979-05-27T07:32:00Z'
  const odt2 = '1979-05-27T00:32:00-07:00'
  const odt3 = '1979-05-27T00:32:00.999999-07:00'

  // local datetime
  const ldt1 = '1979-05-27T07:32:00'
  const ldt2 = '1979-05-27T00:32:00.999999'

  // local date
  const ld1 = '1979-05-27'

  // local time
  const lt1 = '07:32:00'
  const lt2 = '00:32:00.999999'

  const timeOnly = (date: Date) => [date.getHours(), date.getMinutes(), date.getSeconds(), date.getMilliseconds()]
  const dateOnly = (date: Date) => date.toISOString().split('T')[0]
  const now = new Date(Date.now())

  const result = parse(`
      # offset datetime
      odt1 = ${odt1}
      odt2 = ${odt2}
      odt3 = ${odt3}

      # local datetime
      ldt1 = ${ldt1}
      ldt2 = ${ldt2}

      # local date
      ld1 = ${ld1}

      # local time
      lt1 = ${lt1}
      lt2 = ${lt2}`) as Record<string, Date>

  t.deepEqual(result.odt1.getTime(), new Date(odt1).getTime())
  t.deepEqual(result.odt2.getTime(), new Date(odt2).getTime())
  t.deepEqual(result.odt3.getTime(), new Date(odt3).getTime())
  t.deepEqual(result.ldt1.getTime(), new Date(ldt1).getTime())
  t.deepEqual(result.ldt2.getTime(), new Date(ldt2).getTime())
  t.deepEqual(dateOnly(result.ld1), dateOnly(new Date(new Date(ld1).getTime() + now.getTimezoneOffset() * 60 * 1000)))
  t.deepEqual(dateOnly(result.lt1), dateOnly(now))
  t.deepEqual(timeOnly(result.lt1), [7, 32, 0, 0])
  t.deepEqual(dateOnly(result.lt2), dateOnly(now))
  t.deepEqual(timeOnly(result.lt2), [0, 32, 0, 999])
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
