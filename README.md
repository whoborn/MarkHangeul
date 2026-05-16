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
- pulldown-cmark 기반 Markdown/GFM preview
- MathJax 기반 LaTeX 수식 preview
- Trunk 기반 정적 빌드

## Syntax

```md
# 제목과 Markdown 문법

**안녕{↗—!}하세요**
수식: $E = mc^2$

녕{↗—!}
헬로{!↗}
妈=마{T1}
麻=마{T2}
马=마{T3}
骂=마{T4}
((원 투)){reduced=true}
녕{pitch=rise,duration=slight-long,stress=strong}
아{duration=extra-long}
詩=시{lang=yue,tone=1}
마{tone=custom,toneContour=214}
아{duration=extra-long,guideColor=true}
마{T2,soundShape=false}
```

범위 규칙:

- 한글, 한자, 가나는 바로 앞 grapheme에 적용합니다.
- 라틴 문자는 바로 앞 단어에 적용합니다.
- 여러 글자 또는 여러 단어는 `((...)){...}`로 적용합니다.

기본 렌더링은 원문 Markdown의 글자 색상과 굵기를 바꾸지 않습니다. 성조 표기는 글자 아래 음형태 보조선을 기본으로 표시하고, `soundShape=false` 또는 `hideGuide=true`로 숨길 수 있습니다. 반대로 `↗`, `↘`, `—`, `!`, `°`, `●` 같은 순수 기호형 표기는 글자 자체 변화만 기본으로 하며, 선이 필요할 때 `soundShape=true`를 지정합니다.

장단은 글자 자체를 좌우로 늘리거나 압축해 표현하며, 7단계를 지원합니다. key-value 장단은 폭 변화와 단색 아래 선을 함께 사용할 수 있고, 기호형 장단은 기본적으로 선 없이 폭 변화만 보입니다. 길이별 유색 선이 필요하면 `guideColor=true`를 추가합니다.

- `extra-long`: 아주 길게
- `long`: 보통 길게
- `slight-long`: 조금 길게
- `normal`: 보통
- `slight-short`: 조금 짧게
- `short`: 보통 짧게
- `extra-short`: 아주 짧게

아래 보조선 색상 구분 예: `아{duration=extra-long,guideColor=true}`, `아{duration=extra-short,guideColor=true}`.

성조는 특정 언어의 4성에 고정하지 않고 `tone=1`~`tone=9`, `toneSystem`, `toneContour`로 범용 표현할 수 있습니다. 한자 자체를 그대로 성조 처리하기보다 실제 소리를 한글로 풀어 `妈=마{T1}`, `麻=마{T2}`, `马=마{T3}`, `骂=마{T4}`처럼 표기하는 것을 기본 예제로 둡니다. 성조 보조선은 평탄선, 상승 곡선, 하강 곡선, 굴곡 곡선을 포함한 원형 contour로 렌더링합니다.

기호형 강세 `!`, `!!`는 글자 좌우 폭을 유지한 상태에서 높이만 키웁니다. 기호형 성량 `°`, `●`는 높이와 좌우 폭이 함께 작아지거나 커지는 형태로 표현합니다. 둘 다 font-weight는 바꾸지 않습니다.

글자 아래 음형태 보조선은 성조에서 기본적으로 표시합니다. 필요할 때만 `soundShape=false` 또는 `hideGuide=true`로 숨깁니다. 예: `마{tone=custom,toneContour=214,soundShape=false}`.

강세 `stress`는 Markdown의 `**굵게**`와 충돌하지 않도록 font-weight를 바꾸지 않습니다. 현재 렌더러는 글자 좌우 폭을 유지하고 높이 변화만 사용합니다.

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
