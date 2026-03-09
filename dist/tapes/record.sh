#!/usr/bin/env bash
TAPES_DIR="$PWD/dist/tapes"
ASSETS_DIR="$PWD/.assets"
RELEASE_DIR="$PWD/target/release"

# Prerequisites
MAIN_TEMPLATE_TAPE="$TAPES_DIR/main.tpltape"
declare -a REQUIRED_FILES=(
  "$MAIN_TEMPLATE_TAPE"
  "$ASSETS_DIR/in.mp4"
)
for file in "${REQUIRED_FILES[@]}"
do
  if [[ ! -f "$file" ]]
  then
    echo "Missing required file: $file"
    exit 1
  fi
done

function run() {
  tape="$TAPES_DIR/main.tape"
  envsubst < "$TAPES_DIR/main.tpltape" > "$tape"
  # Run vhs with effy release build
  PATH="$PATH:$RELEASE_DIR" vhs "$tape"
  # Clean up
  rm "$ASSETS_DIR/in_out.mp3" "$ASSETS_DIR/in_out.mp4"
}

# relative path only!
export OUT=".assets/out"
mkdir -p "$OUT"

## Dark
echo "Dark mode"
export TYPE=""
export THEME="ChallengerDeep"
run

## Light
echo "Light mode"
export TYPE="w"
export THEME="zenbones_light"
run

echo "Processing done. Results:"
du -hs "$OUT"/*

if command -v magick >/dev/null 2>&1
then
  echo "Optimizing..."
  OPT=".assets/opt"
  cp -r "$OUT" "$OPT"
  magick mogrify -dither none -colors 32 "$OPT"/*
  echo "Optimizing done. Results:"
  du -hs "$OPT"/*
fi

