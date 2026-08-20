# Carteira de Investimentos - Rust Fullstack

Uma aplicação fullstack para gerenciamento de carteira de investimentos, construída com Rust no backend e templates Askama no frontend.

## 📋 O que o projeto faz

Esta aplicação permite que usuários gerenciem seus ativos de investimento através de uma interface web moderna e responsiva. As principais funcionalidades incluem:

- **Autenticação de usuários**: Login e registro com JWT tokens armazenados em cookies HttpOnly
- **Dashboard interativo**: Visualização completa da carteira com resumo do patrimônio
- **Gerenciamento de ativos**: Adicionar, editar e remover ativos (ações, criptomoedas, ETFs, etc.)
- **Cálculos automáticos**: Valor total por ativo, percentual na carteira e patrimônio consolidado
- **Interface moderna**: Design dark mode com Tailwind CSS e interatividade via HTMX

## 🚀 Como executar a aplicação

### Pré-requisitos

- **Rust** (versão 1.75+)
- **PostgreSQL** (versão 14+)
- **Docker** (opcional, para rodar o banco via container)

### 1. Clone o repositório

```bash
git clone <url-do-repositorio>
cd rust-fullstack-carteira-investimentos
```

### 2. Configure o banco de dados

#### Opção A: Docker (recomendado)

```bash
docker compose up -d
```

#### Opção B: PostgreSQL local

Certifique-se de ter um banco PostgreSQL rodando e crie o banco `postgres` com usuário `postgres` e senha `postgres`.

### 3. Configure as variáveis de ambiente

Crie um arquivo `.env` na raiz do projeto:

```env
DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres
```

### 4. Execute as migrações

```bash
sqlx migrate run
```

> **Nota**: Se não tiver o `sqlx-cli` instalado: `cargo install sqlx-cli`

### 5. Compile e execute

```bash
cargo run
```

A aplicação estará disponível em: **http://localhost:3000**

## 🛠️ Tecnologias utilizadas

### Backend
| Tecnologia | Versão | Descrição |
|------------|--------|-----------|
| **Rust** | 2024 Edition | Linguagem de programação principal |
| **Axum** | 0.8 | Framework web assíncrono |
| **SQLx** | 0.8 | Toolkit SQL assíncrono com verificação em tempo de compilação |
| **PostgreSQL** | 14+ | Banco de dados relacional |
| **JWT Simple** | 0.12 | Autenticação com tokens JWT |
| **Password Auth** | 1.0 | Hash seguro de senhas (Argon2) |
| **Askama** | 0.15 | Templates HTML type-safe em Rust |
| **Tokio** | 1.50 | Runtime assíncrono |
| **Tracing** | 0.1 | Logging e instrumentação |

### Frontend
| Tecnologia | Descrição |
|------------|-----------|
| **Tailwind CSS** | Framework CSS utility-first (via CDN) |
| **HTMX** | Interatividade sem JavaScript complexo |
| **Space Mono** | Fonte monoespaçada para visualização de dados |

### Ferramentas de Desenvolvimento
- **Insta** - Snapshot testing para APIs
- **Color-Eyre** - Error handling aprimorado
- **ThisError** - Derive macros para erros

## ✨ Melhorias implementadas

### 1. Dashboard Completo com Visualização de Portfólio
- **Resumo do patrimônio**: Cards com valor total, quantidade de ativos e ativos com posição
- **Tabela de ativos**: Lista completa com colunas para nome, quantidade, valor unitário, valor total e % da carteira
- **Cálculos em tempo real**: Valores totais e percentuais calculados dinamicamente no frontend

### 2. Gerenciamento de Quantidade por Ativo
- Adicionado campo `quantity` na model `Asset` e tabela `assets`
- Migração de banco para suportar posições fracionárias (precisão de 8 casas decimais)
- Suporte a criptomoedas e ativos fracionados

### 3. CRUD Completo via Interface Web
- **Create**: Modal para adicionar novos ativos com validação
- **Read**: Listagem em tabela responsiva com ordenação
- **Update**: Modal de edição pré-preenchido com dados atuais
- **Delete**: Exclusão com confirmação via HTMX

### 4. Autenticação Melhorada
- Redirecionamento automático para dashboard após login
- Registro automático de novos usuários (auto-signup)
- Proteção de rotas admin para operações de escrita

### 5. Experiência do Usuário (UX)
- Design dark mode profissional com paleta cyan/slate
- Animações e transições suaves
- Feedback visual em hover/focus
- Modais acessíveis com foco automático
- Formatação de moeda brasileira (R$) e números

### 6. Arquitetura Limpa
- Separação clara: `models`, `repository`, `routes`, `auth`, `error`
- Extractor pattern para `Repository`, `User`, `Admin`
- Error handling unificado com `AppError` e `IntoResponse`

## 🧪 Como testar

### Testes Automatizados

```bash
# Executa todos os testes (requer banco de dados de teste)
cargo test

# Testes específicos da API
cargo test test_create_asset
cargo test test_list_assets
cargo test test_update_asset
```

Os testes usam `sqlx::test` com fixtures e `insta` para snapshot testing.

### Testes Manuais

1. **Acesse** http://localhost:3000
2. **Faça login** com qualquer usuário/senha (cria conta automaticamente)
3. **No dashboard**:
   - Clique em "Novo Ativo" para adicionar
   - Preencha: Nome (ex: "Bitcoin"), Valor Unitário (ex: 300000), Quantidade (ex: 0.5)
   - Veja o resumo atualizar automaticamente
   - Clique no ícone de editar (lápis) para alterar
   - Clique no ícone de lixeira para excluir (com confirmação)

### Endpoints da API

| Método | Endpoint | Descrição | Auth |
|--------|----------|-----------|------|
| GET | `/api/assets` | Lista todos os ativos | User |
| POST | `/api/assets` | Cria novo ativo | Admin |
| PATCH | `/api/assets/{id}` | Atualiza ativo | Admin |
| DELETE | `/api/assets/{id}` | Remove ativo | Admin |

**Autenticação Admin**: Header `Authorization: im-the-admin`

### Exemplo via cURL

```bash
# Listar ativos (precisa de cookie de sessão)
curl -b cookies.txt http://localhost:3000/api/assets

# Criar ativo (admin)
curl -X POST http://localhost:3000/api/assets \
  -H "Content-Type: application/json" \
  -H "Authorization: im-the-admin" \
  -d '{"name": "Bitcoin", "unit_value": 300000, "quantity": 0.5}'

# Atualizar ativo
curl -X PATCH http://localhost:3000/api/assets/1 \
  -H "Content-Type: application/json" \
  -H "Authorization: im-the-admin" \
  -d '{"quantity": 1.0}'

# Deletar ativo
curl -X DELETE http://localhost:3000/api/assets/1 \
  -H "Authorization: im-the-admin"
```

## 📚 O que aprendi durante o desafio

### Rust & Ecossistema
- **Axum extractors**: Como criar extractors customizados (`FromRequestParts`) para injeção de dependências limpa (Repository, User, Admin)
- **SQLx compile-time queries**: Verificação de SQL em tempo de compilação previne erros de runtime
- **Askama templates**: Templates type-safe que falham em compilação se houver erros de HTML/ variáveis
- **State management**: Uso de `AppState` com `PgPool` clonado para conexões de banco compartilhadas

### Arquitetura Fullstack Rust
- **Separação de responsabilidades**: Models, Repository, Routes, Auth desacoplados
- **Error handling unificado**: `thiserror` + `IntoResponse` para erros HTTP consistentes
- **Authentication patterns**: JWT em cookies HttpOnly + extractor para usuário opcional/obrigatório

### Frontend Moderno com Pouco JS
- **HTMX**: Interatividade declarativa (hx-get, hx-post, hx-patch, hx-delete) sem escrever JavaScript manual
- **Template partials**: Modais como templates separados renderizados server-side
- **Progressive enhancement**: Funciona mesmo sem JS (fallback para forms tradicionais)

### Banco de Dados & Migrações
- **Migrações versionadas**: Controle de schema com up/down.sql
- **Fixtures de teste**: SQLx fixtures para dados de teste consistentes
- **Tipos numéricos**: `DOUBLE PRECISION` para valores monetários com precisão adequada

### Boas Práticas
- **Type-driven development**: Tipos Rust guiam o design da API
- **Security defaults**: Cookies HttpOnly, senhas com Argon2, validação de entrada
- **Observability**: Tracing spans nas rotas da API para debugging

---

## 📁 Estrutura do Projeto

```
├── Cargo.toml
├── compose.yml              # Docker Compose para PostgreSQL
├── .env                     # Variáveis de ambiente
├── migrations/              # Migrações SQL versionadas
│   ├── 20260328192535_create_assets.up/down.sql
│   ├── 20260329160020_create_users.up/down.sql
│   └── 20260820000000_add_quantity_to_assets.up/down.sql
├── src/
│   ├── main.rs              # Entry point
│   ├── app.rs               # Configuração da aplicação e estado
│   ├── models.rs            # Structs de domínio (Asset, PortfolioSummary)
│   ├── repository.rs        # Camada de acesso a dados
│   ├── error.rs             # Tipos de erro unificados
│   ├── auth/
│   │   ├── mod.rs
│   │   ├── user.rs          # Autenticação JWT + cookies
│   │   └── admin.rs         # Auth simples para admin
│   └── routes/
│       ├── mod.rs
│       ├── frontend.rs      # Rotas HTML (login, dashboard, modais)
│       └── api.rs           # Rotas REST (/api/assets)
└── templates/
    ├── login.html           # Página de login
    ├── dashboard.html       # Dashboard principal
    ├── new_asset_modal.html # Modal novo ativo
    └── edit_asset_modal.html # Modal editar ativo
```

## 📄 Licença

Este projeto é open source e está disponível sob a licença MIT.