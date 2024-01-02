fn main() {
    // immutable var by default
    let x: u32 = 4;
    println!("x = {}", x);

    // Error!
    // x = 5
    //  println!("x = {}", x)

    // mutable var: redifine x as mutable
    let mut y: u32 = 5;
    println!("mut y = {}", y);
    y = 55;
    println!("mut y = {}", y);

    // warning on unused vars and unnecessary mutable vars

    println!("\nredifine vars in order to avoid mutable vars");
    let x = 1;
    println!("x = {}", x);
    let x = x + 1;
    println!("x = {}", x);

    // create scope
    {
        let x = x - 2;
        println!("inside scope with var from outside scope");
        println!("x = {}", x);
    }

    println!("outside scope");
    println!("x = {}", x);

    println!("redifine x as string");
    let x = "Mario";
    println!("x = {}", x);

    // not possible to change type to a mutable var
    // let mut x = 4
    // x = "eeee"

    // constants: type is mandatory, not inferred
    const SECONDS_IN_MINUTE: u32 = 60;
    println!("Seconds in a minute: {}", SECONDS_IN_MINUTE);

    // constants cannot be redefined
    // const SECONDS_IN_MINUTE = 100 => error!
}
