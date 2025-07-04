
#![no_std]


use soroban_sdk::{contract, contractimpl,  Env, symbol_short, Symbol};
const RESULTADO: Symbol = symbol_short!("RESULTADO");
#[contract]
pub struct Contract;


#[contractimpl]
impl Contract {
    
    pub fn sumar(env: Env, a: i128, b: i128) -> i128 {
        // Implementar función que sume dos números
        let resultado: i128 = a + b;
        env.storage().instance().set(&RESULTADO, &resultado);
       
        resultado
    }

    pub fn resultado_anterior(env: Env) -> i128 {
        // Implementar función que retorne el valor anterior
        
        let resultado = env.storage().instance().get(&RESULTADO).unwrap_or(0);
        return resultado;
    }
}

mod test;
