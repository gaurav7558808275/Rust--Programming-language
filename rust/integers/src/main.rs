fn main() {
    println!("we are talking aboput integers today!\n");
    // there are two types of data  : scalars and compound
    //scalar is divided into : integers, floating point numbers, booleans and characters.

    let mut x:i64 = 2147483647 ; 
    x+=1;
    println!("the value of x = {}",x);
    // float type data
   let y:f32 = 3.0  ;
    println!("the float is {}",y);

    // how to plat with tuples

    
    let bup: (i32,i64,i32) = (22,33,44);
    let(x,y,z) = bup;
    println!(" tupple is {}{}{}",x,y,z);
}
