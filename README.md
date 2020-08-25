## 📒 📈 Portifolio

![checks](https://img.shields.io/github/workflow/status/lsunsi/portifolio/checks?style=flat-square)
![version](https://img.shields.io/github/v/release/lsunsi/portifolio?style=flat-square)

Acompanhamento de investimentos para seres humanos.

Inicialmente vai funcionar só para meus ativos e minha carteira, _porque né._

#### Direcionamento

###### Sobre features

- Acompanhamento de patrimônio e ganhos
- Comparação com benchmarks populares
- Quebras por ativo, período e por tags
- Aproximação de imposto de renda devido
- Projeções por previsões oficiais de juros

###### Sobre valores

- Correteza das informações, priorizando sempre dados absolutos e de valor auto-demonstrável (para seres humanos)
- Clareza nas informações, deixando sempre visível as limitações dos cálculos assim como as extrapolações
- Performance de uso, priorizando técnicas e tecnologias eficientes e não deixando segundos na mesa sem um bom motivo

###### Sobre ativos

- ETFs (todos, mas priorizando os meus atuais)
- Fundos (todos, mas priorizando os que eu já tive)
- Crypto (todos, mas priorizando as mais populares)
- Títulos privados (todos, priorizando os mais comuns)
- Títulos públicos (todos, porque é fácil)

#### Acompanhamento

O andamento e planejamentos do projeto pode ser acompanhado através [deste board](https://github.com/lsunsi/portifolio/projects/5).
A intenção é usar cards simples como forma de organizar e persistir o estado do trabalho, mas no geral as regras
de crianção e manutenção dos cards são muito relaxadas pra extrair muita previsibilidade.

#### Desenvolvimento

O ambiente de desenvolvimento foi pensado priorizando a performance do software e do desenvolvedor. Para ver o sistema em pé localmente, o server e o client precisam ser inicializados.

As dependências do servidor são agrupadas em um pod [(gerenciado pelo Podman)](https://developers.redhat.com/blog/2019/01/15/podman-managing-containers-pods/), que é automaticamente iniciado pelo script [dev/start](server/dev/start). O server, por sua vez, é compilado e inicado pelo [cargo](https://github.com/rust-lang/cargo).

```
# TLDR: You need podman and cargo installed, then
$ cd server/
$ ./dev/start
```

Para o client não temos nenhum shenanigan atualmente. É um projeto [next](https://nextjs.org/) executado como qualquer outro.

```
# TLDR: You need npm installed, then
$ cd client/
$ npm install && npx next
```

Nesse ponto você já teria o server e o client rodando e comunicando normalmente. Configurações posteriores seriam application-level, como importar preços ou portfolios. Escrevo sobre isso em um momento mais estável.

#### Agradecimento

Obrigado por ter lido até aqui, faz parecer que ficar escrevendo essas coisas e falando sozinho é justificado.

###### 📒 📈
