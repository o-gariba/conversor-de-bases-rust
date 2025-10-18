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

## Métodos Utilizados

O projeto é implementado inteiramente dentro da função `main` e utiliza uma combinação de métodos da biblioteca padrão do Rust e de crates externas para realizar a conversão de bases. Abaixo estão os métodos mais importantes e suas respectivas funções:

### Leitura e Escrita no Console

- **`std::io::stdout().flush()`**: Garante que o prompt de entrada seja exibido no terminal antes de o programa pausar para aguardar a resposta do usuário.
- **`std::io::stdin().read_line()`**: Realiza a leitura da entrada do usuário, como a base de origem, o valor a ser convertido e a base de destino.

### Manipulação de Strings

- **`String::trim()`**: Remove espaços em branco do início e do final da entrada do usuário, garantindo que a validação e o processamento dos dados não sejam afetados por espaçamento acidental.
- **`String::parse()`**: Converte a string que representa a base numérica (lida do terminal) em um tipo inteiro (`u32`), permitindo sua utilização em cálculos.
- **`String::chars()`**: Cria um iterador sobre os caracteres da string de entrada, permitindo que cada dígito do número seja processado individualmente.
- **`String::to_lowercase()`**: Converte a resposta do usuário para letras minúsculas ao final de cada conversão, simplificando a verificação de sua decisão de continuar ou sair.

### Conversão e Validação de Dígitos

- **`char::to_digit()`**: Converte um caractere (como 'A' ou '9') em seu valor numérico correspondente, com base na base de origem. É um método central para a implementação do Método de Horner.
- **`char::from_digit()`**: Realiza a operação inversa, convertendo um valor numérico em seu caractere correspondente na base de destino (por exemplo, 10 se torna 'A' em hexadecimal).
- **`char::to_ascii_uppercase()`**: Garante que os dígitos alfabéticos (A-F) no resultado final sejam exibidos em maiúsculas, seguindo a convenção padrão.

### Manipulação de Vetores

- **`Vec::new()`**: Inicializa um vetor vazio para armazenar os dígitos do número convertido na base de destino.
- **`Vec::push()`**: Adiciona cada novo dígito calculado ao final do vetor.
- **`Vec::reverse()`**: Inverte a ordem dos dígitos no vetor, já que o algoritmo de conversão para a base de destino gera os dígitos do menos significativo para o mais significativo.
- **`Vec::iter().collect()`**: Concatena os caracteres do vetor em uma `String`, formando o resultado final a ser exibido.

### Funcionalidades Adicionais

- **`colored::Colorize`**: Melhora a interface do usuário no terminal, adicionando cores para destacar prompts, mensagens de erro e resultados, tornando a interação mais clara e agradável. Métodos como `cyan()`, `yellow()`, `red()` e `green()` são utilizados para esse fim.