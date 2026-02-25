# Calculatrice CLI en Rust

Une petite calculatrice en ligne de commande écrite en Rust, créée pour apprendre le langage et comprendre la programmation « plus près de la machine ». Ce projet est volontairement simple et pédagogique : l'objectif principal est l'apprentissage, pas la production.

## Motivation et apprentissage

J'ai décidé de repartir sur un nouveau langage plus proche de la machine pour "rentrer dans le dur" de la programmation. Après plusieurs années à travailler principalement avec TypeScript et des applications web, je voulais :

- Approfondir des concepts bas-niveau (ownership, emprunts, gestion explicite des erreurs).
- Comprendre comment gérer les erreurs proprement avec `Result`, `Ok` et `Err`.
- Travailler le parsing d'input utilisateur et les conversions de types en Rust.
- Écrire une boucle CLI interactive et apprendre l'organisation d'un projet Cargo.

Important : pour cet apprentissage je me suis volontairement interdit d'utiliser l'IA pour générer la logique principale. Le but était d'implémenter et déboguer moi-même chaque partie pour bien intégrer les concepts.

## Fonctionnalités

- Opérations de base : addition, soustraction, multiplication, division.
- Gestion des erreurs : input invalide, division par zéro, erreurs de conversion.
- Interface CLI interactive avec commande `exit` pour quitter.
- Entrées tolérantes aux espaces et formats simples (ex : `5 + 3`).

## Comment l'utiliser

1. Cloner le dépôt :

```bash
git clone <URL_DU_REPO>
cd <nom_du_dossier>
```

2. Compiler et lancer :

```bash
cargo run
```

3. Exemple d'utilisation dans l'interface :

```
> 5 + 3
Résultat: 8

> 10 / 2
Résultat: 5

> 10 / 0
Erreur: division par zéro

> exit
```

Notes : selon la version locale de Rust installée, vous pouvez aussi compiler en release : `cargo run --release`.

## Structure du projet

- `Cargo.toml` – configuration du package.
- `src/main.rs` – point d'entrée et logique principale de la CLI.
- `target/` – fichiers générés par Cargo lors des builds.

## Ce que j'ai appris (en bref)

- Manipulation de `Result` et gestion des erreurs idiomatiques.
- Parsing d'entrées texte et conversion en types numériques.
- Contrôle de flux en Rust (`loop`, `match`, `if let`).
- Organisation d'un projet Rust avec Cargo.

## Technologies

- Rust (stable)
- Cargo

## Auteur

Renan Yhuel

Email : renan.yhuel@gmail.com