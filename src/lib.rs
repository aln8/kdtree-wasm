use kdtree;
use serde;
use wasm_bindgen::prelude::*;

type PointUnit = f64;
type Point = Vec<PointUnit>;
type Data = f64;

#[wasm_bindgen]
pub struct KdTree {
    kdtree: kdtree::KdTree<PointUnit, Data, Point>,
    dimensions: usize,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

fn assert_error(cond: bool, msg: &str) {
    if cond {
        return;
    }

    error(msg);
    panic!("{}", msg);
}

#[wasm_bindgen]
impl KdTree {
    #[wasm_bindgen(constructor)]
    pub fn new(dimensions: usize) -> KdTree {
        assert_error(dimensions > 0, "Dimensions must be set and >= 1");

        KdTree {
            kdtree: kdtree::KdTree::new(dimensions),
            dimensions,
        }
    }

    pub fn size(&self) -> usize {
        self.kdtree.size()
    }

    #[inline(always)]
    fn check_point(&self, point: &Point) {
        assert_error(
            point.len() == self.dimensions,
            "Point dimensions do not match KdTree dimensions",
        );
    }

    pub fn within(&self, point: Point, radius: PointUnit, distance: &js_sys::Function) -> JsValue {
        self.check_point(&point);

        let val: Vec<(PointUnit, &Data)> = self
            .kdtree
            .within(&point.clone(), radius, &js_fn_into_distance(&distance))
            .unwrap();

        serde_wasm_bindgen::to_value(&val).unwrap()
    }

    pub fn within_euclidean(&self, point: Point, radius: PointUnit) -> JsValue {
        self.check_point(&point);

        let val: Vec<(PointUnit, &Data)> = self
            .kdtree
            .within(&point.clone(), radius, &kdtree::distance::squared_euclidean)
            .unwrap();

        serde_wasm_bindgen::to_value(&val).unwrap()
    }

    pub fn nearest(&mut self, point: Point, num: usize, distance: &js_sys::Function) -> JsValue {
        self.check_point(&point);
        let val: Vec<(PointUnit, &Data)> = self
            .kdtree
            .nearest(&point.clone(), num, &js_fn_into_distance(&distance))
            .unwrap();

        serde_wasm_bindgen::to_value(&val).unwrap()
    }

    pub fn nearest_euclidean(&mut self, point: Point, num: usize) -> JsValue {
        self.check_point(&point);
        let val: Vec<(PointUnit, &Data)> = self
            .kdtree
            .nearest(&point.clone(), num, &kdtree::distance::squared_euclidean)
            .unwrap();

        serde_wasm_bindgen::to_value(&val).unwrap()
    }

    pub fn add(&mut self, point: Point, data: Data) -> () {
        self.check_point(&point);

        self.kdtree.add(point.clone(), data).unwrap()
    }

    pub fn drop(self) {
        drop(self);
    }
}

fn js_fn_into_distance<A>(js_f: &js_sys::Function) -> impl Fn(&[A], &[A]) -> A + '_
where
    A: serde::ser::Serialize + serde::de::DeserializeOwned,
{
    |a: &[A], b: &[A]| {
        let this = JsValue::null();
        let jsa = serde_wasm_bindgen::to_value(a).unwrap();
        let jsb = serde_wasm_bindgen::to_value(b).unwrap();
        let res = js_f.call2(&this, &jsa, &jsb).unwrap();
        return serde_wasm_bindgen::from_value(res).unwrap();
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_must_use)]

    use super::KdTree;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn it_basic_test() {
        let mut kdt = KdTree::new(2);

        kdt.add(vec![1.0, 2.0], 3.0);
        kdt.add(vec![6.0, 9.0], 4.0);
        kdt.add(vec![3.0, 7.0], 5.0);
        kdt.add(vec![7.0, 4.0], 6.0);

        let f = js_sys::Function::new_with_args(
            "a, b",
            "return Math.pow(a[0] - b[0], 2) + Math.pow(a[1] - b[1], 2)",
        );

        let nearest =
            serde_wasm_bindgen::from_value::<Vec<(f64, f64)>>(kdt.nearest(vec![5.0, 1.0], 1, &f))
                .unwrap()[0];

        assert_eq!(nearest.0, 13.0f64);
        assert_eq!(nearest.1, 6.0f64);

        let nearest_euclidean = serde_wasm_bindgen::from_value::<Vec<(f64, f64)>>(
            kdt.nearest_euclidean(vec![5.0, 1.0], 1),
        )
        .unwrap()[0];

        assert_eq!(nearest_euclidean.0, 13.0f64);
        assert_eq!(nearest_euclidean.1, 6.0f64);
    }

    #[wasm_bindgen_test]
    #[should_panic(expected = "Point dimensions do not match KdTree dimensions")]
    fn it_checks_add_len() {
        let mut kdt = KdTree::new(1);

        kdt.add(vec![1.0, 2.0], 3.0);
    }

    #[wasm_bindgen_test]
    #[should_panic(expected = "Point dimensions do not match KdTree dimensions")]
    fn it_checks_nearest_len() {
        let mut kdt = KdTree::new(2);

        kdt.add(vec![1.0, 2.0], 3.0);

        let f = js_sys::Function::new_with_args(
            "a, b",
            "return Math.pow(a[0] - b[0], 2) + Math.pow(a[1] - b[1], 2)",
        );

        kdt.nearest(vec![5.0, 1.0, 2.0], 1, &f);
    }

    #[wasm_bindgen_test]
    #[should_panic(expected = "Point dimensions do not match KdTree dimensions")]
    fn it_checks_within_len() {
        let mut kdt = KdTree::new(2);

        kdt.add(vec![1.0, 2.0], 3.0);
        let f = js_sys::Function::new_with_args(
            "a, b",
            "return Math.pow(a[0] - b[0], 2) + Math.pow(a[1] - b[1], 2)",
        );

        kdt.within(vec![5.0, 1.0, 2.0], 1.0, &f);
    }

    #[wasm_bindgen_test]
    fn it_within() {
        let mut kdt = KdTree::new(2);

        kdt.add(vec![1.0, 2.0], 3.0);
        let f = js_sys::Function::new_with_args(
            "a, b",
            "return Math.pow(a[0] - b[0], 2) + Math.pow(a[1] - b[1], 2)",
        );

        let within =
            serde_wasm_bindgen::from_value::<Vec<(f64, f64)>>(kdt.within(vec![2.0, 1.0], 20.0, &f))
                .unwrap()[0];

        assert_eq!(within.0, 2.0);
        assert_eq!(within.1, 3.0);

        let within_euclidean = serde_wasm_bindgen::from_value::<Vec<(f64, f64)>>(
            kdt.within_euclidean(vec![2.0, 1.0], 20.0),
        )
        .unwrap()[0];

        assert_eq!(within_euclidean.0, 2.0);
        assert_eq!(within_euclidean.1, 3.0);
    }
}
