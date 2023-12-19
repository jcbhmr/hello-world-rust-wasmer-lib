import test from "node:test";
import assert from "node:assert/strict";
import { bindings } from "@jcbhmr/hello-world-wasm-lib";
const exports = await bindings.exports()
import { Calculator } from "@jcbhmr/hello-world-wasm-lib/src/bindings/exports/exports.js";

test("add(1, 2) == 3", () => {
  assert.equal(exports.add(1, 2), 3)
})

test("greet('Alan')", () => {
  console.log(exports.greet('Alan'))
})

test("greetMany(['Alan', 'Ada'])", () => {
  console.log(exports.greetMany(['Alan', 'Ada']))
})

test("distanceBetween({0,0}, {3,4}) == 5", () => {
  const a = { x: 0, y: 0 }
  const b = { x: 3, y: 4  }
  assert.equal(exports.distanceBetween(a, b), 5)
})

test("perimeterOfCircle({{0,0},5}) ~= 10pi", () => {
  console.log(exports.perimeterOfCircle({center:{x:0,y:0},radius:5}))
  console.log("10pi:", 10 * Math.PI)
})

test("areaOfCircle({{0,0},5}) ~= 25pi", () => {
  console.log(exports.areaOfCircle({center:{x:0,y:0},radius:5}))
  console.log("25pi:", 25 * Math.PI)
})

test("Calculator", async t => {
  /** @type {Calculator} */
  let calculator;
  await t.test("Calculator.new()", () => {
    calculator = Calculator.new(exports, 0)
  })
  await t.test("calculator.add(5)", () => {
    calculator.add(5)
    assert.equal(calculator.currentValue(), 5)
  })
  await t.test("calculator.multiply(2)", () => {
    calculator.multiply(2)
    assert.equal(calculator.currentValue(), 10)
  })
  console.log("calculator.currentValue():", calculator.currentValue())
})
