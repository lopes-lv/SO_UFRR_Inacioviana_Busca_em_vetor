use std::sync::Arc;
use std::sync::atomic::{AtomicBool,Ordering};
use std::thread;
use std::io;
use std::time::Instant;

fn busca_sequencial(alvo:i32,vetor:&Vec<i32>)->bool{

    for i in 0..vetor.len(){
        if vetor[i]==alvo{
            return true;
        }
    }
    return false;
}


fn main() {

    //criando o vetor vazio
    let mut vetor:Vec<i32>=Vec:: new();

    //inicializando gerador de numeros aleatorios 
    
    
    //pegando o tamanho do vetor 
    println!("Qual será o tamanho do vetor?");
    let mut tamanho=String::new();
    io::stdin().read_line(&mut tamanho).expect("Falha ao ler");
    let tamanho:i32=tamanho.trim().parse().expect("Falha ao trasformar");

    //gerando o vetor com numeros aleatorios 
    for i in 0..tamanho{
        vetor.push(i);
        //println!("numero adicionado {}",i);
    }

    let vetor_arc=Arc::new(vetor);


    println!("Vetor criado com sucesso");

    //threads, variavel de encontro
    let encontrou=Arc::new(AtomicBool::new(false));    

    let tamanho_total=vetor_arc.len();
    
    let encontrou_thread1=Arc::clone(&encontrou);

    let encontrou_thread2=Arc::clone(&encontrou);

    //pegando qual numero estamos procurando 
    println!("Qual numero procura? :");
    // trasformando ele em inteiro
    let mut alvo =String::new();
    io::stdin().read_line(&mut alvo).expect("Falha ao ler");
    let alvo:i32=alvo.trim().parse().expect("Falhou a trasformação");

    
    let meio=vetor_arc.len()/2;
    
    //divindo o vetor em dois 
    let metade1=Arc::clone(&vetor_arc);

    let metade2=Arc::clone(&vetor_arc);

    let inicio=Instant::now();

    //criando as threads
    let thread1=thread:: spawn( move ||{
    
        for i in 0..meio{
            if encontrou_thread1.load(Ordering::Relaxed){
                break;
            }
            if metade1[i]==alvo{
                encontrou_thread1.store(true,Ordering::Relaxed);
                break;
                
            }
        }

    });

    let thread2=thread::spawn(move ||{
    
        for i in meio..tamanho_total{
            if encontrou_thread2.load(Ordering::Relaxed){
                break;
            }
            if metade2[i]==alvo{
                encontrou_thread2.store(true,Ordering::Relaxed);
                break;
                
            }
        }

    });

    //fazendo o programa esperar ate que elas terminem 
    thread1.join().unwrap();
    thread2.join().unwrap();

    let fim=inicio.elapsed();

    let init_seq=Instant::now();
    let resultado_seq=busca_sequencial(alvo,&vetor_arc);
    let fim_seq=init_seq.elapsed();

    //print dizendo se o numero foi encontrado ou não 
    println!("---------------------------------------");
    println!("Tamanho do vetor: {}",vetor_arc.len());
    println!("O numero alvo: {}",alvo);
    if encontrou.load(Ordering::Relaxed) && resultado_seq{println!("Status: Encontrado")}
    else{println!("Statu: Não encontrado")};

    println!("---------------------------------------");
    println!("DESEMPENHO");

    println!("O tempo de procura com threads foi de: {:?}\nO tempo da busca sequencial foi de {:?}",fim,fim_seq);

}
