extern crate bittrexlib;
extern crate rustc_serialize;
extern crate time;

use bittrexlib::*;

use rustc_serialize::{Decodable, Decoder};
use rustc_serialize::json::{self, ToJson, Json};

use std::io;
use std::io::Read;
use std::thread::sleep;
use std::time::Duration;
use std::io::{BufRead};

#[derive(Clone, Copy, RustcDecodable, RustcEncodable)]
struct BotTrade {
    price: f64,
    quantity: f64,
    target: f64,
}

impl BotTrade {
	fn alter_quantity(&mut self, quantity: f64) {
		self.quantity = quantity;
	}
}

fn main() {

	//Get Balances
//api Key

    println!("Enter Your Bittrex Api Key");
    let mut input1 = String::new();
    let stdin1 = io::stdin();
    stdin1.lock().read_line(&mut input1).unwrap();

    let next_string = &input1.trim_right_matches("\n");
	
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

		if input == "0\n" {
    		loop_index = 0;
    		println!("goodbye");
		}
    	else if input == "1\n" {
    		let the_balances = get_balances(&the_api_key, &the_secret_trimmed);
    		for balance in the_balances {
    			println!("Currency {:?}", balance.Currency);
    			println!("Balance {:?}", balance.Balance);
    			println!("Available {:?}", balance.Available);
    			println!("Pending {:?}", balance.Pending);
    			println!("CryptoAddress {:?}", balance.CryptoAddress);
    		}
      		loop_index = 1;	
    	}
    	else if input == "2\n" {
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
		else if input == "3\n" {
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

   			println!("Select a price and quantity, displayed are a few willing sellers:");
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

			let method_response = buy_market(&the_api_key, &the_secret_trimmed, &firstcoin_trimmed, &secondcoin_trimmed, &quantity_trimmed, &rate_trimmed);
			
			println!("{:?}", method_response);
		}
		else if input == "4\n" {
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

   			println!("Select a price and quantity, displayed are a few willing buyers:");
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
			let method_response = sell_market(&the_api_key, &the_secret_trimmed, &firstcoin_trimmed, &secondcoin_trimmed, &quantity_trimmed, &rate_trimmed);
			println!("{:?}", method_response);
		}
		else if input == "5\n" {
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
   			println!("Select how many records you'd like to see e.g. 50");
    		let mut depth = String::new();
    		let stdin3 = io::stdin();
    		stdin3.lock().read_line(&mut depth).unwrap();
    		let depth_trimmed = depth.trim_right_matches("\n");
			let order_book = get_orderbook(&the_secret_trimmed, &firstcoin_trimmed, &secondcoin_trimmed, &depth_trimmed);

			for buy in order_book.buy {
    			println!("BUY Quantity: {:?}                              Rate: {:?}", buy.Quantity ,buy.Rate);
    		}
    		for sell in order_book.sell {
    			println!("SELL Quantity: {:?}                              Rate: {:?}", sell.Quantity ,sell.Rate);
    		}
		}
		else if input == "6\n" {
			println!("You are about to activate automated trade");
			println!("Once the parameters are set the robot will continue to trade unless something interferes somehow");
			println!("If you choose to trade long, you will need the currency of the firstcoin");
			println!("If you choose to trade short, you will need the currency of the secondcoin");

   			println!("Select first coin e.g. \"BTC\"");
    		let mut firstcoin = String::new();
    		let stdin1 = io::stdin();
    		stdin1.lock().read_line(&mut firstcoin).unwrap();

    		let firstcoin_trimmed = firstcoin.trim_right_matches("\n");

   			println!("Select second coin e.g. \"MAID\"");
    		let mut secondcoin = String::new();
    		let stdin2 = io::stdin();
    		stdin2.lock().read_line(&mut secondcoin).unwrap();

    		let secondcoin_trimmed = secondcoin.trim_right_matches("\n");

			println!("Choose a direction:  enter 1 for Long, enter 2 for Short");
    		let mut direction = String::new();
    		let stdin3 = io::stdin();
    		stdin3.lock().read_line(&mut direction).unwrap();

    		let direction_trimmed = direction.trim_right_matches("\n");

    		let direction_parsed: i32 = direction_trimmed.parse().ok().expect("invalid input");

			println!("Choose a percent discount: the price below the current market price you want to buy for e.g. 3 \
				an entry of 3 will mean that the robot will buy any orders available -3% and below the price from when the bot was started");
			let mut discount = String::new();
    		let stdin4 = io::stdin();
    		stdin4.lock().read_line(&mut discount).unwrap();

    		let discount_trimmed = discount.trim_right_matches("\n");

    		let discount_parsed: f64 = discount_trimmed.parse().ok().expect("invalid input");

			println!("Choose a percentage for profit taking in other words, if the price is \
				greater than the buy price by 10% start selling it off and vice versa");
			let mut profit = String::new();
    		let stdin5 = io::stdin();
    		stdin5.lock().read_line(&mut profit).unwrap();

    		let profit_trimmed = profit.trim_right_matches("\n");

    		let profit_parsed: f64 = profit_trimmed.parse().ok().expect("invalid input");

			println!("Choose a percentage of the account balance to use e.g. 50");
			let mut balance = String::new();
    		let stdin6 = io::stdin();
    		stdin6.lock().read_line(&mut balance).unwrap();

    		let balance_trimmed = balance.trim_right_matches("\n");

    		let balance_parsed: f64 = balance_trimmed.parse().ok().expect("invalid input");

			println!("Choose a time interval in minutes, how frequently do you want to make trades? e.g. 30 \
				an entry for 30 will mean that each 30 minutes the automator will reload with more coins to trade\
				the key is to not miss future opportunities by trading everything at once\
				with an entry of 30 one max quantity will trade every 30 minutes");
			let mut interval = String::new();
    		let stdin7 = io::stdin();
    		stdin7.lock().read_line(&mut interval).unwrap();

    		let interval_trimmed = interval.trim_right_matches("\n");

    		let interval_parsed: i32 = interval_trimmed.parse().ok().expect("invalid input");

    		//check if long or sort then pick the currency to compare and get the balance of.
    		//1

    		let mut twentyhour_price: f64 = 0.00;
    		let mut target_price: f64 = 0.00;
    		let mut target_price_multiplier: f64 = 0.00;
    		let the_balances = get_balances(&the_api_key, &the_secret_trimmed);
    		let mut max_trade_amount = 0.00;
    		if direction_parsed == 1 {
    			for balance in the_balances {
    				if balance.Currency == firstcoin_trimmed {
    					let balance_percent: f64 = 100.00 / balance_parsed;
    					let usable_balance: f64 = balance.Available * balance_percent;
    					let divided_balance: f64 = usable_balance / 50.0;
    					println!("trade size will be: {:?}", divided_balance);
    					max_trade_amount = divided_balance;

    					let ticker_string = get_ticker(&firstcoin_trimmed, &secondcoin_trimmed);
    					twentyhour_price = ticker_string.Last;

    					target_price_multiplier = 100.00 - discount_parsed;
    					target_price = twentyhour_price * target_price_multiplier;
    				}
    			}
    		} 
    		else if direction_parsed == 2 {
    			for balance in the_balances {
    				if balance.Currency == secondcoin_trimmed {
    					let balance_percent: f64 = 100.00 / balance_parsed;
    					let usable_balance: f64 = balance.Available * balance_percent;
    					let divided_balance: f64 = usable_balance / 50.0;
    					println!("trade size will be: {:?}", divided_balance);
    					max_trade_amount = divided_balance;

    					let ticker_string = get_ticker(&firstcoin_trimmed, &secondcoin_trimmed);
    					twentyhour_price = ticker_string.Last;

    					target_price_multiplier = 100.00 + discount_parsed;
    					target_price = twentyhour_price * target_price_multiplier;
    				}
    			}
    		}


    		//2

    		let trade_interval = interval_parsed * 10;
    		let mut keep_daily_timing = 0;
    		let mut total_traded = 0.00;
    		let mut int_indexing = 0;
    		let mut keep_timing = 0;

    		let mut minimum_tradesize = 0.00;

    		let mut bots_trades: Vec<BotTrade> = Vec::new();
    		let mut bots_clone1 = bots_trades.clone();
    		let mut bots_clone2 = bots_trades.clone();
    		let mut bots_clone3 = bots_trades.clone();
    		let mut bots_clone4 = bots_trades.clone();
    		let mut bots_clone5 = bots_trades.clone();
    		let mut bots_clone6 = bots_trades.clone();
    		let mut bots_clone7 = bots_trades.clone();
    		let mut bots_clone8 = bots_trades.clone();
    		let mut bots_clone9 = bots_trades.clone();
    		let mut bots_clone10 = bots_trades.clone();

    		let mut outside_quant = Vec::new();
    		let mut count_quant = Vec::new();

    		let mut total_profit = 0.00;
    		while int_indexing != 1 {

    			keep_timing += 1;
    			keep_daily_timing += 1;


    				//two things, track the trades made and get profit taking
    				//the other thing is figure out for calculating the minimum trade size
    			//3
    			if total_traded < max_trade_amount {
    				let mut available_trade = max_trade_amount - total_traded;

    				if direction_parsed == 1 {
    					if bots_clone5.len() > 0 {
    						let mut profit_index = 0;
    						for trade in &bots_trades {
    							if count_quant[profit_index] == 0 {
    								count_quant[profit_index] = 1;
    								outside_quant[profit_index] = trade.quantity;
    							}
    							let order_book = get_orderbook(&the_secret_trimmed, &firstcoin_trimmed, &secondcoin_trimmed, "500");
    							for buy in order_book.buy {
    								if buy.Rate >= trade.target {
    									if buy.Quantity < outside_quant[profit_index] {
    										let the_trade_quantity: f64 = outside_quant[profit_index] - buy.Quantity;
    										let xy: f64 = (the_trade_quantity * 100000000.00).round() / 100000000.00;
    										outside_quant[profit_index] -= the_trade_quantity;
    										let the_trade = sell_market(&the_api_key, &the_secret_trimmed, &firstcoin_trimmed, &secondcoin_trimmed, &xy.to_string(), &buy.Rate.to_string());
    										total_profit += (trade.target - trade.price) * xy;
    										println!("total profit {:?}", total_profit);
    									}
    									else {
    										let the_trade = sell_market(&the_api_key, &the_secret_trimmed, &firstcoin_trimmed, &secondcoin_trimmed, &trade.quantity.to_string(), &buy.Rate.to_string());
    										total_profit += (trade.target - trade.price) * trade.quantity;
    										outside_quant[profit_index] = 0.00;
    										println!("total profit {:?}", total_profit);
    									}
    									
    								}
    							}
    							profit_index += 1;
    						}
    					}
    					let order_book = get_orderbook(&the_secret_trimmed, &firstcoin_trimmed, &secondcoin_trimmed, "500");
    					
    					for sell in order_book.sell {
    						available_trade = max_trade_amount - total_traded;
    						if (sell.Rate < target_price) & (available_trade > 0.00056) {

    							if sell.Quantity > available_trade {
    								let the_longrate = available_trade / sell.Rate;
    								let y = (the_longrate * 100000000.00).round() / 100000000.00;
    								let the_trade = buy_market(&the_api_key, &the_secret_trimmed, &firstcoin_trimmed, &secondcoin_trimmed, &y.to_string(), &sell.Rate.to_string());
    								total_traded += available_trade;
    								let profit_multiplier = profit_parsed / 100.00;
    								let profit_multiplier_next = profit_multiplier + 1.00;
    								let the_targetrate = sell.Rate * profit_multiplier_next;
    								let xy = (the_targetrate * 100000000.00).round() / 100000000.00;

    								let the_botstrade: BotTrade = BotTrade {
    									price: sell.Rate,
    									quantity: y,
    									target: xy,
    								};
    								bots_clone1.push(the_botstrade);

    										println!("TRADE MADE {:?}", the_trade);
    										println!("TRADE MADE {:?}", the_botstrade.quantity);

    							}
    							else {
    								let the_adjusted_quantity = available_trade - sell.Quantity;
    								let the_longrate = the_adjusted_quantity / sell.Rate;
    								let y = (the_longrate * 100000000.00).round() / 100000000.00;

    								let the_trade = buy_market(&the_api_key, &the_secret_trimmed, &firstcoin_trimmed, &secondcoin_trimmed, &y.to_string(), &sell.Rate.to_string());
    								total_traded += the_adjusted_quantity;
    								let profit_multiplier = profit_parsed / 100.00;
    								let profit_multiplier_next = profit_multiplier + 1.00;
    								let the_targetrate = sell.Rate * profit_multiplier_next;
    								let xy = (the_targetrate * 100000000.00).round() / 100000000.00;

    								let the_botstrade: BotTrade = BotTrade {
    									price: sell.Rate,
    									quantity: y,
    									target: xy,
    								};
    								bots_clone2.push(the_botstrade);
    										println!("TRADE MADE {:?}", the_trade);
    										println!("TRADE MADE {:?}", the_botstrade.quantity);
    							}

    						}
    					}
    				}
    				else if direction_parsed == 2 {
    					if bots_clone6.len() > 0 {
    						let mut profit_index = 0;
    						for trade in &bots_trades {
    							if count_quant[profit_index] == 0 {
    								count_quant[profit_index] = 1;
    								outside_quant[profit_index] = trade.quantity;
    							}
    							let order_book = get_orderbook(&the_secret_trimmed, &firstcoin_trimmed, &secondcoin_trimmed, "500");
    							for sell in order_book.sell {
    								if sell.Rate <= trade.target {
    									if sell.Quantity < outside_quant[profit_index] {
    										let the_trade_quantity = outside_quant[profit_index] - sell.Quantity;
    										let xy = (the_trade_quantity * 100000000.00).round() / 100000000.00;
    										outside_quant[profit_index] -= the_trade_quantity;
    										let the_trade = buy_market(&the_api_key, &the_secret_trimmed, &firstcoin_trimmed, &secondcoin_trimmed, &xy.to_string(), &sell.Rate.to_string());
    										total_profit += (trade.price - trade.target) * xy;

    										println!("total profit {:?}", total_profit);
    									}
    									else {
    										let the_trade = buy_market(&the_api_key, &the_secret_trimmed, &firstcoin_trimmed, &secondcoin_trimmed, &trade.quantity.to_string(), &sell.Rate.to_string());
    										total_profit += (trade.price - trade.target) * trade.quantity;
    										outside_quant[profit_index] = 0.00;
    										println!("total profit {:?}", total_profit);
    									}
    									
    								}
    							}
    							profit_index += 1;
    						}
    					}


    					let order_book = get_orderbook(&the_secret_trimmed, &firstcoin_trimmed, &secondcoin_trimmed, "500");
    					let temp_ticker = get_ticker(&firstcoin_trimmed, &secondcoin_trimmed);
    					minimum_tradesize = 0.00056 / temp_ticker.Last;
    					for buy in order_book.buy {
    						available_trade = max_trade_amount - total_traded;
    						if (buy.Rate > target_price) & (available_trade > minimum_tradesize) {
    							if buy.Quantity > available_trade {
    								let y = (available_trade * 10000.00).round() / 10000.00;
    								let the_trade = sell_market(&the_api_key, &the_secret_trimmed, &firstcoin_trimmed, &secondcoin_trimmed, &y.to_string(), &buy.Rate.to_string());
    								total_traded += available_trade;
    								let profit_multiplier = profit_parsed / 100.00;
    								let profit_multiplier_next = 1.00 - profit_multiplier;
    								let the_targetrate = buy.Rate * profit_multiplier_next;
    								let xy = (the_targetrate * 100000000.00).round() / 100000000.00;

    								let the_botstrade: BotTrade = BotTrade {
    									price: buy.Rate,
    									quantity: y,
    									target: xy,
    								};
    								bots_clone3.push(the_botstrade);
    										println!("TRADE MADE {:?}", the_botstrade.quantity);
    							}
    							else {
    								let the_adjusted_quantity = available_trade - buy.Quantity;
    								let y = (the_adjusted_quantity * 1000000.00).round() / 100000.00;
    								let the_trade = sell_market(&the_api_key, &the_secret_trimmed, &firstcoin_trimmed, &secondcoin_trimmed, &y.to_string(), &buy.Rate.to_string());
    								total_traded += the_adjusted_quantity;
    								let profit_multiplier = profit_parsed / 100.00;
    								let profit_multiplier_next = 1.00 - profit_multiplier;
    								let the_targetrate = buy.Rate * profit_multiplier_next;
    								let xy = (the_targetrate * 100000000.00).round() / 100000000.00;

    								let the_botstrade: BotTrade = BotTrade {
    									price: buy.Rate,
    									quantity: y,
    									target: xy,
    								};
    								bots_clone4.push(the_botstrade);
    										println!("TRADE MADE {:?}", the_botstrade.quantity);
    							}
    						}
    					}
    				}
    			}

    			if keep_timing == trade_interval {
    				keep_timing = 0;
    				total_traded = 0.00;
    				println!("resetting trade total");
    			}

    			if keep_daily_timing == 14400 {
    				let ticker_string = get_ticker(&firstcoin_trimmed, &secondcoin_trimmed);
    				twentyhour_price = ticker_string.Last;
    				target_price = target_price_multiplier * twentyhour_price;
    				keep_daily_timing = 0;
    			}

    			let the_seconds = Duration::new(6, 0);
				sleep(the_seconds);
    		}


    		// first needs to get the trade amount, 
    		/*
				first take the balance allocated so

				1.  fetch balance
					take % of balance
					divide %balance by 50
					= the size of rounds in the clip

					//max_trade_amount == round size

				2.	then get the last price of the currency

				3.	then get the order book, 
					filter for any orders that are less than those last price of the currency - x%

					then trigger a buy for what is on the list below x% of the last currency

					keep track of all buys to look to take profit

					keep track of how many intervals went through to reload the clip
					
					if it gets full it should just sit tight and start taking profit until all profit is taken 
					interval at a time, then reestablish with the last price profit was taken on the last position
					and continue buying again

					readjust start price each 24 hours

    		*/
		}
	}	
}
