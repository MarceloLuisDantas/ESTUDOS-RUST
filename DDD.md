# :dromedary_camel: DDD (Diario Do ~~-Dev-~~ Dromedario)

__PS:__ _Tenho dislexia, e este diario é apenas para eu ter onde dizer o que eu penso sobre meus estudos e coisas envolvendo isso, então pfv n se importe com os erros de portugues._

<!-- <details>
  <summary>DD/05/2022</summary>

  <hr>
  Fim do dia:
  
  <hr>
  :headphones: Musica:

<details> -->

<details>
  <summary>16/04/2022</summary>
  Depois de desistir mais uma vez do projeto de criar um RPG de terminal por encontror um detalhe que me incomoda no NCurses (Cores muito saturadas, podia ter ido ver outra lib? Sim, fiz isso? kk não) Descidi trazer o DDD para o projeto que quase sempre dou uma cutucada. Seja para testar algo ou para fazer algo par enviar em algum rep onde n posso usar o cargo por seja la qual for o motivo. Então acho que aqui seria o melhor lugar para fazer o DDD. É isso.
  <hr>
  Final do dia (Ou começo, to escrevendo isso as 1:48), hj eu tentei fazer Quick Sort acreditando por memoria que seria oa mais simples dentre os algoritmos uteis (com "uteis" eu me refiro coisas diferente de Bogo e Gravity sort por exemplo, umas coisa n mene) Porem n sei se foi combinação de Sono + Ser Burro, mas eu entendi e ao mesmo tempo não. Eu to indo dormir para ver se amanha eu entendo melhor o que ta acontecendo. E caso realmente n consiga, eu tento algum outro algoritmo mais facil.
  <hr>
  :headphones: Musica: Rhapsody Of Fire - the Wind, the Rain and the Moon 
</details>

<details>
  <summary>17/04/2022</summary>
  Então, eu com a minha mania de arrumar coisas uteis par procastinar sem peso na conciencia, descidi mudar o Readme para uma explicação do pq o Rust existe e o que ele faz para ter destaque em relação a outras linguagens. Vai ser util? Sim, porem n passa
  de um modo de procastinar sem ser julgado pela sociedade.
  <hr>
  Fim do dia:
  <hr>
  :headphones: Musica: Asriel: Abyss 
</details>

<details>
  <summary>19/04/2022</summary>
  Então, finalmente depois de usn 3 ou 4 dias aceitaram minha contribuição e um proejto e consegui destravar par acontinuar aqui. Então vamo votlar a tentar o Quick Sort ou desistir e ir para um merge sort.
  Então meio que eu consegui fazer o Quick funcionar e entender como ele funciona. Foi bem chato conseguir visualizar ele funcionado na minha mente direitrinho msa eu consegui. Vou tentar fazer o merge sort provavelmente (se não tiverem implementado no projeto claro)
  <hr>
  :headphones: Musica: Rapsody of Fire - Dawn of Victory
</details>

<details>
  <summary>19/04/2022</summary>
  Esqueci qeu vc existia e comecei a fazer umas coisa sem escrever, então vim me coisa sobre isso. Então eu pensei "caralho vou deixar pra amanha o merge sort" e de fato eu podia fazer agora, porem eu fiquei com preguiça pq parece ser muito chato de implementar. Então eu descidi fazer o "soma_dois_numeros.rs". Pq parecia facil, mas vou precisar aprender a usar HashMap, o que parece ser facil, mas é algo que eu nunca mechi com, então bora.
  Então, eu pensei ter terminado o codigo, quando eu fui testar e ele tava sempre dando o mesmo resultado. E eu fiquei nisso ate eu jogar um input que iria estourar i32 e perceber que não deu erro pq eu tava usando o mesmo binario desde a primeira compilação, é claro que não ia mudar o output.
  <hr>
  :headphones: Musica: Border of Life - TouHou 7
</details>

<details>
  <summary>26/04/2022</summary>
  Pois é fiquei muito tempo sem tocar aqui, motivos? Voltei a jogar SW, porem isso n vem ao caso. Hj eu to afim de fazer lista linkada, vai dar certo? Acho que sim, n é muito dificil. Porem vai ser muito util entender direito como elas funcionam, por mais que elas por si sejam meio inuteis.
</details>

<details>
  <summary>1/05/2022</summary>
  Sim eu sei, eu to piscando nisso com a mesma frequencia que o sol completa 1 ano na via lactia, porem isso n vem ao caso. Continuo tentando entender Linked List, e para isso preciso entender Box, Rc e essas coisas de gerenciamento de ponteiros inteligentes. Espero conseguir fazer mais que 2 linhas hj.
  Tecnica compilou, eu entendi um pouco das coisa, porem na minha cabeça o push ta colocando no começo da lista e não no final, preciso entender melho isso antes de seguir.
  Parei por agora na parte de impl o Drop. To confuso sobre o funcionamento do mem::replace e outras coisas. E eu tava certo, o push tava acontecendo no inicio e não no final. Isso é uma Stack não uma lista, depois eu penso em como inverter isso e fazer ele alucar o novo indice no final.
  <hr>
  :headphones: Musica: Orden Organ - Fields of Sorrow (essa me deixou F no chat)
</details>

<details>
  <summary>6/05/2022</summary>
  Eu não quero nem um comentario sobre minha demora.... ouviu? 
  Pois é, faculdade começou essa semana e eu fiquei o dia todo nela por causa do evento e chegava cansado por n estar acostuamdo com acordar cedo. Ai hj eu descidi centar o cu na cadeira e entender o que é um Smart Pointer, que é um ponteiro muito mais dificil de lidar com porem que evita o grande problema causado por algo que nunca se pode resolver, o programador ser burro.  
  <hr>
  :headphones: Musica: Powerwolf - Kreuzfeuer (eu n tinha reparado no quão bela é essa musica)
</details>

<details>
  <summary>7/05/2022</summary>
  GOOD MORNING MORIOH
  Mais um dia tentando entender ponteiros, na real é o mesmo dia, pois ontem eu so comecei ai fui dormir pq já era meia noite e eu tava com sono. Então hj eu vou ler mais o capitulo do livro sobre ponteiros para tentar entender alguma coisa, pq isso é mais complicado de entender do que usar, usar é ate facil, porem eu n quero ficar precisando compilar e ver o que o rustc tem a dizer sobre minhas cagada, então eu quero entender direito
  <hr>
  Fim do dia: 
  Eu consegui fazer a copisa funciona com push e pop, porem eu acredito que não. Pq eu tenho quase certeza que eu os valores estão apenas sendo liberados para s... vou testar isso, momento. Exato, eu acredito que eu esteja apenas "liberando" os valores para serem sobrescritos, logo eu preciso ver como eu faço para realmente liberar o espaço na moemoria, lembro que tem uma trait que faz isso, vou dar uma olha e lembro tbm do mem::replace, porem por hora vai ficar assim. Porem vou dar uma comentada.
  <hr>
  :headphones: Musica: Powerwolf - Incense and Iron
</details>

<details>
  <summary>8/05/2022</summary>
  3 dias seguindos, acho que é um Recorde pessoal. Bom dia, boa tarde, boa noite pra vc que ta lendo isso. Hoje eu descidi desligar um pouco de Smart Pointers, box, rc, pipipi popopo, e descidi dar uma olhada em algotimos, porem dessa vez eu quero brincar um pouco com Hash, fiquei curiosos sobre como eles funcionam. Então bora tentar alguma coisa.
</details>

<details>
  <summary>12/05/2022</summary>
  Eu tinha qeu abrir a minha boca sobre como eu tava conseguindo manter uma sequencia. O resultado do dia 8 foi em uma conversa no telegram sobre como implementar um hash n faz muito sentido, pois as operações são completamente random que não iriam melhorar em muita coisa a minha capacidade como programador. Então fiquei meio xernos e vou voltar a aquele interpretador, porem dessa vez vou so ignorar AST ou algo bem feito, foi apenas fazer funcionar e des-foda-se.
  <hr>
  Final do dia: Consegui fazer algumas coisas, seguindo a ideia do "antes feito do que inexistente" eu so fiz as coisas de forma mais simples possivel, sem AST, sem parser, sem porra nem uma, quem verifica a sintaxe é liretalmentre uma função, tudo é feito da forma mais simples possivel. Porem esta funcionando. Amanha eu tento fazer algo mais, porem por hora vai ficar como esta mesmo. To feliz de conseguir fazer algo.
  <hr>
  :headphones: Musica: Battle Beast - Master of Illusion
</details>

<details>
  <summary>13/05/2022</summary>
  Mais(eu demorei umas 5 tentativas para escrever isso) um dia tentando fazer algo. Ontem eu já peguei mais o costume de usar Options e Results. Foi bem util, eu acredito, são coisas muito uties. E como ontem eu criei as funções para criar variaveis e remover variaveis, hj eu quero tentar criar uma função para criar constantes, e as aritimetricas, para isso eu acredito que vou precisar criar um enum ou outra aestrutura que vai conter o valor, e vai dizer se é constante ou variavel. Bora tentar.
  <hr>
  Final do dia: Hj ate que fiz bastante coisa, fiz a parte da aritmetica, foi bem mais complicado do que eu pensava pq tomei umas descições no meio que me fez precisar arrumar boa parte do codigo, contando com os testes, e escrevi novos testes. Eu ia mecher com a parte de tornar os valores imutaveis, e aproveitar para permitir a criação de int, float e string, porem descidi ao menos escrever isso antes que que faça merda pq eu me conheço muito bem.
  Na real acho que vou so tentar comentar o codigo pq ta ficando grande e ir fazer qualquer outra coisa e continuar amanha.
  <hr>
  :headphones: Musica: Orden Organ - Vampire in the Ghost Town
</details>

<details>
  <summary>14/05/2022</summary>
  Coisas a fazer today. Implementar o que eu disse ontem de uma estrutura que vai dizer qual o tipo dos valores e se eles são mutaveis ou imutaveis. Bora. 
  Oi, consegui criar a estrutura para valores que aceita Str, Float e Int, porem agora preciso refatorar o codigo para passar tudo que era sobre f32 apenas para aceitar essa nova estrutura, incluindo os testes. KK EU VOU TRANCAR O CURSO.
  Ok ok, consegui fazer isso, porem agora vou precisar criar uma nova função mut para modificar variaveis. Porem agora vou subir tudo pro github e dar uma refatorada antes de voltar a futricar.
<details>

<details>
  <summary>16/05/2022</summary>
  Hj eu descidi implementar a o "set" para alterar os valores mutaveis. E para isso eu tive que implementar algumas outras coisas como type_of para ajudar nos testes e arrumar outras coisas.
  Agora eu to querendo refazer todos os codigos de testes pra deixar mais arrumado. E quero ver se eu consigo implementar o help para cada função e para um todo. Vou fazer isso hj? Não sei, mas pretendo.
  <hr>
  Fim do dia: Ok, para facilitar umas coisas e deixar mais organizado eu descidi criar um modulo apenas para os erros, assim fica mais facil de mudar algum erro e na hora de testar tbm. Isso vai me ajudar bastante na hora de deixar tudo mais minimamente agradavel aos olhos. Amanha eu tento re-escrever os testes de uma forma mais organizada.
  <hr>
  :headphones: Musica: 紺色のシンフォニック・スイート - 無邪気な狂気 笑顔で暴力
<details>

<details>
  <summary>18/05/2022</summary>
  Hj eu descidi deixar um pouco de lado o interpretador e fazer alguma outra coisa. Então depois de muito tempo pensando(3 segundos) eu pensei como seria legal criar um servidor FTP(Acho que HTTP vai ser mais facil) e mandar para fernanda o link via SHH ou algo assim para ela abrir no terminal um HTML e mandar um msg e aparecer no meu terminal. Então bora.
<details>

<details>
  <summary>20/05/2022</summary>
  Fiquei com vontade de mudar Rust Of Life do terminal para uma interface grafica, sem mais perguntas.
  <hr>
  Fim do dia:
  
  <hr>
  :headphones: Musica:

<details>