extern crate bittrexlib;
extern crate rustc_serialize;
extern crate time;
extern crate crypto;

extern crate curl;

use curl::http;

use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::digest::Digest;
use crypto::sha2::Sha512;

use std::io;
use std::io::Read;
use std::thread::sleep;
use std::time::Duration;
use std::io::{BufRead};

use rustc_serialize::hex::ToHex;
use rustc_serialize::{Decodable, Decoder};
use rustc_serialize::json::{self, ToJson, Json};


#[derive(RustcDecodable, RustcEncodable)]
struct Response {
	success: bool,
	message: String,
	result: Vec<Balances>,
}

#[derive(RustcDecodable, RustcEncodable)]
struct OrderBookLine {
	Quantity: f64,
	Rate: f64,
}

#[derive(RustcDecodable, RustcEncodable)]
struct OrderBook {
	buy: Vec<OrderBookLine>,
	sell: Vec<OrderBookLine>,
}

#[derive(RustcDecodable, RustcEncodable)]
struct Balances {
	Currency: String,
	Balance: f64,
	Available: f64,
	Pending: f64,
	CryptoAddress: String,
}

#[derive(RustcDecodable, RustcEncodable)]
struct OurTime {
	tv_sec: i64,
	tv_nsec: i64,
}
use time::SteadyTime;
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
    let the_secret_bytes = the_secret_trimmed.as_bytes();

//nonce
	let api_keystring = "apikey=".to_string() + &the_api_key;
	println!("{:?}", &api_keystring);
	let api_nonce = "&nonce=1";
	let parameters = "".to_string() + &api_keystring+ &api_nonce;

	let begin_url = "https://bittrex.com/api/v1.1/account/getbalances?".to_string() + &parameters;
	let the_url_clone = begin_url.clone();
//hmac-sha512 signature of uri
	let the_sha = Sha512::new();
	let the_base_key = the_secret_bytes;
	let mut the_new_mac = crypto::hmac::Hmac::new(the_sha, the_base_key);
	the_new_mac.input(begin_url.as_bytes());
	let the_signature_string =  &the_new_mac.result().code().to_hex().to_string();
//initiate uri

//send out ur along with the signature of uri
	

	//Get Balances
	let resp = http::handle()
		.post(the_url_clone, &parameters)
		.header("apisign", &the_signature_string)
		.exec().unwrap();
	
	let us = String::from_utf8_lossy(resp.get_body());
	let mut data = String::new();
    resp.get_body().read_to_string(&mut data).unwrap();

    let json = Json::from_str(&data).unwrap();
    let this_part = json.find("result");
    let this_data: String = json::encode(&this_part).unwrap();

    let the_balances: Vec<Balances> = json::decode(&this_data).unwrap();

    for balance in the_balances {
    	println!("Currency {:?}", balance.Currency);
    	println!("Balance {:?}", balance.Balance);
    	println!("Available {:?}", balance.Available);
    	println!("Pending {:?}", balance.Pending);
    	println!("CryptoAddress {:?}", balance.CryptoAddress);
    }
}