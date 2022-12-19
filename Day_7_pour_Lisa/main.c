#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct Fichier{
    char name[30];
    unsigned int taille;
    unsigned int depth;
    struct Dossier *parent;
};
struct Dossier {
    char name[30];
    unsigned int taille;
    unsigned int depth;
    struct Dossier *parent;
    struct Dossier **dossiers;
    unsigned int nb_dossier;
    struct Fichier **fichiers;
    unsigned int nb_fichiers;
};

// Création d'un dossier vide
// On l'ajoutera au filesystem plus tard
struct Dossier * nouveau_dossier(char *name){
    struct Dossier *n_doss = malloc(sizeof(struct Dossier));
    strcpy(n_doss->name, name);
    n_doss->name[strlen(name)-1] = '\0';
    n_doss->depth = 0;
    n_doss->taille = 0;
    n_doss->parent = NULL;
    n_doss->dossiers = malloc(sizeof(struct Dossier *));
    n_doss->dossiers = 0;
    n_doss->fichiers = malloc(sizeof(struct Fichier *));
    n_doss->fichiers = 0;

    return n_doss;
}

// Création d'un fichier vide
// On l'ajoutera au filesystem plus tard
struct Fichier * nouveau_fichier(char *name, char *size){
    long taille = strtol(size,NULL, 10);
    struct Fichier *n_fichier = malloc(sizeof(struct Fichier));
    name[strlen(name)-1] = '\0';
    strcpy(n_fichier->name, name);
    n_fichier->depth = 0;
    n_fichier->taille = (unsigned int)taille;
    n_fichier->parent = NULL;

    return n_fichier;
}

// Recherche d'un dossier dans le dossier courant
struct Dossier *find_dossier(struct Dossier *current, char *name){
    for(int i=0; i<current->nb_dossier; i++){
        if (strncmp(current->dossiers[i]->name,name, strlen(name)-1) == 0){
            return current->dossiers[i];
        }
    }
    printf("Error: can't find %s\n",name);
    exit(-1);
}

// Ajout d'un dossier au filesystem
void ajout_dossier(struct Dossier *current, struct Dossier *nouveau){
    nouveau->depth = current->depth + 1;
    nouveau->parent = current;
    current->dossiers = realloc(current->dossiers, sizeof(struct Dossier *)*current->nb_dossier+1);
    current->dossiers[current->nb_dossier] = nouveau;
    current->nb_dossier++;
}

// Ajout d'un fichier au filesystem
void ajout_fichier(struct Dossier *current, struct Fichier *nouveau){
    nouveau->depth = current->depth + 1;
    current->taille += nouveau->taille;
    current->fichiers = realloc(current->fichiers, sizeof(struct Fichier *)*current->nb_fichiers+1);
    current->fichiers[current->nb_fichiers] = nouveau;
    current->nb_fichiers++;
}

// Affichage du filesystem en récursif
void print_fs(struct Dossier *dossier){
    char tab[20];
    for(int i=0;i<dossier->depth;i++){
        tab[i] = '\t';
    }
    printf("%s%s (dir size=%d)\n",tab, dossier->name, dossier->taille);
    for(int i=0; i<dossier->nb_dossier; i++){
        print_fs(dossier->dossiers[i]);
    }
    for(int i=0; i<dossier->nb_fichiers; i++){
        printf("\t%s%s (fichier size=%d)\n",tab, dossier->fichiers[i]->name, dossier->fichiers[i]->taille);
    }
}

// Mise à jour de la taille des dossiers en récursif
// Je me base sur le fait que la taille des dossier est déjà égale à la somme de ses fichiers
unsigned int update_size(struct Dossier *dossier){
    unsigned int to_add = 0;

    // si le dossier courant contient des dossier, je récupère la somme de ceux-ci
    if(dossier->nb_dossier != 0){
        for(int i=0; i<dossier->nb_dossier; i++){
            to_add += update_size(dossier->dossiers[i]);
        }
    }
    dossier->taille += to_add;

    return dossier->taille;
}

void step1(){
    FILE *file = fopen("../Day_7/input_test.txt", "r");
    char line[128];

    struct Dossier *root = nouveau_dossier("/\n");
    struct Dossier *current = root;

    // On crée l'arborescence avec root comme racine
    // et on met à jour la taille d'un dossier avec la somme des tailles de ses fichiers
    while(fgets(line, sizeof(line), file) != NULL) {
        if(line[0] == '$'){
            // La commande CD modifie le dossier courent (variable current)
            if(strncmp(line+2,"cd",2)==0){
                // On retourne à la racine
                if(strncmp(line+5, "/", 1) == 0){
                    current = root;
                
                // On remonte d'un étage dans le filesystem
                }else if(strncmp(line+5,"..",2)==0){
                    current = current->parent;
                
                // On cherche si ce dossier existe dans le dossier courant
                }else{
                    current = find_dossier(current, line+5);
                }
            
            // commande inutile dans mon implémentation
            }else if(strncmp(line+2,"ls",2)==0){
                //printf("LS !\n");
            }else{
                printf("Unknown cmd %s\n", line);
            }
        }else{
            // On récupère les deux champs avec la fonction des enfers !
            char * field1 = strtok ( line, " " );
            char * field2 = strtok ( NULL, " " );

            // Si c'est un dossier, field2 est le nom du dossier
            if(strncmp(field1,"dir",3)==0){
                ajout_dossier(current, nouveau_dossier(field2));
            
            // Si c'est un fichier, field1 est la taille et field2 est le nom du fichier
            }else{
                ajout_fichier(current, nouveau_fichier(field2, field1));
            }
        }
    }
    print_fs(root);

    // On a plus qu'à itérer sur tous les dossiers en partant des feuilles pour mettre à jour les tailles
    update_size(root);

    print_fs(root);

    fclose(file);
}

int main(){
    step1();
}