//@ edition: 2021

#![feature(return_type_notation)]
//~^ WARN the feature `return_type_notation` is incomplete

trait Trait {
    async fn method() {}
}

fn foo<T: Trait<method(i32): Send>>() {}
//~^ ERROR argument types not allowed with return type notation

fn bar<T: Trait<method() -> (): Send>>() {}
//~^ ERROR return type not allowed with return type notation

fn baz<T: Trait<method(): Send>>() {}
//~^ ERROR return type notation arguments must be elided with `..`

fn main() {}
