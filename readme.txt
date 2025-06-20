Calcul de la probabilité de gagner pour le jeu "mon premier verger",
par programmation dynamique.

Plateau :
- 4 arbres avec chacun leur couleur et dessus 4 fruits de cette couleur
- un chemin de 5 cases et un corbeau devant le chemin (avant la 1re case)

À chaque tour on lance le dé, s'il tombe sur :
- l'un des 4 faces de couleur : on enlève un fruit de la couleur correspondante (et on le met dans le panier)
- la face joker : on choisit la couleur que l'on veut
- la face corbeau : on avance le corbeau sur le chemin
Si on a ramassé tous les fruits on a gagné, si le corbeau sort du chemin (arrive dans le verger, donc a avancé 6 fois) on a perdu.

Dernière ligne de result.csv: on a 76% de chances de gagner. 63% (resp 45%) avec une (resp 2) case de moins pour le corbeau.
Dernière ligne de result_worst-choice.csv: on a 70% de chances de gagner si on prend dans l'arbre le moins rempli plutôt que le plus rempli.



Compiler et lancer le programme par la commande `cargo run > foo.csv`.