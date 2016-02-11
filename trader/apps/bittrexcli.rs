extern crate bittrexlib;
extern crate rustc_serialize;

use bittrexlib::*;

use rustc_serialize::{Decodable, Decoder};
use rustc_serialize::json::{self, ToJson, Json};

use std::io;
use std::io::Read;
use std::thread::sleep;
use std::time::Duration;
use std::io::{BufRead};


fn main() {

	//Get Balances
//api Key

    println!("Enter Your Bittrex Api Key");
    let mut input1 = String::new();
    let stdin1 = io::stdin();
    stdin1.lock().read_line(&mut input1).unwrap();

    let next_string = &input1.trim_right_matches("\n");
    println!("{:?}", &next_string);
	
	let the_api_key = &next_string;
//secret Key
	println!("Enter Your Bittrex Secret Key");
    let mut input2 = String::new();
    let stdin2 = io::stdin();
    stdin2.lock().read_line(&mut input2).unwrap();

    let the_secret_trimmed = input2.trim_right_matches("\n");
    //let the_secret_bytes = the_secret_trimmed.as_bytes();
    let mut loop_index = 1;
	while loop_index != 0 {

   		println!("Welcome to Tropix - Bittrexcli Trader");
   		println!("Options available:");
   		println!("0: Exit");
   		println!("1: Check Balances");
   		println!("2: Check Existing Orders");
   		println!("3: Buy");
   		println!("4: Sell");
   		println!("5: Get Orderbook");
   		println!("6: Trade Automation");
   		let mut input = String::new();
   		let stdin = io::stdin();
   		stdin.lock().read_line(&mut input).unwrap();

		if input=="0\n" {
    		loop_index = 0;
    		println!("goodbye");
		}
    	else if input=="1\n" {
    		let the_balances = bittrexlib::get_balances(&the_api_key, &the_secret_trimmed);
    		for balance in the_balances {
    			println!("Currency {:?}", balance.Currency);
    			println!("Balance {:?}", balance.Balance);
    			println!("Available {:?}", balance.Available);
    			println!("Pending {:?}", balance.Pending);
    			println!("CryptoAddress {:?}", balance.CryptoAddress);
    		}
      		loop_index = 1;	
    	}
    	else if input=="2\n" {
			println!("Let us know which order book");
   			println!("Select first coin e.g. \"BTC\"");
    		let mut firstcoin = String::new();
    		let stdin1 = io::stdin();
    		stdin1.lock().read_line(&mut firstcoin).unwrap();
    		let firstcoin_trimmed = firstcoin.trim_right_matches("\n");
   			println!("Select second coin\"MAID\"");
    		let mut secondcoin = String::new();
    		let stdin2 = io::stdin();
    		stdin2.lock().read_line(&mut secondcoin).unwrap();
    		let secondcoin_trimmed = secondcoin.trim_right_matches("\n");

			let its_ts = get_openorders(&the_api_key, &the_secret_trimmed, &firstcoin_trimmed, &secondcoin_trimmed);
			let mut count_int = 0;
			for order in &its_ts {				
				println!("Index: {:?} Uuid: {:?} Price: {:?} Quantity: {:?} Side: {:?}", count_int, order.OrderUuid, order.Limit, order.Quantity, order.OrderType);
				count_int += 1;			
			}
			let mut new_loopindex = 1;
			while new_loopindex != 0 {
				println!("To cancel any of the above orders enter the index number and press enter or");
   				println!("type -1 to exit");
    			let mut input_response = String::new();
    			let stdin_cancel = io::stdin();
    			stdin_cancel.lock().read_line(&mut input_response).unwrap();
    			let inputresponse_trimmed = input_response.trim_right_matches("\n");

    			if inputresponse_trimmed == "-1".to_string() {
    				new_loopindex -= 1;
    			}
    			else {
    				let the_index: usize = inputresponse_trimmed.parse().ok().expect("invalid input");
    				let cancel_response = cancel_order(&the_api_key, &the_secret_trimmed, &its_ts[the_index].OrderUuid);
    				println!("{:?}", cancel_response);
    			}
			}
		}
		else if(input=="3\n") {
			println!("You're about to make a BUY choose two currencies, a quantity and a price.");
   			println!("Select first coin e.g. \"BTC\"");
    		let mut firstcoin = String::new();
    		let stdin1 = io::stdin();
    		stdin1.lock().read_line(&mut firstcoin).unwrap();
    		let firstcoin_trimmed = firstcoin.trim_right_matches("\n");
   			println!("Select second coin\"MAID\"");
    		let mut secondcoin = String::new();
    		let stdin2 = io::stdin();
    		stdin2.lock().read_line(&mut secondcoin).unwrap();
    		let secondcoin_trimmed = secondcoin.trim_right_matches("\n");
   			println!("Select a price and quantity, a few willing sellers:");
			let the_orderbook = get_orderbook(&the_secret_trimmed, &firstcoin_trimmed, &secondcoin_trimmed, "3");
			println!("Quantity: {:?}  Price: {:?}", the_orderbook.sell[0].Quantity, the_orderbook.sell[0].Rate);
			println!("Quantity: {:?}  Price: {:?}", the_orderbook.sell[1].Quantity, the_orderbook.sell[1].Rate);
			println!("Quantity: {:?}  Price: {:?}", the_orderbook.sell[2].Quantity, the_orderbook.sell[2].Rate);
   			println!("type in a quantity");
    		let mut quantity = String::new();
    		let stdin3 = io::stdin();
    		stdin3.lock().read_line(&mut quantity).unwrap();
    		let quantity_trimmed = quantity.trim_right_matches("\n");
    		println!("type in a rate");
    		let mut rate = String::new();
    		let stdin5 = io::stdin();
    		stdin5.lock().read_line(&mut rate).unwrap();
    		let rate_trimmed = rate.trim_right_matches("\n");
			let its_ts = buy_market(&the_api_key, &the_secret_trimmed, &firstcoin_trimmed, &secondcoin_trimmed, &quantity_trimmed, &rate_trimmed);
			println!("{:?}", its_ts);
		}
		else if input=="4\n" {
			println!("You're about to make a SELL choose two currencies, a quantity and a price.");
   			println!("Select first coin e.g. \"BTC\"");
    		let mut firstcoin = String::new();
    		let stdin1 = io::stdin();
    		stdin1.lock().read_line(&mut firstcoin).unwrap();
    		let firstcoin_trimmed = firstcoin.trim_right_matches("\n");
   			println!("Select second coin\"ETH\"");
    		let mut secondcoin = String::new();
    		let stdin2 = io::stdin();
    		stdin2.lock().read_line(&mut secondcoin).unwrap();
    		let secondcoin_trimmed = secondcoin.trim_right_matches("\n");
   			println!("Select a price and quantity, a few willing buyers:");
			let the_orderbook = get_orderbook(&the_secret_trimmed, &firstcoin_trimmed, &secondcoin_trimmed, "3");
			println!("Quantity: {:?}  Price: {:?}", the_orderbook.buy[0].Quantity, the_orderbook.buy[0].Rate);
			println!("Quantity: {:?}  Price: {:?}", the_orderbook.buy[1].Quantity, the_orderbook.buy[1].Rate);
			println!("Quantity: {:?}  Price: {:?}", the_orderbook.buy[2].Quantity, the_orderbook.buy[2].Rate);
   			println!("type in a quantity");
    		let mut quantity = String::new();
    		let stdin3 = io::stdin();
    		stdin3.lock().read_line(&mut quantity).unwrap();
    		let quantity_trimmed = quantity.trim_right_matches("\n");
    		println!("type in a rate");
    		let mut rate = String::new();
    		let stdin5 = io::stdin();
    		stdin5.lock().read_line(&mut rate).unwrap();
    		let rate_trimmed = rate.trim_right_matches("\n");
			let its_ts = sell_market(&the_api_key, &the_secret_trimmed, &firstcoin_trimmed, &secondcoin_trimmed, &quantity_trimmed, &rate_trimmed);
			println!("{:?}", its_ts);
		}
		else if input=="5\n" {
			println!("Let us know which order book");
   			println!("Select first coin e.g. \"BTC\"");
    		let mut firstcoin = String::new();
    		let stdin1 = io::stdin();
    		stdin1.lock().read_line(&mut firstcoin).unwrap();
    		let firstcoin_trimmed = firstcoin.trim_right_matches("\n");
   			println!("Select second coin\"MAID\"");
    		let mut secondcoin = String::new();
    		let stdin2 = io::stdin();
    		stdin2.lock().read_line(&mut secondcoin).unwrap();
    		let secondcoin_trimmed = secondcoin.trim_right_matches("\n");
   			println!("Select how many records you'd like to see");
    		let mut depth = String::new();
    		let stdin3 = io::stdin();
    		stdin3.lock().read_line(&mut depth).unwrap();
    		let depth_trimmed = depth.trim_right_matches("\n");
			let its_ts = get_orderbook(&the_secret_trimmed, &firstcoin_trimmed, &secondcoin_trimmed, &depth_trimmed);

			for buy in its_ts.buy {
    			println!("BUY Quantity: {:?}                              Rate: {:?}", buy.Quantity ,buy.Rate);
    		}
    		for sell in its_ts.sell {
    			println!("SELL Quantity: {:?}                              Rate: {:?}", sell.Quantity ,sell.Rate);
    		}
		}
		else if input == "6\n" {
			println!("You are about to activate automated trade");
			println!("Once the parameters are set the robot will continue to trade unless something interferes somehow");
			println!("If you choose to trade long, you will need the currency of the firstcoin");
			println!("If you choose to trade short, you will need the currency of the secondcoin");
   			println!("Select first coin e.g. \"BTC\"");
   			println!("Select second coin e.g. \"MAID\"");
			println!("Choose a direction:  enter 1 for Long, enter 2 for Short");
			println!("Choose a percent discount: the price below the current market price you want to buy for e.g. 3 \
				an entry of 3 will mean that the robot will buy any orders available -3% and below the price from when the bot was started");
			println!("Choose a percentage for profit taking in other words, if the price is \
				greater than the buy price by 10% start selling it off and vice versa");
			println!("Choose a percentage of the account balance to use e.g. 50");
			println!("Choose a time interval in minutes, how frequently do you want to make trades? e.g. 30 \
				an entry for 30 will mean that only one trade every 30 minutes will happen");


			let firstcoin = "";
			let secondcoin = "";
			let direction = 0;
			let discount = 0;
			let profit = 0;
			let interval = 0;

		}
	}	
}