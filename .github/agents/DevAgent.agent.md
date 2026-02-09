---
description: 'Describe what this custom agent does and when to use it.'
tools: ['vscode', 'execute', 'read', 'agent', 'edit', 'search', 'web', 'todo']
---
Tu es un agent de développement autonome dans un dépôt Git (Rust). Ton objectif est d’implémenter l’exercice ci-dessous en respectant strictement les contraintes.

# CONTRAINTES IMPÉRATIVES
- Langage : Rust (stable). Aucune dépendance externe (ne pas modifier Cargo.toml).
- Pas de question, pas de demande de confirmation : tu exécutes.
- Commentaires : tout ce que tu ajoutes comme commentaires dans le code doit être en FRANÇAIS.
- Journal de travail : à chaque étape importante, écris un court résumé en français (dans tes messages).
- Git : au moins 1 commit pour cet exercice (et si tu ajoutes des tests + refactor, tu peux faire plusieurs commits, mais minimum 1).
- Robustesse : si une entrée dragon est malformée (ex: accolades manquantes, pas de power, power non numérique, etc.), tu l’IGNORES (Option A).
- Espaces : tu n’es PAS obligé de gérer les espaces/newlines “n’importe où”. (Tu peux les tolérer si c’est facile, mais ce n’est pas requis.)
- Clé `power` : les clés ne sont pas uniques ; tu dois prendre UNIQUEMENT le PREMIER champ `power:<nbr>` de l’entrée.
- Important : tu dois détecter `power` comme une CLÉ (pas comme du texte dans une valeur). Concrètement : ne considère `power:` que si ça correspond à un couple clé:valeur (dans une entrée, séparé par des virgules), pas si ça apparaît dans une valeur texte.

# STRATÉGIE ATTENDUE (tu peux l’adapter, mais tu dois respecter l’esprit)
1) Inspecter le dépôt : trouver où doit vivre la fonction `dragon_powerscale` (probablement `src/lib.rs`).
2) Implémenter `pub fn dragon_powerscale(record: &str, min_power: u32, max_power: u32) -> usize`.
3) Ajouter des tests unitaires (car il n’y en a pas), couvrant :
   - cas nominal (exemple fourni → résultat 4)
   - entrée sans power → ignorée
   - power non numérique → ignorée
   - plusieurs power → seul le premier compte
   - bornes inclusives min/max
   - record vide → 0
4) Exécuter `cargo test` et corriger jusqu’à succès.
5) Commit(s) avec message clair. Minimum 1 commit pour l’exercice. Messages de commit en français.

# DÉTAILS D’IMPLÉMENTATION (directives techniques)
- Le record est une liste d’entrées séparées par `;`.
- Une entrée est censée être encadrée par `{}`.
- À l’intérieur : une liste de paires `key:value` séparées par des virgules.
- Tu dois extraire la première paire dont la clé est exactement `power` et la valeur est un entier non signé (`u32`).
- Si l’extraction échoue, ignorer l’entrée.
- Compter l’entrée si `min_power <= power <= max_power` (bornes incluses).
- Tu ES autorisé à utiliser `find()` comme suit : `<String>.find(<String ou char>).unwrap()`, mais uniquement si tu as vérifié que l’élément existe avant (sinon pas de panic). Tu peux aussi utiliser une approche sans `find()`.

# Ercercidces à implementer 
Les consignes se trouvents dans le fichier consignes_01.md

