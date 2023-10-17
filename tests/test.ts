import { KdTree } from '../dist/node/kdtree_wasm'

describe('test kdtree-wasm', () => {
  let kdtree: KdTree
  let original: (message?: any, ...optionalParams: any[]) => void;

  function distance(a: Float64Array, b: Float64Array) {
    return Math.pow(a[0] - b[0], 2) + Math.pow(a[1] - b[1], 2)
  }

  beforeEach(() => {
    original = console.error
    console.error = jest.fn()

    if(kdtree) {
      kdtree.drop()
    }

    kdtree = new KdTree(2)
  })

  afterEach(() => {
    console.error = original
  })

  it('nearest', () => {
    kdtree.add(new Float64Array([1, 2]), 3);
    kdtree.add(new Float64Array([6, 9]), 4);
    kdtree.add(new Float64Array([3, 7]), 5);
    kdtree.add(new Float64Array([7, 4]), 6);
    
    expect(kdtree.size()).toEqual(4)

    const result = kdtree.nearest(new Float64Array([5.0, 1.0]), 1, distance)
    expect(result[0][0]).toEqual(13)
    expect(result[0][1]).toEqual(6)

    const result_euclidean = kdtree.nearest_euclidean(new Float64Array([5.0, 1.0]), 1)
    expect(result_euclidean[0][0]).toEqual(13)
    expect(result_euclidean[0][1]).toEqual(6)
  })

  it('within', () => {
    kdtree.add(new Float64Array([1, 2]), 3);
    kdtree.add(new Float64Array([6, 9]), 4);
    kdtree.add(new Float64Array([3, 7]), 5);
    kdtree.add(new Float64Array([7, 4]), 6);
    
    const result = kdtree.within(new Float64Array([5.0, 1.0]), 20, distance)
    expect(result[0][0]).toEqual(13)
    expect(result[0][1]).toEqual(6)
    expect(result[1][0]).toEqual(17)
    expect(result[1][1]).toEqual(3)


    const result_euclidean = kdtree.within_euclidean(new Float64Array([5.0, 1.0]), 20)
    expect(result_euclidean[0][0]).toEqual(13)
    expect(result_euclidean[0][1]).toEqual(6)
    expect(result_euclidean[1][0]).toEqual(17)
    expect(result_euclidean[1][1]).toEqual(3)
  })

  it('checks_add_len', () => {
    const t = () => {
      const kdt = new KdTree(1);
      kdt.add(new Float64Array([7, 4]), 1);
    }
    expect(t).toThrowError(WebAssembly.RuntimeError)
    expect(console.error).toHaveBeenCalledTimes(1)
  })

  it('checks_nearest_len', () => {
    const t = () => {
      const kdt = new KdTree(2);
      kdt.add(new Float64Array([7, 4]), 1);
      kdt.nearest(new Float64Array([5.0, 1.0, 2.0]), 1, distance);
    }
    expect(t).toThrowError(WebAssembly.RuntimeError)
    expect(console.error).toHaveBeenCalledTimes(1)
  })

  it('checks_within_len', () => {
    const t = () => {
      const kdt = new KdTree(2);
      kdt.add(new Float64Array([7, 4]), 1);
      kdt.within(new Float64Array([5.0, 1.0, 2.0]), 1, distance);
    }
    expect(t).toThrowError(WebAssembly.RuntimeError)
    expect(console.error).toHaveBeenCalledTimes(1)
  })
})
