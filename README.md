# kdtree-wasm

This project is wasm wrapper for rust kdtree lib in order to use in Nodejs and browser environment.

Project inspire by https://github.com/styladev/kdtree-wasm, but seems like it doesn't update for 4 years, also this project add some improvements.

- Allow user provide distance calculation function
- Add support for both web and nodejs
- Update wasm-bindgen version so it provide better performance

# Usage

```
import { KdTree } from 'kdtree-wasm'

const kdtree = new KdTree(2)

kdtree.add(new Float64Array([1, 2]), 3);
kdtree.add(new Float64Array([6, 9]), 4);
kdtree.add(new Float64Array([3, 7]), 5);
kdtree.add(new Float64Array([7, 4]), 6);

let result = kdtree.nearest(new Float64Array([5.0, 1.0]), 1, distance)

console.log(result)

let result_euclidean = kdtree.nearest_euclidean(new Float64Array([5.0, 1.0]), 1)
console.log(result_euclidean)

let result = kdtree.within(new Float64Array([5.0, 1.0]), 20, distance)
console.log(result)

let result_euclidean = kdtree.within_euclidean(new Float64Array([5.0, 1.0]), 20)
console.log(result_euclidean)
```

For more things, check `tests/test.ts`

---

Stars the repo if you like it 
Open issues if you need more feature
