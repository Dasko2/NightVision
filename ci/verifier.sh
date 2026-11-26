#!/usr/bin/env bash
# Compile le mod contre les sources d'Horizon au commit épinglé dans horizon-mod.toml.
# Chaque fonction de mixin est ajoutée au fichier du jeu qui contient sa cible,
# puis le client est vérifié par le compilateur.
# Usage : ci/verifier.sh <dossier des sources Horizon>
set -euo pipefail
mod="$(cd "$(dirname "$0")/.." && pwd)"
cd "$1"

rm -rf nv_check
mkdir nv_check
cp -r "$mod/Cargo.toml" "$mod/src" nv_check/
sed -i 's#path = ".horizon/sdk/api/crates/horizon-mod-api"#path = "../crates/horizon-mod-api"#' nv_check/Cargo.toml
sed -i '0,/^members = \[/s##members = [\n    "nv_check",#' Cargo.toml
sed -i '0,/^\[dependencies\]/s##[dependencies]\nnightvision = { path = "../../nv_check" }#' crates/horizon-client/Cargo.toml

# inject <helper> <fichier du jeu> <nom de la fonction ciblée>
inject() {
  grep -qE "fn $3\b" "$2" || { echo "Cible introuvable : $3 dans $2" >&2; exit 1; }
  cat "$mod/src/mixins/$1" >> "$2"
}
game=crates/horizon-client/src/game
inject tick.rs "$game/native_hooks/client_tick_pre.rs" native_client_tick_pre
inject frame.rs "$game.rs" render_and_flip
inject night_vision.rs "$game/blocks/sculk.rs" night_vision_intensity

cargo test -p nightvision
cargo check -p horizon-client
