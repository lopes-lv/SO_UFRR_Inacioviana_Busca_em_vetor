use std::sync::{Arc,Mutex};
use std::thread;
use std::io;
use rand::RngExt;


fn main() {

    //criando o vetor vazio
    let mut vetor:Vec<i32>=Vec:: new();

    //inicializando gerador de numeros aleatorios 
    let mut rng=rand::rng();
    
    //pegando o tamanho do vetor 
    println!("Qual será o tamanho do vetor?");
    let mut tamanho=String::new();
    io::stdin().read_line(&mut tamanho).expect("Falha ao ler");
    let tamanho:i32=tamanho.trim().parse().expect("Falha ao trasformar");

    //gerando o vetor com numeros aleatorios 
    for _ in 0..tamanho{
        let num_ale=rng.random_range(0..=tamanho);
        vetor.push(num_ale);
        //println!("numero adicionado {}",num_ale);
    }

    let vetor_arc=Arc::new(vetor);


    println!("Vetor criado com sucesso");

    //threads, variavel de encontro
    let encontrou=Arc::new(Mutex::new(false));    

    loop{

        let tamanho_total=vetor_arc.len();
       
        let encontrou_thread1=Arc::clone(&encontrou);

        let encontrou_thread2=Arc::clone(&encontrou);

        //pegando qual numero estamos procurando 
        println!("Qual numero procura? :");
        // trasformando ele em inteiro
        let mut alvo =String::new();
        io::stdin().read_line(&mut alvo).expect("Falha ao ler");
        let alvo:i32=alvo.trim().parse().expect("Falhou a trasformação");

        println!("tamanho do vetor {}",vetor_arc.len());
        let meio=vetor_arc.len()/2;
        
        //divindo o vetor em dois 
        let metade1=Arc::clone(&vetor_arc);

        let metade2=Arc::clone(&vetor_arc);


        //criando as threads
        let thread1=thread:: spawn( move ||{
        
            for i in 0..meio{
                if *encontrou_thread1.lock().unwrap() == true{
                    break;
                }
                if metade1[i]==alvo{
                    *encontrou_thread1.lock().unwrap()=true;
                    break;
                    
                }
            }

        });

        let thread2=thread::spawn(move ||{
        
            for i in meio..tamanho_total{
                if *encontrou_thread2.lock().unwrap() == true{
                    break;
                }
                if metade2[i]==alvo{
                    *encontrou_thread2.lock().unwrap()=true;
                    break;
                    
                }
            }

        });

        //fazendo o programa esperar ate que elas terminem 
        thread1.join().unwrap();
        thread2.join().unwrap();

        //print dizendo se o numero foi encontrado ou não 
        println!("O numero {} foi encontrado? {}", alvo,*encontrou.lock().unwrap());

        
        if *encontrou.lock().unwrap() == true{
                break;
        }

    }


}
