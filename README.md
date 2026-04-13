# Busca Concorrente em Vetores Gigantes (Rust)

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Status](https://img.shields.io/badge/Status-Conclu%C3%ADdo-success?style=for-the-badge)

##  Sobre o Projeto
Este projeto foi desenvolvido como requisito acadêmico para explorar padrões de paralelismo, sincronização e comunicação entre processos na disciplina de Sistemas Operacionais / Programação Concorrente na Universidade Federal de Roraima (UFRR). 

O sistema implementa um algoritmo de busca linear concorrente em um vetor de grande escala alocado dinamicamente. O espaço de busca é particionado e processado simultaneamente por múltiplas threads, otimizando o tempo de resposta através de mecanismos de *early exit* (saída antecipada).

##  Objetivo
Dividir um vetor em duas partes e usar dois threads para procurar um número.<br>
<br>
**Criterios de implementação**. 
  * Criar duas threads, cada uma responsável por metade do vetor.<br>
  * Usar variável global para indicar se o elemento foi encontrado.<br>
  * Proteger acesso com mutex para evitar condição de corrida.<br>


##  Lógica de Concorrência Utilizada

A arquitetura do projeto baseia-se no padrão de **Particionamento de Dados (Data Partitioning)**. 

1. **Compartilhamento Seguro (`Arc`):** O vetor principal e a variável de estado (`encontrou`) são envelopados em um `Arc` (Atomic Reference Counted). Isso permite que múltiplas threads possuam ponteiros de leitura para a mesma região de memória no Heap, contornando a regra estrita de *Ownership* do Rust sem a necessidade de duplicar dados (o que causaria estouro de memória em vetores gigantes).
2. **Exclusão Mútua (`Mutex`):** A flag booleana que indica se o alvo foi encontrado é protegida por um `Mutex`. Isso garante que apenas uma thread possa alterar o estado da variável por vez.
3. **Comunicação e Otimização:** A cada iteração do loop de busca, as threads realizam uma leitura não-bloqueante do Mutex. Se a flag indicar `true` (ou seja, a outra thread já encontrou o alvo), a thread atual executa um `break`, interrompendo o processamento desnecessário e economizando ciclos de CPU.

##  Desafios e Soluções

Durante a implementação, foram abordados os principais problemas clássicos da programação concorrente:

* **Condições de Corrida (Race Conditions):** Se ambas as threads encontrassem o alvo simultaneamente e tentassem escrever `true` na flag de estado, ocorreria um comportamento indefinido em nível de hardware. O uso estruturado do `Mutex` eliminou esse risco.
* **Deadlock:** O risco de Deadlock foi mitigado garantindo que as threads necessitem de apenas um único recurso bloqueante por vez (a flag booleana) e que esse *lock* seja liberado imediatamente após a leitura ou escrita na mesma iteração, impedindo o estado de espera circular.
* **Desempenho e Overhead:** O principal desafio técnico foi o custo de processamento ao travar/destravar o Mutex a cada iteração. Para vetores gigantes, o *overhead* do lock pode superar o ganho do paralelismo. A solução adotada (early exit) foi essencial para garantir que o desempenho global fosse superior à execução single-thread na maioria dos cenários.
