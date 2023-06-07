<div align="center">

<br/>

**Mathieu Mercier**

<br/><br/><br/><br/>

**Recherche Veille technologique**

**Qu'est ce qui fait la popularité du Rust?**

<br/><br/><br/><br/>

**Travail remis à**

**Nicolas Bourré**

<br/><br/><br/><br/>

**9 juin 2023**

**Collège Shawinigan**

<br/><br/>

</div>

****


## **Table des Matières**
<br/><br/>
- [**Table des Matières**](#table-des-matières)
- [**Introduction**](#introduction)
- [**Dévellopement**](#dévellopement)
  - [**Qu'est ce que c'est le Rust?**](#quest-ce-que-cest-le-rust)
  - [**Méthodologie**](#méthodologie)
  - [**Les points positifs**](#les-points-positifs)
    - [**Résistance au bug**](#résistance-au-bug)
    - [**Performance**](#performance)
    - [**Programmation à plusieur fils**](#programmation-à-plusieur-fils)
  - [**Les points négatifs**](#les-points-négatifs)
    - [**Le rust n'est pas orienter objet**](#le-rust-nest-pas-orienter-objet)
    - [**Complexité**](#complexité)
  - [**Résultats**](#résultats)
- [**Conclusion**](#conclusion)
- [**Médiagraphie**](#médiagraphie)


## **Introduction**
<br/>
L'informatique est un domaine en constante évolution, avec des changements rapides tant au niveau des capacités des ordinateurs que des langages utilisés pour les programmer. Récemment, un nouveau langage a connu une montée fulgurante en popularité : Rust. Ce texte vise à recueillir les opinions positives et négatives sur Rust afin de comprendre les raisons de sa popularité croissante.

<img src=https://www.rustacean.net/assets/cuddlyferris.svg style="width:70%">


## **Dévellopement**
<br/>

### **Qu'est ce que c'est le Rust?**
<br/>
Le Rust est un langage de programmation dit "de niveau système", ce qui signifie qu'il est conçu pour une programmation plus précise et plus proche du système que des langages tels que le C# ou le Java, qui ont une portée plus large et sont généralement utilisés pour des applications nécessitant moins de précision et d'économie de capacité. Rust est très léger à compiler. En effet, comme ses prédécesseurs, le C et le C++, Rust utilise peu de ressources et est donc bien adapté aux applications nécessitant des économies de processeur et de mémoire.
<br/><br/>

### **Méthodologie**
<br/>
La recherche a commencé par une expérimentation de Rust afin d'avoir une meilleure idée de ce qu'est ce langage et de la disponibilité des informations nécessaires pour l'apprendre. Un simple jeu de la vie a donc été réalisé en Rust qui s'affiche dans une fenêtre. Pour pousser les tests plus loin, une variation du jeu de la vie appelée SmoothLife a été développée.
<a href="https://gyazo.com/64f1b0a48b4b6f31e61c18d6feddde47"><img src="https://i.gyazo.com/64f1b0a48b4b6f31e61c18d6feddde47.gif" alt="Image from Gyazo" width="1280"/></a> Ensuite, une recherche en ligne a été effectuée. Pour mener cette recherche, de nombreux articles ont été lus et les opinions des auteurs ont été compilées avant d'être classées afin de dégager les idées générales de ce que les programmeurs aiment et n'aiment pas.
<br/><br/>

###  **Les points positifs**
<br/>
Cette section est un résumé de ce que les programmeurs semblent aimer du langage Rust.
<br/><br/>

#### **Résistance au bug**
Ce n'est un secret pour personne que des langages tels que le C sont sujets aux bugs, et lorsque ces derniers surviennent, il n'est généralement pas facile de les trouver. Parfois, le programme va simplement planter ou affiche un message dans la console ou le compilateur qui ne fournit que peu de détails sur la cause du problème ou son emplacement. Rust a essayé d'éviter autant que possible ce genre de problèmes grâce aux vérifications effectuées par le compilateur avant la compilation du programme.

En effet, le compilateur Rust vérifie toutes les opérations afin de s'assurer qu'aucun code dangereux ou simplement mal écrit ne passe inaperçu, pouvant causer un bug. De plus, lorsque qu'une erreur est détectée, le compilateur fournit des détails sur la cause de l'erreur, son emplacement et même propose une solution pour résoudre le problème. Certains pourraient dire que Rust limite la liberté du programmeur et que le fait que le compilateur soit strict pose problème. Cependant, pour les rares cas où le compilateur est trop sévère, il existe le "unsafe Rust" qui permet de compiler du code plus risqué.

#### **Performance**

Même s'il est plus lourd à compiler que le C, le langage Rust, avec un poids et une rapidité comparables au C++, est extrêmement rapide à compiler et léger par rapport à la vaste majorité des langages modernes. Il offre néanmoins de nombreux avantages en termes de confort lors de la compilation, ce qui le distingue de ses prédécesseurs.

Cette caractéristique le rend parfait pour la programmation de petits ordinateurs embarqués qui ont généralement des capacités limitées, ainsi que pour la création d'une "API" capable de gérer de grandes quantités de données sans surcharger l'appareil sur lequel elle est exécutée. 

Le Rust a également une nouvelle gestion de la mémoire qui est moins complexe et sujette aux bugs que celle de ses prédécesseurs, tout en étant plus légère que les "garbage collectors" présents dans de nombreux langages. Cela permet une optimisation plus simple de la mémoire.

La combinaison de confort moderne et d'excellentes performances est l'une des raisons pour lesquelles les programmeurs apprécient le langage Rust.

#### **Programmation à plusieur fils**
En programmation, il existe un principe appelé le "multithreading", qui consiste à exécuter plusieurs opérations d'un programme en parallèle pour gagner en vitesse et en efficacité. L'utilisation de plusieurs processus simultanément a longtemps posé un problème : la compétition pour l'utilisation de la mémoire, où deux processus souhaitent utiliser la même zone mémoire et se font donc concurrence pour son utilisation.

Le Rust offre une solution à ce problème, ce qui le rend beaucoup plus sécurisé à utiliser pour ce type de situation. En effet, cette sécurité accrue qui prévient les conflits rend les programmes utilisant le multithreading beaucoup plus stables et moins risqués à utiliser, ce qui plaît énormément aux programmeurs ayant besoin de mettre en œuvre du multithreading.
<br/><br/>

### **Les points négatifs**

Cette section est un résumé de ce que les programmeurs semblent ne pas aimer du langage Rust.
<br/><br/>

#### **Le rust n'est pas orienter objet**
Le langage Rust n'est pas spécifiquement conçu pour la programmation orientée objet. Bien qu'il soit possible de réaliser de la programmation orientée objet avec Rust, ce n'est pas sa principale orientation et il n'est pas conçu pour faciliter ce paradigme. L'approche orientée objet est actuellement la norme et la méthode de structuration de programme la plus répandue. Malheureusement, l'orientation objet n'est pas au cœur de la philosophie du langage Rust, ce qui peut être problématique pour de nombreux programmeurs.

#### **Complexité**
Le Rust n'est pas un langage simple. En effet, les langages de niveau système ont généralement la réputation d'être difficiles à utiliser et à maîtriser, et Rust ne fait pas exception. Les systèmes de gestion de la mémoire, bien que plus faciles à comprendre que ceux de langages tels que le C, restent complexes et nécessitent beaucoup de pratique avant de les maîtriser. De plus, même si le multithreading est une spécialité de Rust, son utilisation n'est toujours pas facile. Le développement avec des processus parallèles reste une tâche complexe et ardue, même si une fois le code compilé, il est beaucoup plus stable.
Le Rust ne peut en aucun cas être considéré comme un langage pour débutants. Même si un programmeur expérimenté trouvera plus facile de naviguer dans ce langage, il reste complexe, ce qui peut décourager certains programmeurs qui préfèrent des langages plus simples à utiliser.


### **Résultats**
<br/>
Le Rust présente de nombreux avantages par rapport aux langages similaires, tels que les systèmes intégrés dans le compilateur qui évitent de nombreuses erreurs imprévues, ses excellentes performances et la grande stabilité de ses processus parallèles. Cela le rend très populaire auprès des programmeurs qui ont besoin d'un langage de niveau système. Même s'il parvient à éviter de nombreux problèmes de ses prédécesseurs, le Rust reste un langage complexe et moins adapté à la programmation orientée objet. Malgré ses imperfections, le Rust offre suffisamment d'avantages par rapport à ses prédécesseurs pour justifier sa montée en popularité.

## **Conclusion**
<br/><br/>
Le Rust est un langage qui est plutôt récent, mais il a gagné en popularité grâce à sa philosophie d'offrir de nombreux conforts modernes sans compromettre les performances. Grâce à sa grande résistance aux bugs, ses performances et ses avantages en programmation multithread, des problèmes tels que la complexité du langage et l'absence d'orientation objet n'ont pas découragé les programmeurs qui restent intéressés par celui-ci. Peut-être que, dans un avenir plus ou moins proche, la popularité du Rust diminuera, mais pour le moment, il semble être là pour rester avec sa communauté de supporters qui continuent de le faire progresser.

## **Médiagraphie**
<br/><br/>
The Rust Programming Language. (s.d.). Récupéré le 7 juin 2023, de https://www.rust-lang.org/

Goh, S. (2020, 20 janvier). What is Rust and why is it so popular? - Stack Overflow Blog. Stack Overflow Blog. https://stackoverflow.blog/2020/01/20/what-is-rust-and-why-is-it-so-popular/

What Is Rust and How It Works - An Overview and Its Use Cases. (s.d.). DevOpsSchool.com. Récupéré le 7 juin 2023, de https://www.devopsschool.com/blog/what-is-rust-and-how-it-works-an-overview-and-its-use-cases/

Mittal, C. (2023, 4 février). 10 Best Use Cases of Rust Programming Language in 2023. DEV Community. https://dev.to/chetanmittaldev/10-best-use-cases-of-rust-programming-language-in-2023-20jk

Clicks, T. (s.d.). 9 Advantages of Programming in Rust. DEV Community. Récupéré le 7 juin 2023, de https://dev.to/timclicks/9-advantages-of-programming-in-rust-22m0

Stutee. (s.d.). Rust Langage de programmation : les avantages et les inconvénients. IONOS Digital Guide. Récupéré le 7 juin 2023, de https://www.ionos.fr/digitalguide/sites-internet/developpement-web/rust-langage-de-programmation/

Carr, J. (2021, 19 janvier). Rust programming: Benefits, frameworks, and more. Simple Programmer. https://simpleprogrammer.com/rust-programming-benefits/

Fink, R. (2021, 29 décembre). 7 reasons to love the Rust language — and 7 reasons not to. InfoWorld. https://www.infoworld.com/article/3675391/7-reasons-to-love-the-rust-language-and-7-reasons-not-to.html

Rust Programming Language. (s.d.). Lefttronic. Récupéré le 7 juin 2023, de https://leftronic.com/blog/rust-programming-language/

Baaz, K. (2023, 17 avril). Why is Rust the Most Popular Programming Language? Turing. https://www.turing.com/blog/rust-is-the-most-popular-programming-language/

Rust Guide: How to Learn Rust in 2023. (s.d.). Serokell. Récupéré le 7 juin 2023, de https://serokell.io/blog/rust-guide
