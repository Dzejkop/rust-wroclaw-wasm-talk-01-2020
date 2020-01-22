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
    ;; WASM only has access to the stack and any defined memories
    ;; for now only one memory is allowed
    ;; access to the memory:
    ;; i32.load,
    ;; i32.load8_s - load sign-extended
    ;; i32.load8_u - load zero-extended
    ;; i32.store
    ;; i32.store8
    ;; i32.store16
    ;;
    ;; all instructions take an address value from the stack [address]
    ;; store instructions also take a value [addres, value]
    (memory $memory 1)
)
