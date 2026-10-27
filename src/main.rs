fn main() {
    println!("Hello, world!");//ah yes the sacred hello world
    let mut a; //just pure sheer aura of 'a' // mut cuz .. 
    //in rust variables are immutable by default (immutable means cannot be changed)
    a = 5 ; //assigned 
    a = a + 10;//if there was no mut here , then it would have given error
    println!("{a}");
    let b : i8;
    b = 67;

    let c = true;
    let d: bool = false;

    let e:i32= 6+9; 

//oh and the rest ,, they are just for random nonsense , ignore em
//not to mention you can do it for two vars too like this
// let (a, b); 
//(a, b) = (1,2);
//now you might be wondering why did i made them in comments 
// if i didnt , i would have to do a lil more work so yea 
// and now you might ask 'erm so why didnt you created another function?'
//its cuz i really dont want to make it 
//now you may be questioning 'but writing this takes energy too '
// i mean yea it does , but idk man ..

idk()
} 
fn idk(){
    //scope
    let (a, b) = (1, 2); // a = 1 and b = 2

let c = {
    let a = 4; // affects inside wrapping {} only
    let b = 6; // affects inside wrapping {} only

    a + b
}; // c = 10

let d = { a + b }; // d = 3

println!("{a} {b} {c} {d}"); // 1 2 10 3

consts()}

fn consts(){

}