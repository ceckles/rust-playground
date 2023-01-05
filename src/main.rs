fn main() {
    // Hello World
    println!("Hello, world!");

    //Variables default implicit and immutable types
    let x = 4;
    //x = "hello" // No go!
    println!("x is: {}", x);
    //uncomment for error message.
    //x=5; //No Go
    //println!("x is now : {}",x);

    // recreate var or add mut to make mutateable
    // remove mut to proof below
    let mut y = 7; //Go
    println!("y is : {}", y);
    //let y = 6; //Go
    y = 6;
    println!("y is now : {}", y);

    //increment
    let z = 6;
    println!("Z is : {}", z);
    let z = z + 1;
    println!("Z is now : {}", z);

    //Scope variable name shadowing
    {
        let x = 2;
        println!("scope x is : {}", x);
        //can also call parent variables within a scope
        let z = z - 2;
        println!("scope z is : {}",z);

        //can also change type when redeclareing but, not if it is a mut
        let x = "Hello World!";
        println!("Message of the day: {}", x);
    }

    //constants can not change once its defined
    const SECONDS_IN_MINUTES : u32 = 60;
    //SECONDS_IN_MINUTES = 100; //no go
    //const SECONDS_IN_MINUTES : u32 = 60; //no go
    println!("Seconds in a Min : {}",SECONDS_IN_MINUTES); 
}
