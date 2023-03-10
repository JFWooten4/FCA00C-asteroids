#![no_std]

use engine::Client as GameEngine;
use soroban_sdk::{contractimpl, BytesN, Env};

pub struct Solution;

mod engine {
    soroban_sdk::contractimport!(file = "../game_engine.wasm");
}

mod test;

#[contractimpl]
impl Solution {
    pub fn solve(env: Env, engine_id: BytesN<32>) {
        let engine = GameEngine::new(&env, &engine_id);

        // YOUR CODE START
        engine.p_shoot();

        engine.p_turn(&engine::Direction::DownRight);
        engine.p_move(&Some(6));
        engine.p_shoot();
    
        engine.p_turn(&engine::Direction::Down);
        engine.p_move(&Some(1));
        engine.p_harvest();
    
        engine.p_move(&Some(3));
        engine.p_harvest();
    
        engine.p_turn(&engine::Direction::Left);
        engine.p_move(&Some(1));
        engine.p_harvest(); 
    
        engine.p_turn(&engine::Direction::DownRight);
        engine.p_shoot();    
    
        engine.p_turn(&engine::Direction::UpRight);
        engine.p_move(&Some(9));
        engine.p_turn(&engine::Direction::Up);
        engine.p_shoot();
        engine.p_upgrade();
    
        engine.p_turn(&engine::Direction::UpLeft);
        engine.p_move(&Some(1));
        engine.p_harvest();
    
        
    
        engine.p_turn(&engine::Direction::Up);
        engine.p_move(&Some(10)); // 10 or 11 works
        engine.p_shoot();
    
        engine.p_turn(&engine::Direction::UpLeft);
        engine.p_move(&Some(4));
        engine.p_turn(&engine::Direction::Up);
        engine.p_move(&Some(8));
        engine.p_shoot();
    
        engine.p_turn(&engine::Direction::Down);
        engine.p_move(&Some(1));
        engine.p_turn(&engine::Direction::DownRight);
        engine.p_shoot();
        
        engine.p_move(&Some(10));
        engine.p_turn(&engine::Direction::Up);
        engine.p_move(&Some(5));
    
        engine.p_turn(&engine::Direction::UpLeft);
        engine.p_shoot();
    
        engine.p_turn(&engine::Direction::Down);
        engine.p_move(&Some(6));
        engine.p_harvest();
    
        engine.p_turn(&engine::Direction::UpRight);
        engine.p_move(&Some(1));
        engine.p_harvest();
    
        engine.p_turn(&engine::Direction::Up);
        engine.p_move(&Some(6));
    
        engine.p_turn(&engine::Direction::UpRight);
        engine.p_move(&Some(9));
        engine.p_turn(&engine::Direction::Down);
        engine.p_move(&Some(4)); // 3 or 4 works
        engine.p_shoot();
    
        engine.p_turn(&engine::Direction::DownRight);
        engine.p_shoot();
        engine.p_move(&Some(7));
        engine.p_turn(&engine::Direction::Down);
        engine.p_shoot();
    
        engine.p_turn(&engine::Direction::UpLeft);
        engine.p_move(&Some(7));
    
        engine.p_turn(&engine::Direction::UpRight);
        engine.p_move(&Some(25));
        engine.p_shoot();
    
        engine.p_turn(&engine::Direction::UpLeft);
        engine.p_move(&Some(2));
        engine.p_turn(&engine::Direction::Left);
        engine.p_shoot();
        engine.p_turn(&engine::Direction::Down);
        engine.p_shoot();
    
        engine.p_turn(&engine::Direction::UpRight);
        engine.p_move(&Some(3));
        engine.p_shoot();
    
        engine.p_turn(&engine::Direction::DownLeft);
        engine.p_move(&Some(1));
        engine.p_turn(&engine::Direction::Up);
        engine.p_shoot();
    
        engine.p_turn(&engine::Direction::UpLeft);
        engine.p_move(&Some(18));
        engine.p_turn(&engine::Direction::UpRight);
        engine.p_shoot();
    
        engine.p_turn(&engine::Direction::DownLeft);
        engine.p_move(&Some(7));
        engine.p_shoot();
        engine.p_turn(&engine::Direction::Up);
        engine.p_move(&Some(1));
        engine.p_turn(&engine::Direction::DownLeft);
        engine.p_shoot();
        engine.p_turn(&engine::Direction::Up);
        engine.p_move(&Some(1));
        engine.p_turn(&engine::Direction::UpRight);
        engine.p_shoot();
        engine.p_turn(&engine::Direction::Down);
        engine.p_move(&Some(2));
        
    
        engine.p_turn(&engine::Direction::UpRight);
        engine.p_move(&Some(7));
        engine.p_turn(&engine::Direction::DownRight);
        engine.p_move(&Some(2));
        engine.p_turn(&engine::Direction::Down);
        engine.p_shoot();
    
        engine.p_turn(&engine::Direction::UpLeft);
        engine.p_move(&Some(2));
    
        
        
        engine.p_turn(&engine::Direction::Up);
        engine.p_move(&Some(10)); // 9 or 10
        engine.p_shoot();
    
        engine.p_turn(&engine::Direction::Up);
        engine.p_move(&Some(2));
        engine.p_shoot();
        engine.p_turn(&engine::Direction::UpRight);
        engine.p_move(&Some(1));
        engine.p_turn(&engine::Direction::Up);
        engine.p_move(&Some(4));
        engine.p_shoot();
        engine.p_turn(&engine::Direction::DownRight);
        engine.p_move(&Some(3));
        engine.p_turn(&engine::Direction::Down);
        engine.p_shoot();
    
        // 44, 87
        engine.p_turn(&engine::Direction::DownLeft);
        engine.p_move(&Some(22));
        engine.p_turn(&engine::Direction::Up);
        engine.p_shoot();
    
        engine.p_turn(&engine::Direction::UpRight);
        engine.p_move(&Some(4));
        engine.p_turn(&engine::Direction::DownRight);
        engine.p_shoot();
        engine.p_turn(&engine::Direction::Up);
        engine.p_move(&Some(4));
        engine.p_turn(&engine::Direction::UpLeft);
        engine.p_shoot();
        engine.p_turn(&engine::Direction::Down);
        engine.p_move(&Some(8));
        engine.p_turn(&engine::Direction::DownRight);
        engine.p_shoot();
        engine.p_turn(&engine::Direction::UpLeft);
        engine.p_move(&Some(11));
        engine.p_turn(&engine::Direction::Right);
        engine.p_move(&Some(1));
        engine.p_shoot();
        engine.p_turn(&engine::Direction::DownRight);
        engine.p_move(&Some(11));
        engine.p_turn(&engine::Direction::Left);
        engine.p_move(&Some(5));
    
        
        // 26, 69
        engine.p_turn(&engine::Direction::UpLeft);
        engine.p_move(&Some(10));
        engine.p_shoot();
    
        
        
        engine.p_move(&Some(12));
        engine.p_turn(&engine::Direction::Up);
        engine.p_shoot();
        
        engine.p_turn(&engine::Direction::DownLeft);
        engine.p_move(&Some(4));
        engine.p_turn(&engine::Direction::Left);
        engine.p_shoot();
        engine.p_move(&Some(1));
        engine.p_shoot();
        engine.p_turn(&engine::Direction::UpLeft);
        engine.p_shoot();
    
        engine.p_turn(&engine::Direction::UpRight);
        engine.p_move(&Some(18));
        
        engine.p_shoot();
        engine.p_turn(&engine::Direction::UpLeft);
        engine.p_move(&Some(7));
        engine.p_turn(&engine::Direction::Down);
        engine.p_shoot();
    
        
    
        engine.p_turn(&engine::Direction::UpRight);
        engine.p_move(&Some(8));
    
        engine.p_turn(&engine::Direction::Up);
        engine.p_shoot();
        engine.p_turn(&engine::Direction::Down);
        engine.p_shoot();
        engine.p_turn(&engine::Direction::Right);
        engine.p_shoot();
    
        engine.p_turn(&engine::Direction::UpRight);
        engine.p_move(&Some(7));
    
        engine.p_turn(&engine::Direction::Left);
        engine.p_shoot();
    
        engine.p_turn(&engine::Direction::Up);
        engine.p_move(&Some(20));
        engine.p_harvest();
        engine.p_shoot();
    
        engine.p_turn(&engine::Direction::Left);
        engine.p_shoot();
        
        
        engine.p_turn(&engine::Direction::UpLeft);
        engine.p_move(&Some(20));
        engine.p_turn(&engine::Direction::Left);
        engine.p_shoot();
    
        
    
        engine.p_turn(&engine::Direction::DownLeft);
        engine.p_move(&Some(7));
        engine.p_turn(&engine::Direction::Up);
        engine.p_move(&Some(4)); // 4 or 5
        engine.p_shoot();
        engine.p_turn(&engine::Direction::UpRight);
        engine.p_move(&Some(6));
        engine.p_turn(&engine::Direction::Up);
        engine.p_shoot();
    
        engine.p_turn(&engine::Direction::UpLeft);
        engine.p_move(&Some(19));
        engine.p_turn(&engine::Direction::Right);
        engine.p_shoot();
        engine.p_turn(&engine::Direction::Left);
        engine.p_move(&Some(7)); // 6 or 7
        engine.p_shoot();
    
        engine.p_turn(&engine::Direction::DownRight);
        engine.p_move(&Some(16));
        engine.p_turn(&engine::Direction::Right);
        engine.p_harvest();
        engine.p_shoot();
    
        engine.p_turn(&engine::Direction::DownLeft);
        engine.p_move(&Some(16));
        engine.p_turn(&engine::Direction::Down);
        engine.p_harvest();
        engine.p_shoot();
    
        engine.p_turn(&engine::Direction::UpLeft);
        engine.p_move(&Some(8));
        engine.p_turn(&engine::Direction::Left);
        engine.p_shoot();
    
        engine.p_turn(&engine::Direction::UpLeft);
        engine.p_move(&Some(14));
        engine.p_shoot();
        engine.p_move(&Some(13));
        engine.p_turn(&engine::Direction::Left);
        engine.p_shoot();
    
        
    
        engine.p_turn(&engine::Direction::DownLeft);
        engine.p_move(&Some(14));
        engine.p_shoot();
        engine.p_move(&Some(12));
        engine.p_turn(&engine::Direction::Left);
        engine.p_move(&Some(12)); // 10 - 12
        engine.p_shoot();
        engine.p_turn(&engine::Direction::UpLeft);
        engine.p_move(&Some(1));
        engine.p_turn(&engine::Direction::Left);
        engine.p_shoot();
        engine.p_turn(&engine::Direction::UpLeft);
        engine.p_move(&Some(5));
        engine.p_turn(&engine::Direction::Up);
        engine.p_shoot();
    
        engine.p_turn(&engine::Direction::UpRight);
        engine.p_move(&Some(8));
        engine.p_turn(&engine::Direction::Down);
        engine.p_shoot();
        
        engine.p_move(&Some(3));
        engine.p_turn(&engine::Direction::UpLeft);
        engine.p_move(&Some(19));
    
        
    
        engine.p_turn(&engine::Direction::Up);
        engine.p_shoot();
        engine.p_turn(&engine::Direction::Left);
        engine.p_shoot();
        
        engine.p_turn(&engine::Direction::DownLeft);
        engine.p_move(&Some(15));
        engine.p_turn(&engine::Direction::Down);
        engine.p_shoot();
    
        engine.p_turn(&engine::Direction::Left);
        engine.p_move(&Some(3));
        engine.p_shoot();
    
        engine.p_turn(&engine::Direction::DownLeft);
        engine.p_move(&Some(2)); // 2 or 3
        engine.p_shoot();
    
        engine.p_turn(&engine::Direction::Up);
        engine.p_move(&Some(10)); // 9 or 10
        engine.p_shoot();
    
        
    
        engine.p_turn(&engine::Direction::UpLeft);
        engine.p_move(&Some(18)); // =29
        engine.p_turn(&engine::Direction::Right);
        engine.p_shoot();
        engine.p_turn(&engine::Direction::Up);
        engine.p_shoot();
        engine.p_turn(&engine::Direction::UpLeft);
        engine.p_move(&Some(4));
        engine.p_shoot();
        engine.p_move(&Some(7));
    
        engine.p_turn(&engine::Direction::Down);
        engine.p_shoot();
        engine.p_turn(&engine::Direction::UpLeft);
        engine.p_move(&Some(9));
        engine.p_harvest();
    
        engine.p_turn(&engine::Direction::DownLeft);
        engine.p_move(&Some(1));
        engine.p_harvest();
    
        
    
        engine.p_move(&Some(7));
        engine.p_turn(&engine::Direction::Left);
        engine.p_shoot();
    
        engine.p_turn(&engine::Direction::DownRight);
        engine.p_move(&Some(14));
        engine.p_turn(&engine::Direction::Left);
        engine.p_shoot();
        
        engine.p_turn(&engine::Direction::DownRight);
        engine.p_move(&Some(4));
        engine.p_turn(&engine::Direction::Left);
        engine.p_shoot();
    
        
        
        engine.p_turn(&engine::Direction::DownLeft);
        engine.p_move(&Some(22));
        engine.p_harvest();
        engine.p_move(&Some(17));
        engine.p_turn(&engine::Direction::Right);
        engine.p_shoot();
    
        engine.p_turn(&engine::Direction::UpLeft);
        engine.p_move(&Some(17));
        engine.p_harvest();
        engine.p_move(&Some(5));
        engine.p_shoot();
        engine.p_move(&Some(9));
        engine.p_turn(&engine::Direction::Up);
        engine.p_shoot();
    
        engine.p_turn(&engine::Direction::UpRight);
        engine.p_move(&Some(31));
    
        engine.p_turn(&engine::Direction::Right);
        engine.p_shoot();
        engine.p_turn(&engine::Direction::Down);
        engine.p_move(&Some(6));
        engine.p_shoot();
        engine.p_turn(&engine::Direction::Left);
        engine.p_move(&Some(3));
        engine.p_shoot();
        engine.p_turn(&engine::Direction::UpLeft);
        engine.p_move(&Some(3));
        engine.p_shoot();
        
        // YOUR CODE END
    }
}
