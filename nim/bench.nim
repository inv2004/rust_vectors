import criterion

import sequtils
import random
import stats
import math

let s = newSeqWith(1_000_000, rand(1000))
let s10 = newSeqWith(10_000_000, rand(1000))
let s11 = newSeqWith(10_000_000, rand(1000))

var cfg = newDefaultConfig()
cfg.brief = true
#cfg.minSamples = 20

benchmark cfg:
  func deltas(s: seq[int]): seq[int] =
    result = newSeq[int](s.len-1)
    for i, x in s[1..s.high]:
      result[i] = x - s[i]

  assert @[1,-2,4,1] == deltas @[1,2,0,4,5]

  func dev_1(s: seq[int]): float =
    result = standardDeviation(s)

  assert "1.414213562373095" == $dev_1 @[1,2,3,4,5]

  func dev_2(s: seq[int]): float =
    let avg = s.sum() / s.len
    let r = s.map(proc (x:int): float =
                      (x.float - avg) ^ 2
                 ).sum() / s.len.float
    result = r.sqrt()

  assert "1.414213562373095" == $dev_2 @[1,2,3,4,5]

  func dev_3(s: seq[int]): float =
    let avg = s.sum() / s.len
    var sum1 = 0.0
    for x in s:
      sum1 += (x.float - avg)^2
    result = (sum1 / s.len.float).sqrt()

  assert "1.414213562373095" == $dev_3 @[1,2,3,4,5]

  func dev_4(s: seq[int]): float =
    let l = s.len
    var sum1 = 0
    var sum2 = 0
    for x in s:
      sum1 += x
      sum2 += x*x
    sqrt(sum2/l - sum1*sum1/(l*l))

  assert "1.414213562373095" == $dev_4 @[1,2,3,4,5]

  func odd_mul_amend_1(s: seq[int]): seq[int] =
    result = s
    for x in result.mitems:
      if x mod 2 == 1:
        x *= 100

  assert @[100,2,300,4,500] == odd_mul_amend_1 @[1,2,3,4,5]

  func odd_mul_amend_2(s: seq[int]): seq[int] =
    s.map(func (x: int): int =
            if x mod 2 == 1: x * 100
            else: x
         )

  assert @[100,2,300,4,500] == odd_mul_amend_2 @[1,2,3,4,5]

  func odd_mul_1(s: seq[int]): seq[int] =
    result = newSeq[int]()
    for x in s:
      if x mod 2 == 1:
        result.add(x * 100)

  assert @[100,300,500] == odd_mul_1 @[1,2,3,4,5]

  func odd_mul_2(s: seq[int]): seq[int] =
    s.filter(func (x: int): bool = x mod 2 == 1)
     .map(func (x: int): int = x * 100)

  assert @[100,300,500] == odd_mul_2 @[1,2,3,4,5]

  func mavg_1(ws: int, s: seq[int]): seq[float] =
    var acc: int = 0
    let ws_float = ws.float
    for x in 0..(ws-2):
      acc += s[x]
    for i in (ws-1)..high(s):
      acc += s[i]
      result.add(acc.float / ws_float)
      acc -= s[1+i-ws] 

  assert @[2.0,3.0,4.0] == mavg_1(3, @[1,2,3,4,5])

  func mavg_2(ws: int, s: seq[int]): seq[float] =
    var result = newSeq[float](s.len-ws+1)
    var acc: int = 0
    let ws_float = ws.float
    for x in 0..(ws-2):
      acc += s[x]
    for i in (ws-1)..high(s):
      acc += s[i]
      result[i+1-ws] = acc.float / ws_float
      acc -= s[1+i-ws]  
    return result

  assert @[2.0,3.0,4.0] == mavg_2(3, @[1,2,3,4,5])

  func mavg_3(ws: int, s: seq[int]): seq[float] =
    var result = newSeq[float](s.len-ws+1)
    var acc: int = 0
    let ws_float = ws.float
    for x in s[0..(ws-2)]:
      acc += x
    for i, x in s[(ws-1)..high(s)]:
      acc += x
      result[i] = acc.float / ws_float
      acc -= s[i]  
    return result

  assert @[2.0,3.0,4.0] == mavg_3(3, @[1,2,3,4,5])

  func q_mavg(ws: int, s: seq[int]): seq[float] =
    var result = newSeq[float](s.len)
    var acc: int = 0
    let ws_float = ws.float
    for i in 0..(ws-2):
      acc += s[i]
      result[i] = acc.float / (i+1).float
    for i in (ws-1)..high(s):
      acc += s[i]
      result[i] = acc.float / ws_float
      acc -= s[1+i-ws]  
    return result

  assert @[1.0,1.5,2.0,3.0,4.0] == q_mavg(3, @[1,2,3,4,5])

  func max_1(s: seq[int]): int =
    for x in s:
      if x > result:
        result = x

  assert 5 == max_1 @[1,2,3,5,4]

  func max_2(s: seq[int]): int =
    s.max

  assert 5 == max_2 @[1,2,3,5,4]

  func med_1(s: seq[int]): float =
    var result = 0
    for x in s:
      result += x
    result / s.len

  assert 3.0 == med_1 @[1,2,3,5,4]
  assert 3.5 == med_1 @[1,2,3,5,4,6]

  func med_2(s: seq[int]): float =
    s.mean

  assert 3.0 == med_2 @[1,2,3,5,4]
  assert 3.5 == med_2 @[1,2,3,5,4,6]

  func wavg(s1, s2: seq[int]): float =
    var result = 0
    var weight = 0
    for i in 0..high(s1):
      result += s1[i]*s2[i]
      weight += s1[i]
    result / weight
   
  assert 3.125 == wavg(@[1,2,1,3,1], @[1,2,3,4,5])

  proc deltas_bench(x: int) {.measure: [0].} =
    blackBox deltas(s)
  proc dev_1_bench(x: int) {.measure: [0].} =
    blackBox dev_1(s)
  proc dev_2_bench(x: int) {.measure: [0].} =
    blackBox dev_2(s)
  proc dev_3_bench(x: int) {.measure: [0].} =
    blackBox dev_3(s)
  proc dev_4_bench(x: int) {.measure: [0].} =
    blackBox dev_4(s)
  proc odd_mul_amend_1_bench(x: int) {.measure: [0].} =
    blackBox odd_mul_amend_1(s)
  proc odd_mul_amend_2_bench(x: int) {.measure: [0].} =
    blackBox odd_mul_amend_2(s)
  proc odd_mul_1_bench(x: int) {.measure: [0].} =
    blackBox odd_mul_1(s)
  proc odd_mul_2_bench(x: int) {.measure: [0].} =
    blackBox odd_mul_2(s)
  proc mavg_1_bench(x: int) {.measure: [0].} =
    blackBox mavg_1(1000, s)
  proc mavg_2_bench(x: int) {.measure: [0].} =
    blackBox mavg_2(1000, s)
  proc mavg_3_bench(x: int) {.measure: [0].} =
    blackBox mavg_3(1000, s)
  proc q_mavg_bench(x: int) {.measure: [0].} =
    blackBox q_mavg(1000, s)
  proc max_1_bench(x: int) {.measure: [0].} =
    blackBox max_1(s10)
  proc max_2_bench(x: int) {.measure: [0].} =
    blackBox max_2(s10)
  proc med_1_bench(x: int) {.measure: [0].} =
    blackBox med_1(s10)
  proc med_2_bench(x: int) {.measure: [0].} =
    blackBox med_2(s10)
  proc wavg_bench(x: int) {.measure: [0].} =
    blackBox wavg(s10, s11)

