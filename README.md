# kdtree-wasm

This project is wasm wrapper for rust kdtree lib in order to use in Nodejs and browser environment.

Project inspire by https://github.com/styladev/kdtree-wasm, but seems like it doesn't update for 4 years, also this project add some improvements.

- Allow user provide distance calculation function
- Add support for web with a wrapper
- Update wasm-bindgen version so it provide better performance

# Usage

## nodejs

```
import { KdTree } from 'kdtree-wasm'

function distance(a, b) {
  return Math.pow(a[0] - b[0], 2) + Math.pow(a[1] - b[1], 2)
}

const kdtree = new KdTree(2)

kdtree.add(new Float64Array([1, 2]), 3);
kdtree.add(new Float64Array([6, 9]), 4);
kdtree.add(new Float64Array([3, 7]), 5);
kdtree.add(new Float64Array([7, 4]), 6);

let result = kdtree.nearest(new Float64Array([5.0, 1.0]), 1, distance)

console.log(result)

let result_euclidean = kdtree.nearest_euclidean(new Float64Array([5.0, 1.0]), 1)
console.log(result_euclidean)

result = kdtree.within(new Float64Array([5.0, 1.0]), 20, distance)
console.log(result)

result_euclidean = kdtree.within_euclidean(new Float64Array([5.0, 1.0]), 20)
console.log(result_euclidean)
```

For more things, check `tests/test.ts`

## web

if you use bundler tools such as vite/webpack, I highly recommend associated wasm plugin

https://github.com/Menci/vite-plugin-wasm

https://github.com/wasm-tool/wasm-pack-plugin

## web with wrapper

However, a wrapper it provide to load in web

```
import { initWasm } from 'kdtree-wasm/dist/bundler/kdtree_wasm_wrapper'
import { KdTree } from 'kdtree-wasm'

await initWasm() // this magic init wasm module/instance in browser 

const kdtree = new KdTree(2)
...

```

---

Stars the repo if you like it

Open issues if you need more feature
