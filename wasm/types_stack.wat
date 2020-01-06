;; WASM supports only 4 types
;; i32, i64, f32 and f64

(module
    ;; values are put on the stack
    (func $f1 (param $a i32) (param $b i32) (result i32)
        ;; stack is empty
        get_local $a ;; value of $a is on the stuck [$a]
        get_local $b ;; [$a, $b]
        i32.add      ;; [$a + $b]
        ;; implicit return
    )
    (export "f1" (func $f1))
)
