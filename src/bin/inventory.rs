fn main(){
    println!("Hey Inventory");
    let company_inventory = Inventory{
        shirts: vec![ShirtColor::Red,ShirtColor::Red, ShirtColor::Green]
    };

// first call with a choice
  let first = Some(ShirtColor::Red);

// second call with no choice
 let second  = None;
    let sam = company_inventory.give_away(second);

    println!("The choice found {:?}", sam);


}


struct Inventory{
    shirts : Vec<ShirtColor>
}

impl Inventory{
    fn give_away(&self, user_pref: Option<ShirtColor>)-> ShirtColor{

        user_pref.unwrap_or_else(|| self.call_me())
        
    }



    fn call_me(&self)-> ShirtColor{
        println!("Hey u called mee..");
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Green => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Green
        }
    }
}


#[derive(Debug)]
enum ShirtColor{
     Red,
     Green,
}