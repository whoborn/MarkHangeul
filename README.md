# MarkHangeul

마크한글(MarkHangeul)은 Markdown 문서 안에서 발음, 억양, 장단, 강세, 성조, 음운 변화를 문자 자체의 시각 변화로 표현하는 표기 포맷입니다.

이 저장소는 Rust/WebAssembly 중심의 monorepo입니다. 기존 React/Vite Node.js 구현은 보존하되 `apps/markhangeul-react`로 분리했고, 현재 주 구현은 Rust core crate와 Yew WASM 웹앱입니다.

## Workspace

```text
.
├── Cargo.toml
├── Trunk.toml
├── index.html
├── assets/styles/
├── crates/
│   ├── markhangeul-core/
│   └── markhangeul-web/
├── apps/
│   └── markhangeul-react/
├── docs/
│   └── SYNTAX.md
└── .github/workflows/deploy-pages.yml
```

## Crates

`markhangeul-core`

- Parser
- AST
- symbol annotation 해석
- key-value annotation 해석
- range annotation 해석
- Plain Markdown export
- JSON AST export
- `cargo test` 기반 테스트

`markhangeul-web`

- Yew 기반 WASM Playground
- EditorPanel
- PreviewPanel
- TokenInspector
- ErrorPanel
- SampleSelector
- ExportPanel
- Trunk 기반 정적 빌드

## Syntax

```md
녕{↗—!}
Hello{!↗}
妈{T2}
((want to)){reduced=true}
녕{pitch=rise,duration=long,stress=strong}
```

범위 규칙:

- 한글, 한자, 가나는 바로 앞 grapheme에 적용합니다.
- 라틴 문자는 바로 앞 단어에 적용합니다.
- 여러 글자 또는 여러 단어는 `((...)){...}`로 적용합니다.

자세한 문법은 [docs/SYNTAX.md](docs/SYNTAX.md)를 보세요.

## Local Development

Workspace 테스트:

```sh
cargo test --workspace
```

WASM 앱 타입 체크:

```sh
rustup target add wasm32-unknown-unknown
cargo check -p markhangeul-web --target wasm32-unknown-unknown
```

Trunk 설치와 실행:

```sh
cargo install --locked trunk
trunk serve
```

정적 빌드:

```sh
trunk build --release
```

현재 Codex 실행 환경처럼 `NO_COLOR=1`이 설정되어 Trunk가 색상 옵션을 잘못 해석하면 다음처럼 실행합니다.

```sh
env -u NO_COLOR trunk build --release
```

## GitHub Pages

`.github/workflows/deploy-pages.yml`는 다음을 수행합니다.

- Rust stable 설치
- `wasm32-unknown-unknown` target 설치
- Trunk 설치
- `cargo test --workspace`
- `trunk build --release --public-url "/${{ github.event.repository.name }}/"`
- GitHub Pages artifact 업로드 및 배포

GitHub 저장소에서는 `Settings > Pages > Build and deployment > Source`를 `GitHub Actions`로 설정하면 됩니다.

## Legacy Node App

기존 Node.js/React/Vite 구현은 `apps/markhangeul-react`에 있습니다.

```sh
cd apps/markhangeul-react
npm install
npm run dev
```

## License

MIT

저작자: 주식회사 후본 배영식 (Whoborn Inc. Bae Young Sik)
