extern crate wasm_bindgen;
extern crate rand;

use wasm_bindgen::prelude::*;
use rand::Rng;

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
  Alabama 
, Alaska 
, Arizona 
, Arkansas 
, California 
, Colorado 
, Connecticut 
, Delaware 
, Florida 
, Georgia 
, Hawaii 
, Idaho 
, Illinois
, Indiana 
, Iowa 
, Kansas 
, Kentucky 
, Louisiana 
, Maine 
, Maryland 
, Massachusetts 
, Michigan 
, Minnesota 
, Mississippi 
, Missouri 
, Montana
, Nebraska 
, Nevada 
, NewHampshire
, NewJersey 
, NewMexico 
, NewYork 
, NorthCarolina 
, NorthDakota 
, Ohio 
, Oklahoma 
, Oregon 
, Pennsylvania
, RhodeIsland 
, SouthCarolina 
, SouthDakota 
, Tennessee 
, Texas 
, Utah 
, Vermont 
, Virginia 
, Washington 
, WestVirginia 
, Wisconsin 
, Wyoming
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
  match coin {
    Coin::Penny          => 1
  , Coin::Nickel         => 5
  , Coin::Dime           => 10
  , Coin::Quarter(state) => {
      log(&format!("State quarter from {:?}!", state)[..]);
      25
    }
  }
}

fn coin_gen(idx: u8) -> Coin {
  match idx {
    0 => Coin::Penny
  , 1 => Coin::Nickel
  , 2 => Coin::Dime
  , _ => Coin::Quarter(state_gen(rand::thread_rng().gen_range(0, 50)))
  }
}

fn state_gen(idx: u8) -> UsState {
  let valid_idx = if idx >= 50 { idx % 50 } else { idx };

  match valid_idx {
       0 => UsState::Alabama 
    ,  1 => UsState::Alaska 
    ,  2 => UsState::Arizona 
    ,  3 => UsState::Arkansas 
    ,  4 => UsState::California 
    ,  5 => UsState::Colorado 
    ,  6 => UsState::Connecticut 
    ,  7 => UsState::Delaware 
    ,  8 => UsState::Florida 
    ,  9 => UsState::Georgia 
    , 10 => UsState::Hawaii 
    , 11 => UsState::Idaho 
    , 12 => UsState::Illinois
    , 13 => UsState::Indiana 
    , 14 => UsState::Iowa 
    , 15 => UsState::Kansas 
    , 16 => UsState::Kentucky 
    , 17 => UsState::Louisiana 
    , 18 => UsState::Maine 
    , 19 => UsState::Maryland 
    , 20 => UsState::Massachusetts 
    , 21 => UsState::Michigan 
    , 22 => UsState::Minnesota 
    , 23 => UsState::Mississippi 
    , 24 => UsState::Missouri 
    , 25 => UsState::Montana
    , 26 => UsState::Nebraska 
    , 27 => UsState::Nevada 
    , 28 => UsState::NewHampshire
    , 29 => UsState::NewJersey 
    , 30 => UsState::NewMexico 
    , 31 => UsState::NewYork 
    , 32 => UsState::NorthCarolina 
    , 33 => UsState::NorthDakota 
    , 34 => UsState::Ohio 
    , 35 => UsState::Oklahoma 
    , 36 => UsState::Oregon 
    , 37 => UsState::Pennsylvania
    , 38 => UsState::RhodeIsland 
    , 39 => UsState::SouthCarolina 
    , 40 => UsState::SouthDakota 
    , 41 => UsState::Tennessee 
    , 42 => UsState::Texas 
    , 43 => UsState::Utah 
    , 44 => UsState::Vermont 
    , 45 => UsState::Virginia 
    , 46 => UsState::Washington 
    , 47 => UsState::WestVirginia 
    , 48 => UsState::Wisconsin 
    , 49 => UsState::Wyoming
    ,  _ => UsState::Wyoming
  }
}

fn as_currency_amount(amt: u16) -> String {
  format!("${:.*}", 2, (amt as f32) / 100.0)
}

// *********************************************************************************************************************
// Functions consumed from JavaScript
// *********************************************************************************************************************
#[wasm_bindgen]
extern "C" {
  // Use `js_namespace` here to bind `console.log(..)` instead of just `log(..)`
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);

  // The `console.log` is quite polymorphic, so we can bind it with multiple signatures.
  // Note that we need to use `js_name` to ensure we always call `log` in JS.
  #[wasm_bindgen(js_namespace = console, js_name = log)]
  fn log_u32(a: u32);

  // Multiple arguments too!
  #[wasm_bindgen(js_namespace = console, js_name = log)]
  fn log_many(a: &str, b: &str);
}


// *********************************************************************************************************************
// Public API
// *********************************************************************************************************************
#[wasm_bindgen]
pub fn main(coins: u32) {
  // let mut coins = String::new();

  // println!("How many coins are in the piggy bank?");

  // io::stdin()
  //   .read_line(&mut coins)
  //   .expect("Failed to read line");

  // let coins: u32 = match coins.trim().parse() {
  //     Ok(num) => num
  //   , Err(err_val) => {
  //       println!("{} is not a valid number", err_val);
  //       0
  //   }
  // };

  let mut purse = Vec::<Coin>::new();

  for _idx in 0..coins {
    purse.push(coin_gen(rand::thread_rng().gen_range(0, 4)))
  }

  let mut total: u16 = 0;

  for coin in purse.iter().clone() {
    total += value_in_cents(coin) as u16;
  }

  log(&format!("The {} coins in your purse are worth {}", coins, as_currency_amount(total)));
}
