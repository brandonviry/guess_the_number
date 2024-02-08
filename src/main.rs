
//! Ce programme est un jeu de devinette où l'utilisateur doit deviner un nombre aléatoire entre 1 et 100.
//!
//! Auteur: VIRY brandon alias Chikara974
//! Date: 07/02/2024


// importation des modules 
use rand::Rng; //  module nombre aléatoire 
use std::io; // mode de la gestion des entres/sorties

// fonction princpale 
fn main() {
   // initalisation de du générateur de nombre aléatoire 
   let mut rng = rand::thread_rng();

   // generation d'un nombre aléatoire entre 1 et 100
   let _random_number = rng.gen_range(1..=100);

   //Nombre de tentative 
   let mut tentative = 10;

   // boulce du jeu
   loop{

    // Initialisation de la varialbe pour stocke la sais de utilisateur 
   let mut input = String::new();

   //Affichage de nombre de tentive et demande de l'utilsateur de  saisir un nombre 
    println!("Saisir un nombre entier entre 1 et 100: (tentative {})",tentative);

    // lecture de l'netrée de l'utilsateur 
   match io::stdin().read_line(&mut input){
         // si réussite de la lecture 
         Ok(_) => {
            // conversion de l'entré par l'utilisateur en nombre de 8 bits 
            match input.trim().parse::<i8>(){
                // conversion reussi
                Ok(val) =>{
                     // comparaison de la saisie de l'utilisateur est égal au nombre aléatoire 
                     if val ==  _random_number {
                         println!("Victoire le nombre était bien : {}", _random_number); // affichage de la victoire et qu nombre aléatoire 
                         break;
                     }
                     else{
                         // sinon affichage de la défaite 
                         println!("Non c'est pas sa !");
                         // retirer de une tentative 
                         tentative-=1;

                         // arret de la boucle de jeux si le nombre de tentative arriver à 0
                         if tentative == 0{
                             break;
                         }
                     }
                }
                  
                Err(_)=>{
                    // echec de la conversion 
                    println!("ERREUR"); // affiche de Erreur 
                }
            }
        }
        Err(error) => {
            // si échec de la lecture
            eprintln!("Echec de la lecture: {}", error);
        }
   }
   }
}




