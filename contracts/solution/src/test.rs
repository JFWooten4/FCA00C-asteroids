#![cfg(test)]

use std::println;

use soroban_sdk::Env;

use crate::{
    engine::{Client as GameEngine, WASM as GameEngineWASM},
    Solution, SolutionClient,
};

extern crate std;

/// ESPECIALLY LEAVE THESE TESTS ALONE
#[test]
fn fca00c_fast() {
    let env = Env::default();
    let engine_id = env.register_contract_wasm(None, GameEngineWASM);
    let engine = GameEngine::new(&env, &engine_id);

    engine.init(
        &1,    // The number of spaces your ship will `p_move()` by default
        &3,    // The maximum distance from which your ship's laser can `p_shoot()` an asteroid
        &8891, // The map's randomness is seeded with a known, consistent `u64` value (this will produce the same map every time for everybody)
        &16,   // The size of each galaxy grid (so 16x16)
        &(
            50, // The amount of fuel your ship contains at initialization
            5,  // The amount of fuel consumed by the `p_shoot()` method
            2,  // The amount of fuel consumed when you `p_move()` a single space
            1,  // The amount of fuel consumed by the `p_turn()` method
        ),
        &1, // The number of points you are rewarded for destroying an asteroid
        &6, // The number of asteroids each galaxy will contain
        &2, // The number of fuel pods each galaxy will contain
    );

    let solution_id = env.register_contract(None, Solution);
    let solution = SolutionClient::new(&env, &solution_id);

    env.budget().reset();

    solution.solve(&engine_id);

    let points = engine.p_points();

    println!("Points: {}", points);
    assert!(points >= 100);
}

#[test]
pub fn fca00c_budget() {
    let env = Env::default();
    let engine_id = env.register_contract_wasm(None, GameEngineWASM);
    let engine = GameEngine::new(&env, &engine_id);

    engine.init(&1, &3, &8891, &16, &(50, 5, 2, 1), &1, &6, &2);

    mod solution {
        soroban_sdk::contractimport!(
            file = "../../target/wasm32-unknown-unknown/release/soroban_asteroids_solution.wasm"
        );
    }

    let solution_id = env.register_contract_wasm(None, solution::WASM);
    let solution = solution::Client::new(&env, &solution_id);

    env.budget().reset();

    solution.solve(&engine_id);

    env.budget().print();

    let points = engine.p_points();

    println!("Points: {}", points);
    assert!(points >= 100);
}

#[test]
pub fn output_entire_map() {
    let env = Env::default();
    let engine_id = env.register_contract_wasm(None, GameEngineWASM);
    let engine = GameEngine::new(&env, &engine_id);

    engine.init(
        &1, &3, &8891, &16, &(
            1, // The amount of fuel your ship contains at initialization
            0,  // The amount of fuel consumed by the `p_shoot()` method
            0,  // The amount of fuel consumed when you `p_move()` a single space
            0,  // The amount of fuel consumed by the `p_turn()` method
        ), &1, &6, &2,
    );

    
    engine.p_turn(Left)
    engine.p_move(16)
    engine.p_turn(Up)
    engine.p_move(16)
    let mut cur_galaxy_map = enginge.get_map();
    println!(":?", cur_galaxy_map);

    engine.p_turn(Right)
    engine.p_move(16)
    let mut cur_galaxy_map = enginge.get_map();
    println!(":?", cur_galaxy_map);

    engine.p_move(16)
    let mut cur_galaxy_map = enginge.get_map();
    println!(":?", cur_galaxy_map);

    engine.p_turn(Left)
    engine.p_move(16 * 3)
    engine.p_turn(Down)
    engine.p_move(16)
    let mut cur_galaxy_map = enginge.get_map();
    println!(":?", cur_galaxy_map);

    engine.p_turn(Right)
    engine.p_move(16)
    let mut cur_galaxy_map = enginge.get_map();
    println!(":?", cur_galaxy_map);

    engine.p_move(16)
    let mut cur_galaxy_map = enginge.get_map();
    println!(":?", cur_galaxy_map);

    engine.p_turn(Left)
    engine.p_move(16 * 3)
    engine.p_turn(Down)
    engine.p_move(16)
    let mut cur_galaxy_map = enginge.get_map();
    println!(":?", cur_galaxy_map);

    engine.p_turn(Right)
    engine.p_move(16)
    let mut cur_galaxy_map = enginge.get_map();
    println!(":?", cur_galaxy_map);

    engine.p_move(16)
    let mut cur_galaxy_map = enginge.get_map();
    println!(":?", cur_galaxy_map);

}