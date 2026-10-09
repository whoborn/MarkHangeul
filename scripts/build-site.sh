#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
base_path="${1:-/MarkHangeul/}"
env -u NO_COLOR trunk build --release --locked --public-url "$base_path"
mv dist/index.html dist/preview.html
cargo build -p markhangeul-sdk --release --locked --target wasm32-unknown-unknown
wasm-bindgen --target web --out-dir dist/sdk --out-name markhangeul_sdk target/wasm32-unknown-unknown/release/markhangeul_sdk.wasm
cp sdk/markhangeul.js assets/styles/markhangeul.css dist/sdk/
cp -R site/. dist/
cp LICENSE dist/sdk/LICENSE
cp LICENSE dist/LICENSE
mkdir -p dist/docs
cp docs/SYNTAX.md docs/LANGUAGE-PROFILES.md docs/INTEGRATION.md docs/INTEGRATION.en.md dist/docs/
touch dist/.nojekyll
python3 - <<'PY'
from pathlib import Path
from zipfile import ZipFile, ZIP_DEFLATED
root = Path('dist')
with ZipFile(root / 'markhangeul-site.zip', 'w', ZIP_DEFLATED) as archive:
    for path in sorted(root.rglob('*')):
        if path.is_file() and path.name != 'markhangeul-site.zip':
            archive.write(path, path.relative_to(root))
PY
python3 scripts/check-site.py "$base_path"
