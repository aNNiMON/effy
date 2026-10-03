#!/usr/bin/env bash
TAPES_DIR="$PWD/dist/tapes"
ASSETS_DIR="$PWD/.assets"
RELEASE_DIR="$PWD/target/release"
DEBUG_DIR="$PWD/target/debug"

if [[ $# -lt 1 ]]
then
  echo "At least one argument is required: themes, main, or optimize"
  exit 1
fi

# Arguments
SCRIPT_THEMES=0
SCRIPT_MAIN=0
OPTIMIZE=0

for arg in "$@"
do
  case "$arg" in
    themes)
      SCRIPT_THEMES=1
      ;;
    main)
      SCRIPT_MAIN=1
      ;;
    optimize)
      OPTIMIZE=1
      ;;
  esac
done

# Prerequisites
declare -a REQUIRED_FILES=(
  "$TAPES_DIR/main.tpltape"
  "$TAPES_DIR/themes.tpltape"
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

has_debug=1
if [[ ! -d "$DEBUG_DIR" ]]
then
  has_debug=0
  echo "Debug build not found. Run 'cargo build' first."
fi

has_release=1
if [[ ! -d "$RELEASE_DIR" ]]
then
  has_release=0
  echo "Release build not found. Run 'cargo build --release' first."
fi


function run() {
  # $1: tape name
  # $2: binary type (debug or release)
  tape="$TAPES_DIR/$1.tape"
  envsubst < "$TAPES_DIR/$1.tpltape" > "$tape"
  # Run vhs with effy release build
  PATH="$PATH:$2" vhs "$tape"
  if [[ $? -eq 0 ]]
  then
    # Clean up
    rm -f "$ASSETS_DIR/in_out.mp3" "$ASSETS_DIR/in_out.mp4"
  fi
}

# relative path only!
export OUT=".assets/out"
mkdir -p "$OUT"

## Themes
if [[ $SCRIPT_THEMES -eq 1 ]]
then
  if [[ $has_debug -eq 0 ]]
  then
    echo "Skipping recording because debug build is not available."
  else
    echo "Themes"
    export TYPE=""
    export THEME="ChallengerDeep"
    run "themes" "$DEBUG_DIR"
  fi
fi

## Main
if [[ $SCRIPT_MAIN -eq 1 ]]
then
  if [[ $has_release -eq 0 ]]
  then
    echo "Skipping recording because release build is not available."
  else
    ## Dark
    echo "Dark mode"
    export TYPE=""
    export THEME="ChallengerDeep"
    run "main" "$RELEASE_DIR"

    ## Light
    echo "Light mode"
    export TYPE="w"
    export THEME="zenbones_light"
    run "main" "$RELEASE_DIR"
  fi
fi


echo "Processing done. Results:"
du -hs "$OUT"/*

if [[ $OPTIMIZE -eq 1 ]]
then
  if command -v magick >/dev/null 2>&1
  then
    echo "Optimizing..."
    OPT=".assets/opt"
    mkdir -p "$OPT"
    cp "$OUT"/* "$OPT"
    magick mogrify -dither none -colors 32 "$OPT"/*
    echo "Optimizing done. Results:"
    du -hs "$OPT"/*
  else
    echo "ImageMagick not found. Skipping optimization."
  fi
fi

