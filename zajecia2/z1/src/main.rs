fn main() {
    let mut a:  f32 = 125.0;

    fun1(&mut a);

    println!("temperatura: {a}")

}


fn fun1(temp_b: &mut f32) -> bool {

    *temp_b = 100.0/255.0 * *temp_b;
    
    return true;
}