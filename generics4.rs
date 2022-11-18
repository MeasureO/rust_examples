struct MyStruct<A, B> {
    a: A,
    b: B,
}

enum MyEnum<A, B> {
    A(A),
    B(B),
}

fn main() {
    let s = MyStruct { a: 10, b: "Hello" };

    // We have to specify the type of the `MyEnum::A` variant here because Rust does not have
    // information to infer it.
    let e = MyEnum::<i32, _>::B("Hello");
}
