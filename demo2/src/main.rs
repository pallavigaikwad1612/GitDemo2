fn main() {
    let vec = vec![1,2,3,4,5];
    println!("My vec is :{:?}",vec);
    println!("Hello there, world!");

    for i in 1..10{
        println!("Current iterator is : {}",i);
    }

    let fn_result = addnum(20, 45);
    println!("Sum result is :{}", fn_result);
}

fn addnum(a:i32 , b:i32)-> i32{
    a + b
}