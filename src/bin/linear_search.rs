fn main(){

    let my_vec = vec![1,22,3,4,5,6,7,8,9];
    let target = 70;
    let outs = linear_serach(&my_vec, &target);
    println!("The result is {:?}", outs);
    verify_result(outs);

}

fn linear_serach(list:&Vec<i32>, target:&i32)-> Option<i32>{
    
    for (index,val ) in list.iter().enumerate(){
        if val == target{
            return Some(index as i32);
        }

    }
   None
}


   fn verify_result(res:Option<i32>){
      match res{
        Some(val) => println!("The Index of the value is : {:?}", val),
        None => println!("No value found..Sorry")
      }
   }
