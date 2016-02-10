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
   		let mut input = String::new();
   		let stdin = io::stdin();
   		stdin.lock().read_line(&mut input).unwrap();

    	if(input=="1\n") {
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
		else if(input=="0\n") {
    		loop_index = 0;
    		println!("goodbye");
		}
		else if(input=="3\n") {
		}
	}
	
}