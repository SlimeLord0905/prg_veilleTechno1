<div align="center">

<br/>

**Mathieu Mercier**

<br/><br/>

**Recherche Veille technologique**

**Qu'est ce qui fait la popularité du Rust?**

<br/><br/>

**Travail remis à**

**Nicolas Bourré**

<br/><br/>

**10 juin 2023**

**Collège Shawinigan**

<br/><br/>

</div>

****


## **Table des Matières**
<br/><br/>


## **Introduction**
<br/><br/>

<img src=https://www.rustacean.net/assets/cuddlyferris.svg style="width:70%">


## **Dévellopement**
<br/>

### **Qu'est ce que c'est le Rust?**
<br/>
Le Rust est un langage de programmation dit "de niveau système", ce qui signifie qu'il est conçu pour une programmation plus précise et plus proche du système que des langages tels que le C# ou le Java, qui ont une portée plus large et sont généralement utilisés pour des applications nécessitant moins de précision et d'économie de capacité. Rust est très léger à compiler. En effet, comme ses prédécesseurs, le C et le C++, Rust utilise peu de ressources et est donc bien adapté aux applications nécessitant des économies de processeur et de mémoire.
<br/><br/>

### **Méthodologie**
<br/>
La recherche a commencé par une expérimentation de Rust afin d'avoir une meilleure idée de ce qu'est ce langage et de la disponibilité des informations nécessaires pour l'apprendre. J'ai donc réalisé un simple jeu de la vie en Rust qui s'affiche dans une fenêtre et peut être réinitialisé en appuyant sur la touche R.
<img src=https://gyazo.com/b3b370f29c576c8916c9e673aac591af style="width:70%"> Ensuite, une recherche en ligne a été effectuée. Pour mener cette recherche, de nombreux articles ont été lus et les opinions des auteurs ont été compilées avant d'être classées afin de dégager les idées générales de ce que les programmeurs aiment et n'aiment pas.
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
<br/>
En programmation, il existe un principe appelé le "multithreading", qui consiste à exécuter plusieurs opérations d'un programme en parallèle pour gagner en vitesse et en efficacité. L'utilisation de plusieurs processus simultanément a longtemps posé un problème : la compétition pour l'utilisation de la mémoire, où deux processus souhaitent utiliser la même zone mémoire et se font donc concurrence pour son utilisation.

Le Rust offre une solution à ce problème, ce qui le rend beaucoup plus sécurisé à utiliser pour ce type de situation. En effet, cette sécurité accrue qui prévient les conflits rend les programmes utilisant le multithreading beaucoup plus stables et moins risqués à utiliser, ce qui plaît énormément aux programmeurs ayant besoin de mettre en œuvre du multithreading.
<br/><br/>

### **Les points négatifs**
<br/>
Cette section est un résumé de ce que les programmeurs semblent ne pas aimer du langage Rust.
<br/><br/>

#### **Le rust n'est pas orienter objet**

#### **Complexité**

### **Résultats**
<br/>

## **Conclusion**
<br/><br/>


## **Médiagraphie**
<br/><br/>
