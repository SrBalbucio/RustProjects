use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::ops::Add;

fn main() {
    let version = 1.0;
    println!("");
    println!("");
    println!("Criado por Sr_Balbucio!");
    println!("Versão: v{version}");
    println!("");
    let mut running: bool = true;
    let mut command = String::new();
    while running {
        command = String::new();
        println!("");
        println!("Diga abaixo o que devo fazer agora:");
        io::stdin()
            .read_line(&mut command)
            .expect("Ocorreu um falha fatal ao ler o seu comando!");
        if command.contains("exit") {
            running = false;
            println!("Me usar e dps só sai com um exit né... pdp");
            return;
        } else if command.contains("analise_texto") {
            println!("Escreva o que eu devo dizer abaixo:");
            command = String::new();
            io::stdin()
                .read_line(&mut command)
                .expect("Ocorreu um falha fatal ao ler o seu comando!");
            println!("Analisando texto...");
            std::thread::sleep(std::time::Duration::from_millis(1000 * 10));
            println!("");
            println!("Texto analisado, conclusão: é uma bosta.");
            println!("O texto analisado foi: {command}");
            println!("O seu texto tem {} caracteres", command.len());
            println!("");
        } else if command.contains("abrir_txt"){
            println!("Onde está o arquivo: (sem a extensão)");
            command = String::new();
            io::stdin().read_to_string(&mut command).expect("Ocorreu uma falha ao ler o caminho");
            let mut file = File::create(command.replace("\n", "").add(".txt")).expect("Ocorreu um erro ao criar o arquivo");
            println!("Arquivo encontrado, preparando para ler!");
            let mut filecontents = String::new();
            file.read_to_string(&mut filecontents).expect("Não foi possível ler o lixo de arquivo que você criou!");
            println!("");
            println!("CONTEÚDO DO ARQUIVO > {filecontents}");
            println!("");

        } else if command.contains("criar_txt"){
            println!("Que texto você deseja por no arquivo:");
            command = String::new();
            io::stdin().read_line(&mut command).expect("Ocorreu uma falha ao ler o seu texto");
            println!("Diga o nome do arquivo: (sem extensão)");
            let mut name = String::new();
            io::stdin().read_line(&mut name).expect("Ocorreu uma falha ao ler o nome do arquivo");
            let mut file = File::create(name.replace("\n", "").add(".txt")).expect("Ocorreu um erro ao criar o arquivo");
            println!("Arquivo criado, escrevendo...");
            file.write_all(command.as_bytes()).expect("Não foi possível escrever o conteúdo porque tu é burro demais");
            println!("");
            println!("Arquivo criado com pouco sucesso!");
            println!("");

        } else if command.contains("game_2") {
            println!("Esse comando se restaura no final, para sair dele use exit");
            let mut game_2_running = true;
            while game_2_running {
                println!("Por favor escreva abaixo o nome da linguagem de programação a ser analisada:");
                command = String::new();
                io::stdin().read_line(&mut command).expect("Não foi possível ler a linguagem dita!");
                if (command.contains("exit")) {
                    game_2_running = false;
                } else {
                    println!("Linguagem de programação selecionada: {command}");
                    println!("Analisando...");
                    std::thread::sleep(std::time::Duration::from_millis(1000 * 5));
                    if command.contains("Java") {
                        println!("Resultado: APENAS PERFEITA!");
                    } else if command.contains("C#") {
                        println!("Resultado: Cheiro de Cópia Proprietária do Java...");
                    } else if command.contains("C") || command.contains("C++") {
                        println!("Resultado: Essencial");
                    } else if command.contains("Python") {
                        println!("Resultado: Pra quem não entende nada qualquer coisa é grande coisa");
                    } else if command.contains("Rust") {
                        println!("Resultado: Falar mal da linguagem que estou programando agora é estranho, portanto excelente!");
                    } else if command.contains("PHP") {
                        println!("Resultado: Nem sempre a segurança é um objetivo");
                    } else if command.contains("Lua") {
                        println!("Resultado: É brasileiro, pare de falar mal agora");
                    } else if command.contains("Assembly") {
                        println!("Resultado: Só é difícil pra quem já é burro");
                    } else {
                        println!("Resultado: Ok, se eu não tenho uma resposta pra isso é porque realmente não faz diferença na minha vida");
                    }
                }
            }
        } else if command.contains("game_1") {

        } else if command.contains("lock") {
            println!("Parando thread por 10 segundos!");
            std::thread::sleep(std::time::Duration::from_millis(1000 * 10));
            println!("Thread retornada!");
        } else if command.contains("version") {
            println!("Estou na versão v{version}, fui criado pelo SrBalbucio na linguagem Rust usando a licença GPL V3!");
            println!("Para saber meus comandos execute: commands ou comandos.");
            println!("Saiba que é um sofrimento trabalhar com você!");
        } else if command.contains("commands") || command.contains("comandos"){
            println!("Meus comandos são:");
            println!("analise_texto: Analisa um texto qualquer seu");
            println!("abrir_txt: Abre e vê o conteúdo de um TXT");
            println!("criar_txt: Criar TXT com conteúdo");
            println!("lock: Trava a aplicação por 10 segundos");
            println!("game_1: Adivinhe o número que estou pensando");
            println!("game_2: Fale uma linguagem de programação e direi se ela é +/- ou só ruim mesmo");
            println!("version: Veja a versão app");
            println!("help: Veja as informações do app");
            println!("exit: Fecha o app");
        } else if command.contains("help"){
            println!("Este programa foi criado por SrBalbucio na linguagem Rust!");
            println!("Para saber meus comandos execute: commands");
            println!("Para sair execute: exit");
        }
    }
}
