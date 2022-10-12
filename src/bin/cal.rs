use electrum_client::{Client, ElectrumApi};
use electrum_client::Error;
use bitcoin::blockdata::block::BlockHeader;
fn main() {
    let jam = apis();
    match jam{
        Ok(val)=> println!("The succ is {:?}", val),
        Err(_) => println!("Zi its a error")
    }
    

   
}

fn sam() -> Result<String,Error>{

    let mut client = Client::new("tcp://electrum.blockstream.info:50001")?;
    let response = client.server_features()?;
    Ok("Success".to_string())

}


fn apis() -> Result<String,Error>{

    let mut client = Client::new("tcp://electrum.blockstream.info:50001")?;
    let response = client.block_headers_subscribe()?;    
    println!("The out is {:?}", response);
    Ok("Success".to_string())

}
