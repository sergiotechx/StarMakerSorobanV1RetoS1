**Reto Workshop Soroban Sesión 1**

😉**Pistas y guias para el reto:** [https://developers.stellar.org/docs/build/smart-contracts](https://developers.stellar.org/docs/build/smart-contracts)

*   implementar la función _sumar_, se le envian 2 números de parámetros
*   implementar _resultado\_anterior, este debe retornar el valor que dió la suma anteriormente ( Storing Data), esto implica hacer algo en la función sumar🤔_

_Clonar el respositorio y cuando esté resuelto subir el repositorio a tu github_

_debe dar ok el test automatico_

```plaintext
cargo test
```


**comandos para probar la solución al reto**

stellar contract build

stellar contract deploy `
 --wasm target\wasm32v1-none\release\calculadora.wasm `
  --source <alias> `
  --network testnet `
  --alias calculadora

stellar contract invoke --id <contract_id> --source <alias> --network testnet -- sumar --a 1 --b 2
stellar contract invoke --id <contract_id> --source <alias> --network testnet -- resultado_anterior 
stellar contract invoke --id <contract_id> --source <alias> --network testnet -- sumar --a 11 --b 2
stellar contract invoke --id <contract_id> --source <alias> --network testnet -- resultado_anterior
