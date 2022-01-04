// move_semantics1.rs
// Make me compile! Execute `rustlings hint move_semantics1` for hints :)



fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);//This must be mutable in order to complete the subsequent push()

    /*
    What is interesting about this example is there right here, you might expect to be able to:
    
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
    
    But you will get an error: vec0 borrowed after move
    Notice how we didn't borrow when calling fill_vec? vec0 owenership was moved to that fxn.
    */

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
