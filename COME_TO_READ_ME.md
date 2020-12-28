## Project review

# Project goal

le but du projet était de générer a partir de 1 fichier texte de question et réponse des qcm randomizer, dont le nom du projet.

# Previous project

Il y a eu des tentative de projet différent précédement flappy bird qui m'interessais plus et paint qui ma donné un mal de crane a cause de la crate, j'ai donc décider de ce projet comme réel projet les autre ne sont pas compté.(pas les perme de delete).
# How it works 

le programme va générer un nombre voulu de qcm tous randomizer que ce soit au niveau des réponses et des questions.
il y a à fournir simplement un fichier txt avec une légère syntaxe à respecter qui est la suivante:

question sur la premiere ligne
answers:-rep 1 -rep2 ...

il faut commencer les réponses avec "answers:" et chaque réponse doit avoir un "-"

ATTENTION:n'utiliser pas le mot clé et le "-" dans votre réponse qui fera bugé le programme et donnera des réponses mal faite.

# Used crate

- rand pour randomizer.
- pdf-canvas car j'ai trouver son utilisation plus simple que d'autre.
- anyhow pour du error handling.

# Coding steps

- récupération des informations dans le fichiers texte.
- on randomize les questions-réponses.
- création des pdfs {
 - set de la taille comme format A4.
 - set nom date.
 - on split les réponses des questions.
 - on retourne les questiond randomizer avec randomize answers.
 - si la phrase est trop long on la coup dans un vec.
 - on utilise la fonction blank_space pour couper dans un espace vide.
 - on print le contenu dans notre pdf tant que on en a encore et si il faut on créée d'autre page. 
}

# Difficult part 

la plus dure des parties était surement de séparer les phrases si elle étaient trop long pour que elle ne dépassent pas du cadre, juste cette partie a du me prendre des jours qui marche maintenant après 5 ou 7 version de test.

randomizer les réponses me demandais de d'abort ne pas les découpé avant de randomizer car sinon on ne sait plus quoi réponse ni question, pour les récupérer on a besoin de les marquer dans le fichier texte je ne vois pas d'autre facon de les trouver pour l'ordinateur.

il y a eu quelque autre moment ou cela prennaient un peu de temps mais au point d'etre un point difficile.

# Possible upgrade 

Une interface minimal pour donner un fichier et combien de fichiers on veut ou alors une plus long avec l'écriture des questions et des réponses.(j'aime pas vraiment les lib GUI de rust je trouve pas la docu assez expliquer et compliquer sans ca m'aurais prit du temps et j'avais d'autre projet derrière).

une formule pour calculer a combien de caractère pour bien faire le retour à la ligne selon la taille des caractère.

# Conclusion

Le projet ma apprit que écrire dans le pdf c'était pas si simple (je pensais que les retours à la ligne était fais tous seul mais pas du tout), utilisation de librairie (pas les plus grand surement mais quand un bon point), overall c'était sympathique peut etre que je l'utiliserais meme plus tard ou comme base d'un programme de randomization.
Merci de la lecture et peace moi j'ai des projets à finir :').