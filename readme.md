## run

run in workspace

可以到相应目录下 `cargo run`

根目录下 packeage, bin, example 都可以运行

packeage 可以跨目录运行，bin, example 不可以跨目录

eg: feel/pest 下不能运行 `cargo run --bin feel-comfy-table`, `cargo run --example comfy_table`

### package

`cargo run -p feel-comfy-table`

### bin

`cargo run --bin feel-comfy-table`

### example

`cargo run --example comfy_table`
`cargo run --example hyphen-named`

## version

rust: 1.67.1
cargo: 1.67.1
rustup: 1.25.2

## epic

cargo init

### Keywords

```
KW_CONST : const
KW_LET : let
KW_FN : fn
KW_IF : if
KW_ELSE : else
KW_FOR : for
KW_BREAK : break
KW_CONTINUE : continue
KW_RETURN : return


KW_AS : as
KW_CRATE : crate
KW_ENUM : enum
KW_EXTERN : extern
KW_FALSE : false
KW_IMPL : impl
KW_IN : in
KW_LOOP : loop
KW_MATCH : match
KW_MOD : mod
KW_MOVE : move
KW_MUT : mut
KW_PUB : pub
KW_REF : ref
KW_SELFVALUE : self
KW_SELFTYPE : Self
KW_STATIC : static
KW_STRUCT : struct
KW_SUPER : super
KW_TRAIT : trait
KW_TRUE : true
KW_TYPE : type
KW_UNSAFE : unsafe
KW_USE : use
KW_WHERE : where
KW_WHILE : while
```
