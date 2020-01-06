(module
  (memory $memory (export "memory") 1)
  (func $fib (param $v i32) (result i32)
    block $B
      get_local $v ;; [$v]
      i32.const 2  ;; [$v, 2]
      i32.lt_s     ;; Is $v less than 2
      br_if $B     ;; break out of the loop

      get_local $v ;; [$v]
      i32.const -1 ;; [$v, -1]
      i32.add      ;; [$v - 1]
      call $fib    ;; call $fib recursively, result pushed on stack

      get_local $v ;; [result, $v]
      i32.const -2 ;; [result, $v, -2]
      i32.add      ;; [result, $v - 2]
      call $fib    ;; call $fib recursively, result pushed on stack

      i32.add      ;; [r1 + r2]
      return
    end
    i32.const 1    ;; fib(1) == fib(2) == 1
  )
  (export "fib" (func $fib))
)
