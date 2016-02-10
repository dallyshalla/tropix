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
use std::ptr::null;

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
pub struct OrderBookLine {
	pub Quantity: f64,
	pub Rate: f64,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct OrderBook {
	pub buy: Vec<OrderBookLine>,
	pub sell: Vec<OrderBookLine>,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Balances {
	pub Currency: String,
	pub Balance: f64,
	pub Available: f64,
	pub Pending: f64,
	pub CryptoAddress: String,
}

#[derive(RustcDecodable, RustcEncodable)]
struct OurTime {
	tv_sec: i64,
	tv_nsec: i64,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct TradeResponse {
	pub success: bool,
	pub message: String,
	pub result: Uuid,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Uuid {
	pub uuid: String,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct OpenOrder {
	pub Uuid: Option<String>,
	pub OrderUuid: String,
	pub Exchange: String,
	pub OrderType: String,
	pub Quantity: f64,
	pub QuantityRemaining: f64,
	pub Limit: f64,
	pub CommissionPaid: f64,
	pub Price: f64,
	pub PricePerUnit: Option<String>,
	pub Opened: String,
	pub Closed: Option<String>,
	pub CancelInitiated: bool,
	pub ImmediateOrCancel: bool,
	pub IsConditional: bool,
	pub Condition: String,
	pub ConditionTarget: Option<String>,
}

pub fn get_balances(apikey: &str, secretkey: &str) -> Vec<Balances> {
	let api_keystring = "apikey=".to_string() + apikey;
	let api_nonce = "&nonce=1";
	let parameters = "".to_string() + &api_keystring+ &api_nonce;
	let the_secret_bytes = secretkey.as_bytes();
	let begin_url = "https://bittrex.com/api/v1.1/account/getbalances?".to_string() + &parameters;
	let the_url_clone = begin_url.clone();
//hmac-sha512 signature of uri
	let the_sha = Sha512::new();
	let the_base_key = the_secret_bytes;
	let mut the_new_mac = crypto::hmac::Hmac::new(the_sha, the_base_key);
	the_new_mac.input(begin_url.as_bytes());
	let the_signature_string =  &the_new_mac.result().code().to_hex().to_string();

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
    the_balances
}


pub fn buy_market(apikey: &str, secretkey: &str, firstcoin: &str, secondcoin: &str, quantity: &str, rate: &str) -> String {

  //Buy
//secret Key
	let api_keystring = "apikey=".to_string() + apikey;
	let market_selection = "&market=".to_string() + firstcoin + &"-".to_string() + secondcoin;
	let market_quantity = "&quantity=".to_string() + quantity;
	let market_rate = "&rate=".to_string() + rate;
	let api_nonce = "&nonce=1";
	let parameters = "".to_string() + &api_keystring + &market_selection + &market_quantity + &market_rate + &api_nonce;
	let the_secret_bytes = secretkey.as_bytes();
//nonce
//uri
	//let the_uri = "https://bittrex.com/api/v1.1/market/buylimit?apikey=1e69a72fa1c942ca9628001d7b628d78&market=BTC-MAID&quantity=1000&rate=0.000056&nonce=10000".to_string();

	let the_begin_url = "https://bittrex.com/api/v1.1/market/buylimit?".to_string() + &parameters;
	let the_url_clone = the_begin_url.clone();
//hmac-sha512 signature of uri
	let mut the_sha = Sha512::new();
	let the_base_key = the_secret_bytes;
	let mut the_new_mac = crypto::hmac::Hmac::new(the_sha, the_base_key);
	the_new_mac.input(the_begin_url.as_bytes());
	//let the_digestive = Digest::input(the_digestive, &the_uri_bytes);
	let the_signature_string =  &the_new_mac.result().code().to_hex().to_string();
    let resp_orderbook = http::handle()
		.post(the_url_clone, &parameters)
		.header("apisign", &the_signature_string)
		.exec().unwrap();

	let mut us = String::from_utf8_lossy(resp_orderbook.get_body());
	let mut data = String::new();
    resp_orderbook.get_body().read_to_string(&mut data).unwrap();

    let json = Json::from_str(&data).unwrap();

    let thetrade_string: String = json::encode(&json).unwrap();
    thetrade_string

}

pub fn sell_market(apikey: &str, secretkey: &str, firstcoin: &str, secondcoin: &str, quantity: &str, rate: &str) -> String {

  //Buy
//secret Key
	let api_keystring = "apikey=".to_string() + apikey;
	let market_selection = "&market=".to_string() + firstcoin + &"-".to_string() + secondcoin;
	let market_quantity = "&quantity=".to_string() + quantity;
	let market_rate = "&rate=".to_string() + rate;
	let api_nonce = "&nonce=1";
	let parameters = "".to_string() + &api_keystring + &market_selection + &market_quantity + &market_rate + &api_nonce;
	let the_secret_bytes = secretkey.as_bytes();
//nonce
//uri
	//let the_uri = "https://bittrex.com/api/v1.1/market/buylimit?apikey=1e69a72fa1c942ca9628001d7b628d78&market=BTC-MAID&quantity=1000&rate=0.000056&nonce=10000".to_string();

	let the_begin_url = "https://bittrex.com/api/v1.1/market/selllimit?".to_string() + &parameters;
	let the_url_clone = the_begin_url.clone();
//hmac-sha512 signature of uri
	let mut the_sha = Sha512::new();
	let the_base_key = the_secret_bytes;
	let mut the_new_mac = crypto::hmac::Hmac::new(the_sha, the_base_key);
	the_new_mac.input(the_begin_url.as_bytes());
	//let the_digestive = Digest::input(the_digestive, &the_uri_bytes);
	let the_signature_string =  &the_new_mac.result().code().to_hex().to_string();
    let resp_orderbook = http::handle()
		.post(the_url_clone, &parameters)
		.header("apisign", &the_signature_string)
		.exec().unwrap();

	let mut us = String::from_utf8_lossy(resp_orderbook.get_body());
	let mut data = String::new();
    resp_orderbook.get_body().read_to_string(&mut data).unwrap();

    let json = Json::from_str(&data).unwrap();

    let thetrade_string: String = json::encode(&json).unwrap();
    thetrade_string

}


pub fn get_openorders(apikey: &str, secretkey: &str, firstcoin: &str, secondcoin: &str) -> Vec<OpenOrder> {

	let api_keystring = "apikey=".to_string() + apikey;
	let market_selection = "&market=".to_string() + firstcoin + &"-".to_string() + secondcoin;
	let api_nonce = "&nonce=1";
	let parameters = "".to_string() + &api_keystring + &market_selection + &api_nonce;
	let the_secret_bytes = secretkey.as_bytes();
//nonce

	let the_begin_url = "https://bittrex.com/api/v1.1/market/getopenorders?".to_string() + &parameters;
	let the_url_clone = the_begin_url.clone();
//uri
//hmac-sha512 signature of uri
	let mut the_sha = Sha512::new();
	let the_base_key = the_secret_bytes;
	let mut the_new_mac = crypto::hmac::Hmac::new(the_sha, the_base_key);
	the_new_mac.input(the_begin_url.as_bytes());
	//let the_digestive = Digest::input(the_digestive, &the_uri_bytes);
	let the_signature_string =  &the_new_mac.result().code().to_hex().to_string();
    let resp_openorders = http::handle()
		.post(the_url_clone, &parameters)
		.header("apisign", &the_signature_string)
		.exec().unwrap();

	let mut us = String::from_utf8_lossy(resp_openorders.get_body());
	let mut data = String::new();
    resp_openorders.get_body().read_to_string(&mut data).unwrap();

    let orderbook_json = Json::from_str(&data).unwrap();
    let orderbook_result = orderbook_json.find("result");
    let orderbook_result_string: String = json::encode(&orderbook_result).unwrap();
   

    let orderbook: Vec<OpenOrder> = json::decode(&orderbook_result_string).unwrap();

    orderbook
}


pub fn get_orderbook(secretkey: &str, firstcoin: &str, secondcoin: &str, depth: &str) -> OrderBook {

	let market_selection = "market=".to_string() + firstcoin + &"-".to_string() + secondcoin;
	let market_type = "&type=both".to_string();
	let market_depth = "&depth=".to_string() + depth;
	let parameters = "".to_string() + &market_selection + &market_type + &market_depth;
	let the_secret_bytes = secretkey.as_bytes();
//nonce

	let the_begin_url = "https://bittrex.com/api/v1.1/public/getorderbook?".to_string() + &parameters;
	let the_url_clone = the_begin_url.clone();
//uri
//hmac-sha512 signature of uri
	let mut the_sha = Sha512::new();
	let the_base_key = the_secret_bytes;
	let mut the_new_mac = crypto::hmac::Hmac::new(the_sha, the_base_key);
	the_new_mac.input(the_begin_url.as_bytes());
	//let the_digestive = Digest::input(the_digestive, &the_uri_bytes);
	let the_signature_string =  &the_new_mac.result().code().to_hex().to_string();
    let resp_openorders = http::handle()
		.post(the_url_clone, &parameters)
		.header("apisign", &the_signature_string)
		.exec().unwrap();

	let mut us = String::from_utf8_lossy(resp_openorders.get_body());
	let mut data = String::new();
    resp_openorders.get_body().read_to_string(&mut data).unwrap();

    let orderbook_json = Json::from_str(&data).unwrap();
    let orderbook_result = orderbook_json.find("result");
    let orderbook_result_string: String = json::encode(&orderbook_result).unwrap();

    let orderbook: OrderBook = json::decode(&orderbook_result_string).unwrap();
    orderbook
}






#[test]
fn it_works() {


}