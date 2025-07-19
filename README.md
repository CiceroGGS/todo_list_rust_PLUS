# Lista de Tarefas Rust PLUS

<div align="center">
  <strong>Sua lista de tarefas, direto no terminal. Rápida, eficiente e feita em Rust. 🦀</strong>
</div>

<p align="center">
  <img alt="Linguagem" src="https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white">
  <img alt="Licença" src="https://img.shields.io/github/license/CiceroGGS/todo_list_rust_PLUS?style=for-the-badge&color=blue">
  <a href="https://github.com/CiceroGGS/todo_list_rust_PLUS/actions/workflows/rust.yml">
    <img alt="Status da Build" src="https://img.shields.io/github/actions/workflow/status/CiceroGGS/todo_list_rust_PLUS/rust.yml?branch=main&style=for-the-badge">
  </a>
</p>

Um aplicativo de linha de comando (CLI) simples e eficiente escrito em Rust, projetado para gerenciar tarefas com persistência de dados local em arquivos.

---

### 📋 Índice

* [Demonstração](#-demonstração)
* [Funcionalidades Principais](#-funcionalidades-principais)
* [Instalação e Configuração](#-instalação-e-configuração)
* [Como Usar](#️-como-usar)
* [Desenvolvimento](#-desenvolvimento)
* [Como Contribuir](#-como-contribuir)
* [Autor](#-autor)

---

### 📸 Demonstração

*(Dica: Você pode usar ferramentas como [termtosvg](https://github.com/nbedos/termtosvg) ou [Asciinema](https://asciinema.org/) para gravar um GIF do seu terminal e substituir a imagem abaixo. Isso deixa o projeto muito mais profissional!)*

![Demonstração do App](https://i.imgur.com/r7qfA5o.png) 
*(Exemplo de imagem da sessão do terminal)*

---

### ✨ Funcionalidades Principais

* **➕ Adicionar Tarefas**: Crie novas tarefas com descrições detalhadas.
* **📋 Listar Tarefas**: Visualize todas as tarefas, com opções para filtrar por status (pendente ou concluída).
* **✔️ Marcar Tarefas**: Alterne o status de uma tarefa entre concluída ou pendente.
* **🗑️ Remover Tarefas**: Exclua tarefas de forma segura pelo seu ID.
* **🔀 Ordenar Tarefas**: Organize a visualização das tarefas por status de conclusão.
* **🔄 Resetar Tudo**: Apague todas as tarefas para começar do zero.
* **💾 Armazenamento Persistente**: Salva seus dados localmente para garantir que não se percam entre as sessões.

---

### 🚀 Instalação e Configuração

Siga os passos abaixo para ter o `todo_list_rust_PLUS` funcionando em sua máquina.

#### Pré-requisitos
* [**Rust**](https://www.rust-lang.org/tools/install) (toolchain completa com `rustup` e `cargo`).

#### Passos de Instalação

1.  **Clone o repositório:**
    ```bash
    git clone [https://github.com/CiceroGGS/todo_list_rust_PLUS.git](https://github.com/CiceroGGS/todo_list_rust_PLUS.git)
    ```

2.  **Navegue até o diretório:**
    ```bash
    cd todo_list_rust_PLUS
    ```

3.  **Compile o projeto em modo de release (otimizado):**
    ```bash
    cargo build --release
    ```

4.  **(Opcional) Torne o comando acessível globalmente:**
    ```bash
    # Para Linux/macOS
    sudo cp target/release/todo_list_rust_PLUS /usr/local/bin/todo
    
    # Agora você pode usar apenas 'todo' em vez de 'todo_list_rust_PLUS'
    ```

> O aplicativo cria os arquivos necessários em sua pasta de usuário (`$HOME`) para armazenar as tarefas.

---

### ⌨️ Como Usar

Execute o aplicativo usando: `todo <COMANDO>`

#### Comandos Disponíveis

| Comando             | Descrição                                 | Exemplo                                 |
| ------------------- | ----------------------------------------- | --------------------------------------- |
| `add <"TAREFA">`    | Adiciona uma nova tarefa.                 | `todo add "Comprar pão"`                |
| `list`              | Exibe todas as tarefas.                   | `todo list`                             |
| `toggle <ID>`       | Alterna o status de uma tarefa.           | `todo toggle 1`                         |
| `rm <ID>`           | Remove uma tarefa pelo seu ID.            | `todo rm 3`                             |
| `sort`              | Ordena as tarefas por status.             | `todo sort`                             |
| `reset`             | **CUIDADO:** Exclui todas as tarefas.     | `todo reset`                            |

---

### 🛠️ Desenvolvimento

* **Executar os testes:**
    ```bash
    cargo test
    ```

* **Compilar para outras plataformas (ex: Windows):**
    ```bash
    rustup target add x86_64-pc-windows-gnu
    cargo build --release --target x86_64-pc-windows-gnu
    ```

---

### 🤝 Como Contribuir

Contribuições são sempre bem-vindas! Sinta-se à vontade para abrir *issues* ou enviar *pull requests*.

1.  Faça um **Fork** do projeto.
2.  Crie uma nova branch (`git checkout -b feature/minha-feature`).
3.  Faça o commit de suas alterações (`git commit -m 'Adiciona minha feature'`).
4.  Envie para a sua branch (`git push origin feature/minha-feature`).
5.  Abra um **Pull Request**.

---

### 📝 Licença

Este projeto está licenciado sob a **Licença MIT**. Veja o arquivo `LICENSE` para mais detalhes.

---

### 👨‍💻 Autor

Feito com ❤️ por **Cícero Guilherme**.

[![LinkedIn](https://img.shields.io/badge/linkedin-%230077B5.svg?style=for-the-badge&logo=linkedin&logoColor=white)](https://www.linkedin.com/in/cicero-guilherme-a9473a260/)
[![GitHub](https://img.shields.io/badge/github-%23121011.svg?style=for-the-badge&logo=github&logoColor=white)](https://github.com/CiceroGGS/)
