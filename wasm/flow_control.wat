(module
  (func $speed_fib (param $n i32) (result i32)
    (local $a i32)
    (local $b i32)
    (local $tmp i32)

    i32.const 0
    set_local $a

    i32.const 1
    set_local $b

    block $B
      loop $L
        get_local $n
        i32.eqz
        br_if $B

        get_local $n
        i32.const -1
        i32.add
        set_local $n

        get_local $a
        get_local $b
        i32.add
        set_local $tmp

        get_local $b
        set_local $a

        get_local $tmp
        set_local $b

        br $L
      end
    end
    get_local $b
  )
  (export "fib" (func $speed_fib))
)
