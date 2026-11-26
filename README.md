# NightVision

Mod natif pour [Horizon](https://playhorizon.dev) : gamma amplifié et vision nocturne, en solo comme sur un serveur.

| Touche | Effet |
| --- | --- |
| **H** | Gamma amplifié (200 à 3000 %, 1500 % par défaut) |
| **N** | Vision nocturne (rendu de l'effet, sans potion ni icône) |

Touches modifiables dans les contrôles ; réglages dans Options → Mods → NightVision.
Tout est côté client : rien n'est envoyé au serveur et le curseur de luminosité n'est pas modifié.

> Certains serveurs interdisent ce genre de mod. Vérifie leurs règles avant de t'en servir.
> Horizon bloque d'ailleurs son propre « fullbright » sur les serveurs des autres.

## Installation

1. Il faut une instance Horizon sur le build `f037744a79f9982c47876f49c5a3067279f88706`
   (le mod utilise des mixins : il ne se compile que sur ce build).
2. Dans le launcher : page Mods → **Add from GitHub** → l'URL de ce dépôt
   (accepter l'avertissement sur les mods natifs), puis **Build**.

En ligne de commande :

```sh
horizon-launcher mods add-github https://github.com/Dasko2/NightVision --trust-native-code
horizon-launcher native-build --instance "NOM" --trust-native-code
```

Dépôt privé : définir `HORIZON_GITHUB_TOKEN`, ou cloner puis
`horizon-launcher native-build dossier --instance "NOM" --trust-native-code`.
