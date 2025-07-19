Lista de Tarefas Rust PLUS
Um aplicativo de linha de comando simples e eficiente escrito em Rust, projetado para gerenciar tarefas com persistência de dados em SQLite.
Visão Geral
todo_list_rust_PLUS é uma ferramenta CLI leve para gerenciar suas tarefas. Permite aos usuários adicionar, listar, alternar, remover e ordenar tarefas, com os dados armazenados em um banco de dados SQLite para persistência. Este projeto visa fornecer uma maneira rápida e confiável de organizar suas tarefas diárias, demonstrando as capacidades do Rust na construção de aplicativos CLI robustos.
Funcionalidades

Adicionar Tarefas: Criar novas tarefas com descrições.
Listar Tarefas: Visualizar todas as tarefas, com opções para filtrar por status (pendente ou concluída).
Alternar Status da Tarefa: Marcar tarefas como concluídas ou pendentes.
Remover Tarefas: Excluir tarefas pelo seu ID.
Ordenar Tarefas: Organizar tarefas por status de conclusão.
Armazenamento Persistente: Usa SQLite para armazenar tarefas em um banco de dados, garantindo que os dados sejam salvos entre as sessões.

Instalação
Pré-requisitos

Rust (instale via rustup)
Cargo (incluído com o Rust)
SQLite (geralmente pré-instalado na maioria dos sistemas)

Passos

Clone o repositório:git clone https://github.com/CiceroGGS/todo_list_rust_PLUS.git


Navegue até o diretório do projeto:cd todo_list_rust_PLUS


Compile o projeto:cargo build --release


(Opcional) Mova o executável para um caminho do sistema para acesso global:cp target/release/todo_list_rust_PLUS /usr/local/bin/



O aplicativo cria um diretório todo_db na sua pasta inicial para armazenar o banco de dados SQLite.
Uso
Execute o aplicativo usando:
todo_list_rust_PLUS <COMANDO>

Comandos Disponíveis

add <TAREFA>: Adicionar uma nova tarefa.Exemplo: todo_list_rust_PLUS add "Comprar mantimentos"
list: Exibir todas as tarefas.Exemplo: todo_list_rust_PLUS list
toggle <ID>: Alternar o status de uma tarefa (concluída/pendente).Exemplo: todo_list_rust_PLUS toggle 1
rm <ID>: Remover uma tarefa pelo seu ID.Exemplo: todo_list_rust_PLUS rm 1
sort: Ordenar tarefas por status de conclusão.Exemplo: todo_list_rust_PLUS sort
reset: Excluir todas as tarefas.Exemplo: todo_list_rust_PLUS reset

Exemplo
$ todo_list_rust_PLUS add "Escrever README"
$ todo_list_rust_PLUS add "Participar de reunião"
$ todo_list_rust_PLUS list
1. Escrever README [Pendente]
2. Participar de reunião [Pendente]
$ todo_list_rust_PLUS toggle 1
$ todo_list_rust_PLUS list
1. Escrever README [Concluída]
2. Participar de reunião [Pendente]
$ todo_list_rust_PLUS rm 2
$ todo_list_rust_PLUS list
1. Escrever README [Concluída]

Desenvolvimento
Executando Testes
Para executar a suíte de testes:
cargo test

Compilando para Outras Plataformas
Para compilar para Windows, certifique-se de que o alvo do Windows esteja instalado:
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu

Contribuição
Contribuições são bem-vindas! Siga estes passos:

Faça um fork do repositório.
Crie uma nova branch (git checkout -b branch-de-recurso).
Faça suas alterações e commit (git commit -m "Adicionar recurso").
Envie para a branch (git push origin branch-de-recurso).
Abra um Pull Request.

Licença
Este projeto está licenciado sob a Licença MIT. Veja o arquivo LICENSE para detalhes.
Agradecimentos
Inspirado em tutoriais de CLI do Rust e na comunidade Rust. Agradecimentos especiais aos autores de projetos semelhantes por fornecerem recursos de aprendizado.
