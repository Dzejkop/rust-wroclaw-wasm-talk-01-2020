(module
  (func $speed_fib (param $n i32) (result i32)
    (local $a i32)
    (local $b i32)
    (local $tmp i32)

    i32.const 0
    local.set $a

    i32.const 1
    local.set $b

    block $B
      loop $L
        local.get $n
        i32.eqz
        br_if $B

        local.get $n
        i32.const -1
        i32.add
        local.set $n

        local.get $a
        local.get $b
        i32.add
        local.set $tmp

        local.get $b
        local.set $a

        local.get $tmp
        local.set $b

        br $L
      end
    end
    local.get $b
  )
  (export "fib" (func $speed_fib))
)
