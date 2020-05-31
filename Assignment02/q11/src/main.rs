//This program will calculate profit and loss on a transaction. 
 
fn main(){
    let cost_price = 12;
    let sale_price = 13;
    let mut profit_lost = 0;
    if sale_price>cost_price {    //calculate profit
        profit_lost = sale_price-cost_price;
        println!("You can booked your profit amount : {}", profit_lost);
    } else if(cost_price>sale_price){   //calculate loss
          profit_lost = cost_price-sale_price;          
          println!("You got a loss of amount : {}", profit_lost);
    } else{     //No Profit No Loss     
          println!("You are running in no profit no loss condition.");
    }  
}
