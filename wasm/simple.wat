;; a module is a collection of...
(module
  ;; types of functions
  (type $t1 (func (param i32) (result i32)))
  (type (func (param f32) (result f32)))

  ;; imports
  (import "env" "external_func" (func $external_func (param i32)))

  ;; globals
  (global $n i32 (i32.const 1))

  ;; memory definitions (currently only 1 is allowed)
  (memory $memory (export "memory") 1)

  ;; functions
  (func $add_one (type $t1) (param $v i32) (result i32)
    get_local $v
    i32.const 1
    i32.add
  )
  (func $add_one_float (type 1) (param $v f32) (result f32) ;; type referenced by an index
    get_local $v
    f32.const 1
    f32.add
  )

  ;; exports
  (export "add_one" (func $add_one))
  (export "float_add_one" (func 1)) ;; <- function referenced by index
  (export "n" (global $n)) ;; globals can also be exported if not mutable

  ;; data sections - to populate memory
  (data $memory (i32.const 0) "Hello")

  ;; additionally:
  ;; tables for function references
  ;; elems to populate the tables
  ;; and an optional start section
  ;; which we will not cover
)

;; comments start with ";;" if you haven't noticed ;)
