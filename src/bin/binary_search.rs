fn main(){
    let my_vec = vec![1,22,3,4,5,6,7,8,9];
    let target = 7;
    let outs = binary_search(&my_vec, &target);
    println!("The result is {:?}", outs);

}


#[allow(unused_variables)]
fn binary_search(list:&Vec<i32>, target:&i32)-> Option<i32>{
    let target = *target;
    //Define index of first and last 
    //ex: [1,2,3,4] =>  ex[0] = 1 and ex[3]= 4

    let mut first = 0;
    let mut last = list.len() - 1;

    for (index,val) in list.iter().enumerate(){
        let midpoint = (first+last)/2;
        if list[midpoint] == target{
            return Some(midpoint as i32);
        }
        else if  list[midpoint] < target {
            first =  midpoint + 1;        
        }
        else if  list[midpoint] > target {
             last = midpoint -1;
            
        }
         

    }

    None


}