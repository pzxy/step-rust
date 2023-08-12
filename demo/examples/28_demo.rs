

fn main(){
    let mut result = [0; 32];
    let s = "aaaaaaaaaaaaaaaaaaaaaaaa".as_bytes();
    result[0..20].copy_from_slice(&s);
    println!("{:?}",result);
}