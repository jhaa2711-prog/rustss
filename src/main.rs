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

consts()
}

fn consts(){
//ConstantsThe const keyword is used to define constants and after the assignment their values are not allowed to change. They live for the entire lifetime of a program but has no fixed address in the memory.
//basically and on the biggest picture on the most abstracted , and just like lolipops and sunshines
//its immutable ...
//connst ohh shit not connst
const number : i8 = 5;
//i8 cuz its really very low number an i didnt wanted to waste any resources even if its  just some 100kbs of ram usage
//if you are thinking i was correct in the prev comment .. THEN YOU ARE WRONG , YOU ARE NOT PAYING ATTENTION
//CONSTANTS DONT GET IN RAM , AS WRITTEN IN THE FIRST COMMENT 
// -9000 POINTS FOR THIS ONE
//also if youu didnt fall for it .. congrats now you are above 0.01% of people :)
//till now if you had followed this , you should have got 8 warnings ... but dont worrry those arent gonna break the code , its just cause you were lazy and made variables declare without using them , YES YOU WERE LAZY
println!("number = {number}") ;
This_is_static_not_friction();
}

fn This_is_static_not_friction(){//you shouldnt be using these type of names , its less professional 
    //dont mind the name of the function
    //static is the global variable .. like THE GLOBAL VARIABLE
    //which means you can snatch (not snatch DONT DO THAT OR YOU WILL GET LIFETIME ERRORS)
    //i mean use them , in different functions 
    //lowkey .. i feel like you dont understand what i am telling , so here is a well formatted explaination which i thought while i was shitting in corner to assert dominance 
//The static keyword is used to define a "global variable" type facility. There is only one instance for each value, and it’s at a fixed location in memory.
    static N: i64 = 9999;
    println!("hi this is n , n introduce your friend group {N}");//this semicolon is after that comment btw
// who in the dumbass wrote this code ,
// even forgot to call those two functions

//one question please ... if you like , ii mean from the bottom of your heart YOU GENUINELY FIND 
//EATING CABBAGES INTERESTING , YOU REALLY ARE A DIFFERENT BREED (NOT MY OPINION BTW , OR IS IT?)

lowkey_idk_iAmBeingLazy();
}
fn lowkey_idk_iAmBeingLazy(){
//so have you watched jjk ? just asking
// what type of women do you like?
//do you believe things are inevitable
//oh sorry , if you were copying till now you would have got an error on line 71 , so just put a semi colon there
// thanks 
//today kids , or big kids? 
//VARiable for(shadowing)
//its just shadowing , the (for) was for dramatic effects
// how do i explain it ... wait 
// lemme shit in the top to assert 90x dominance
// oh no .. i cant 
// well nevermidn i hope you didnt read that
// i believe that you have low attention span and you missed what i said
// and thats good
VAR_shadow();
}
fn VAR_shadow(){
    // so think like , theres one box(unit) , so soemtimes we need to
    //transform that to a dfferent box for further processsig(like in food industry or other foods)
    //so instead of using different ariable names (naming is hard , trust)
    //RUST(racoon united sanity test) allows us to re-declare the same variable with
    //different data type and with different mutability set-ttin--ngs
    //and thhis is shadpowunggqidutw68tdgagqt863r7quabdtr38qabd8q3r6w7aysbdn83qe7wyasbdnz
    //elphant ran over ma keyboard

    let s: &str = "hello"; // &str
let s: String = s.to_uppercase(); // String , but as ya can see the s is "hello" changing into uppercase
println!("{s}"); // HELLO
//also i cant explain the difference between &str and String rn 
// cuz i am laz- no , cuz ummm ..,.... i need to assert dominance
//https://share.google/aimode/uIqnNFPSfQCNHEt5R
//there it is
}
//a quick nerdy check 
// use const and static(not recomhwudgagwyfedded) at the top of the file ..
//to establish dominance 
// jk 
//👨‍🏫 Before going to the next...
//Usually, constants and statics are placed at the top of the code file, outside the functions (after module imports/ use declarations).
//also thats it for today guys, next explanation tomororrww
// the time flies if you let it fly
//you sleep bcz you sleep
//~sun tzu , the art of something
//lol i lowkey was trying to copy all with ctrl + a , but kept hitting ctrl + s , 
//fuunyy yea ? 
// yeah?
// IT IS FUNNY OR OTHERWISE 
//....
//....
//NOTHING , WHAT CAN I DO , ? I WONT EVEN KNOW 
//IF YOU THINK IT IS WHAT I WOULD SAY THEN YOU ARE WRONG
// THATS WHAT LOSERS THINK
// MY WPM IS ABOVE 50 
// I WILL COME IN YOUR DREAMS 
// AND HAUNT YOU 
// IF YOU DONT FIND IT FUNNNY 