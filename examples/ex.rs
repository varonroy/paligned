use paligned::paligned;

#[allow(unused)]
#[derive(Debug)]
struct Foo {
    val: i32,
}

fn main() {
    let foo = Foo { val: 1 };

    paligned!([
        ["expression","value", "description"],
        ["1 + 1 =", 1 + 1],
        ["foo:", @debug foo, "`@debug` example"],
    ]);

    println!();

    println!("expression value  description");
    println!("1 + 1 = {}", 1 + 1);
    println!("foo:    {:?}", foo);
}
