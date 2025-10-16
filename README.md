# Conversor de Bases Numéricas

## Visão Geral

Este projeto é uma ferramenta de linha de comando desenvolvida em Rust para a conversão de números entre diferentes bases numéricas, suportando bases de 2 a 16. A principal motivação para sua criação foi a necessidade de aprofundar os conhecimentos em Rust e explorar conceitos matemáticos fundamentais aplicados à computação.

## Conceitos Principais

### Método de Horner

O Método de Horner é um algoritmo eficiente para a avaliação de polinômios e, neste contexto, é utilizado para converter um número de uma base qualquer para a base decimal (base 10).

Um número `d_n d_{n-1} ... d_1 d_0` em uma base `b` pode ser representado como um polinômio:

`d_n * b^n + d_{n-1} * b^{n-1} + ... + d_1 * b^1 + d_0 * b^0`

O algoritmo de Horner otimiza o cálculo, reorganizando a expressão da seguinte forma:

`((...((d_n * b + d_{n-1}) * b + d_{n-2}) * b + ...) * b + d_1) * b + d_0`

Isso reduz o número de operações de exponenciação, tornando a conversão mais rápida e computacionalmente menos custosa. O processo consiste em percorrer os dígitos do número, da esquerda para a direita, multiplicando o resultado acumulado pela base e somando o valor do dígito atual.

### Crate `BigUInt`

Para garantir que o conversor possa lidar com números de tamanho arbitrário, que excedem a capacidade dos tipos de dados primitivos (como `u64` ou `u128`), o projeto utiliza a crate `num-bigint`, que fornece o tipo `BigUint`.

O `BigUint` representa um inteiro não negativo de precisão ilimitada, permitindo que o programa realize cálculos com números extremamente grandes sem risco de overflow. Isso é crucial para a conversão de números longos ou de bases elevadas, onde os valores intermediários na conversão para a base decimal podem crescer rapidamente.
