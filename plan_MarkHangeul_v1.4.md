---

# 마크한글(MarkHangeul) 최종 Plan v1.4

## Rust + WASM + GitHub Actions 배포형 구현 계획

---

# 1. 최종 기술 방향

## 권장 스택

| 영역       | 기술                       |
| -------- | ------------------------ |
| 언어       | Rust                     |
| 브라우저 실행  | WebAssembly              |
| UI 프레임워크 | Yew                      |
| 빌드/번들러   | Trunk                    |
| 파서/엔진    | Rust crate               |
| 테스트      | cargo test + wasm 테스트 확장 |
| 자동화      | GitHub Actions           |
| 배포       | GitHub Pages             |

Yew는 Rust 기반의 프런트엔드 WebAssembly 프레임워크이며, 공식 문서에서도 Trunk를 권장 빌드 도구로 안내합니다. ([Yew][2])

---

# 2. 프로젝트 전체 구조

마크한글은 다음 3층으로 분리합니다.

```text
1. markhangeul-core
   - 문법 파서
   - AST
   - 속성 해석
   - symbol annotation 해석
   - key-value annotation 해석
   - export 변환

2. markhangeul-web
   - Yew 기반 UI
   - Playground
   - Preview Renderer
   - Inspector
   - Error Panel

3. WASM 빌드/배포 계층
   - Trunk
   - GitHub Actions
   - GitHub Pages
```

이 구조의 장점은 다음과 같습니다.

* 핵심 파서를 UI와 분리할 수 있음
* 향후 CLI, 서버, 라이브러리 배포 가능
* WASM UI는 동일 코어를 그대로 사용
* 테스트가 쉬움

---

# 3. 권장 저장소 구조

```text
markhangeul/
  Cargo.toml
  Trunk.toml
  index.html
  README.md
  LICENSE

  crates/
    markhangeul-core/
      Cargo.toml
      src/
        lib.rs
        ast.rs
        parser/
          mod.rs
          tokenize.rs
          symbol.rs
          key_value.rs
          range.rs
        render_model/
          mod.rs
        serializer/
          mod.rs
          plain_markdown.rs
          json.rs
        errors.rs
      tests/
        parser_cases.rs
        range_cases.rs

    markhangeul-web/
      Cargo.toml
      src/
        main.rs
        app.rs
        components/
          mod.rs
          editor.rs
          preview.rs
          inspector.rs
          sample_selector.rs
          syntax_guide.rs
          error_panel.rs
        styles/
          mod.rs

  assets/
    styles/
      markhangeul.css
      playground.css

  .github/
    workflows/
      deploy-pages.yml
```

---

# 4. Cargo Workspace 구조

최상위 `Cargo.toml`은 workspace로 둡니다.

```toml
[workspace]
members = [
  "crates/markhangeul-core",
  "crates/markhangeul-web"
]

resolver = "2"
```

---

# 5. core crate의 책임

## `markhangeul-core`

이 crate는 UI와 완전히 분리된 **순수 Rust 라이브러리**입니다.

### 담당 기능

1. 마크한글 문법 파싱
2. 일반 Markdown 텍스트 보존
3. `대상{표기}` 감지
4. 기호형 annotation 해석
5. key-value형 annotation 해석
6. `((범위)){...}` 적용 범위 해석
7. AST 생성
8. Plain Markdown export
9. JSON AST export
10. 오류 정보 반환

---

# 6. 마크한글 핵심 문법

```text
대상{발음표기}
```

예:

```md
녕{↗—!}
Hello{!↗}
妈{T2}
((want to)){reduced=true}
```

---

# 7. 파싱 규칙

## 7.1 기본 대상 해석

### 한글·한자·가나

바로 앞 **1개 grapheme**에 적용

```md
녕{↗}
妈{T2}
こ{—}
```

### 라틴 문자

바로 앞 **단어 전체**에 적용

```md
Hello{!↗}
world{—}
```

### 명시적 범위

`((...))` 전체에 적용

```md
((안녕하세요)){↗}
((want to)){reduced=true}
```

---

# 8. annotation 해석 방식

## 8.1 기호형 annotation

| 표기      | 의미     |
| ------- | ------ |
| `↗`     | 상승 억양  |
| `↘`     | 하강 억양  |
| `↑`     | 높음     |
| `↓`     | 낮음     |
| `—`     | 장음     |
| `——`    | 매우 긴 음 |
| `!`     | 강세     |
| `!!`    | 강한 강세  |
| `°`     | 약한 성량  |
| `●`     | 큰 성량   |
| `T1~T4` | 성조     |

---

## 8.2 key-value annotation

```md
녕{pitch=rise,duration=long,stress=strong}
```

---

# 9. AST 설계

```rust
pub struct MarkHangeulNode {
    pub text: String,
    pub raw_annotation: String,
    pub scope: Scope,
    pub attributes: MarkHangeulAttributes,
}

pub enum Scope {
    Grapheme,
    Word,
    Range,
}
```

```rust
pub struct MarkHangeulAttributes {
    pub pitch: Option<Pitch>,
    pub duration: Option<Duration>,
    pub stress: Option<Stress>,
    pub volume: Option<Volume>,
    pub tone: Option<Tone>,

    pub ipa: Option<String>,
    pub phoneme: Option<String>,
    pub lang: Option<String>,
    pub note: Option<String>,

    pub nasal: Option<bool>,
    pub aspiration: Option<bool>,
    pub fortis: Option<bool>,
    pub lenis: Option<bool>,
    pub palatalization: Option<bool>,
    pub retroflexion: Option<bool>,
    pub liaison: Option<bool>,
    pub reduced: Option<bool>,
    pub assimilation: Option<bool>,
    pub deletion: Option<bool>,
    pub mora: Option<String>,
    pub syllable_role: Option<String>,
}
```

---

# 10. Yew 기반 UI 구성

## 웹 Playground

```text
좌측:
- 마크한글 입력 에디터

우측:
- 실시간 시각 렌더링

하단/사이드:
- 파싱 결과 Inspector
- AST JSON
- 오류 메시지
- Plain Markdown export
```

---

# 11. UI 컴포넌트

```text
App
 ├─ Header
 ├─ EditorPanel
 ├─ PreviewPanel
 ├─ TokenInspector
 ├─ ErrorPanel
 ├─ SampleSelector
 └─ ExportPanel
```

---

# 12. 렌더링 요구사항

마크한글은 **badge를 붙이는 시스템이 아니라, 글자 자체를 변화시키는 시스템**이어야 합니다.

## 렌더링 매핑

| 속성       | 시각 표현          |
| -------- | -------------- |
| pitch    | 글자의 세로 위치      |
| duration | 글자의 가로 폭       |
| stress   | 크기와 굵기         |
| volume   | 밀도와 대비         |
| tone     | 성조 곡선 또는 위치 패턴 |

예:

```md
녕{↗—!}
```

렌더링 결과:

* `녕`이 살짝 위로 상승
* 글자 폭이 길게 확장
* 글자가 조금 더 크고 굵게 표현

---

# 13. CSS 렌더링 전략

초기 MVP에서는 CSS class 기반 렌더링을 사용합니다.

```text
mh-pitch-rise
mh-duration-long
mh-stress-strong
mh-volume-soft
mh-tone-2
```

향후 확장:

* SVG tone curve
* Variable font
* 자모 단위 조작
* 음절 내부 시각 변형

---

# 14. Trunk 설정

Trunk는 Rust/WASM 앱의 빌드·번들링을 담당하며, HTML에 Rust 진입점을 연결해 정적 사이트를 생성할 수 있습니다. `--public-url` 값은 배포 경로에 맞춘 정적 자산 경로 제어에 사용할 수 있고, `<base data-trunk-public-url/>`를 통해 HTML에 반영할 수 있습니다. ([Trunk.rs][1])

## `index.html`

```html
<!doctype html>
<html lang="ko">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>MarkHangeul</title>

    <base data-trunk-public-url />

    <link data-trunk rel="css" href="assets/styles/markhangeul.css" />
    <link data-trunk rel="css" href="assets/styles/playground.css" />
  </head>
  <body>
    <main id="app"></main>
  </body>
</html>
```

---

## `Trunk.toml`

```toml
[build]
dist = "dist"
public_url = "/"

[watch]
watch = ["crates", "assets", "index.html"]
```

GitHub Pages 프로젝트 경로에 맞춰 배포 시에는 Actions 단계에서 `--public-url`을 덮어쓸 수 있게 둡니다. Trunk는 `--public-url` 값을 HTML 출력에 반영할 수 있습니다. ([Trunk.rs][3])

---

# 15. GitHub Actions 배포 구조

GitHub Pages 커스텀 워크플로는 일반적으로 다음 흐름을 가집니다.

1. 저장소 checkout
2. Rust toolchain 설치
3. WASM target 설치
4. Trunk 설치
5. 테스트 실행
6. WASM 빌드
7. Pages artifact 업로드
8. GitHub Pages 배포

GitHub 공식 문서는 Pages 커스텀 워크플로에서 artifact 업로드와 Pages 배포 단계를 설명합니다. ([GitHub Docs][4])

---

# 16. GitHub Actions 예시

`.github/workflows/deploy-pages.yml`

```yaml
name: Deploy MarkHangeul WASM Site

on:
  push:
    branches: [main]
  workflow_dispatch:

permissions:
  contents: read
  pages: write
  id-token: write

concurrency:
  group: pages
  cancel-in-progress: true

jobs:
  build:
    runs-on: ubuntu-latest

    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown

      - name: Cache Cargo
        uses: Swatinem/rust-cache@v2

      - name: Install Trunk
        run: cargo install --locked trunk

      - name: Run Core Tests
        run: cargo test -p markhangeul-core

      - name: Build WASM App
        run: trunk build --release --public-url "/${{ github.event.repository.name }}/"

      - name: Configure Pages
        uses: actions/configure-pages@v5

      - name: Upload Pages Artifact
        uses: actions/upload-pages-artifact@v5
        with:
          path: dist

  deploy:
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}

    runs-on: ubuntu-latest
    needs: build

    steps:
      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v5
```

GitHub Pages는 Actions 기반 배포를 지원하며, `configure-pages`, `upload-pages-artifact`, `deploy-pages` 흐름이 공식 배포 경로입니다. 현재 공식 릴리스 기준으로 `deploy-pages`는 v5.0.0이 확인되며, `upload-pages-artifact`도 v5 릴리스가 확인됩니다. ([GitHub Docs][4])

---

# 17. GitHub 저장소 설정

GitHub에서 다음을 설정합니다.

```text
Settings
→ Pages
→ Build and deployment
→ Source: GitHub Actions
```

커스텀 Pages 워크플로는 Pages 설정에서 Actions 배포를 사용하도록 두는 방식이 공식 문서에 안내되어 있습니다. ([GitHub Docs][4])

---

# 18. 로컬 개발 명령어

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
trunk serve
```

Yew 문서와 Trunk 문서는 Rust/WASM 앱 개발에서 Trunk 사용을 기본적인 로컬 실행 흐름으로 설명합니다. ([Yew][2])

---

# 19. 최종 Vibe Coding용 Master Prompt

## Rust + WASM + GitHub Actions Edition

```text
프로젝트명:
마크한글(MarkHangeul)

프로젝트 본질:
마크한글은 한글의 제자 원리와 조형적 특성을 현대적으로 확장하여,
세계 여러 언어의 발음·억양·장단·강세·성조 및 음운 변화를
Markdown 문서 안에서 직관적으로 표현하는 시각 발음 표기 포맷이다.

기술 스택:
- Rust
- WebAssembly
- Yew
- Trunk
- GitHub Actions
- GitHub Pages

아키텍처:
1. markhangeul-core
   - Rust 라이브러리
   - 문법 파서
   - AST 생성
   - 기호형 annotation 해석
   - key-value annotation 해석
   - 범위형 annotation 해석
   - Plain Markdown export
   - JSON AST export

2. markhangeul-web
   - Yew 기반 WASM UI
   - EditorPanel
   - PreviewPanel
   - TokenInspector
   - ErrorPanel
   - SampleSelector
   - ExportPanel

3. 빌드/배포
   - Trunk로 WASM 웹앱 빌드
   - GitHub Actions에서 cargo test, trunk build 수행
   - GitHub Pages에 자동 배포

마크한글 문법:
- 대상{표기}
- 예:
  녕{↗—!}
  Hello{!↗}
  妈{T2}
  ((want to)){reduced=true}

지원해야 할 문법:
1. 기호형 annotation
2. key-value형 annotation
3. 범위형 annotation

기호 의미:
- ↗ : 상승 억양
- ↘ : 하강 억양
- ↑ : 높은 음
- ↓ : 낮은 음
- — : 장음
- —— : 매우 긴 음
- ! : 강세
- !! : 강한 강세
- ° : 약한 성량
- ● : 큰 성량
- T1~T4 : 성조

렌더링 원칙:
- badge 형태가 아니라 글자 자체가 변화해야 한다.
- pitch는 세로 위치 변화로 표현
- duration은 글자 폭 변화로 표현
- stress는 글자 크기와 굵기 변화로 표현
- volume은 밀도와 대비 변화로 표현
- tone은 성조별 시각 패턴으로 표현

Scope 규칙:
- 한글/한자/가나는 바로 앞 grapheme 적용
- 라틴 문자는 바로 앞 단어 적용
- 여러 글자나 여러 단어는 ((...)){...}로 범위 적용

반드시 구현할 것:
1. Rust workspace 구성
2. markhangeul-core crate
3. markhangeul-web crate
4. Parser
5. AST
6. Renderer
7. Playground UI
8. Sample 문장
9. Inspector
10. Error Panel
11. Plain Markdown export
12. JSON AST export
13. cargo test 기반 테스트
14. Trunk 빌드 설정
15. GitHub Actions 배포 workflow
16. GitHub Pages 배포 가능 구조
17. README 및 문법 명세서

개발 우선순위:
- 프로젝트 철학 보존
- Markdown-safe 직관성
- 파서와 UI의 분리
- 확장 가능한 AST
- 브라우저 WASM 실행
- GitHub Actions 자동 배포
```

---

# 20. 최종 추천

이 프로젝트에는 다음 구성이 가장 잘 맞습니다.

```text
Rust core + Yew UI + Trunk WASM build + GitHub Actions + GitHub Pages
```

이 방식은:

* 마크한글의 문법·파서·렌더러를 하나의 Rust 생태계 안에서 관리할 수 있고
* 브라우저에서 직접 실행되며
* 정적 호스팅으로 배포 비용이 낮고
* GitHub 저장소에 push만 해도 자동으로 테스트·배포가 가능하게 만듭니다. ([Yew][2])

[1]: https://trunk-rs.github.io/trunk/?utm_source=chatgpt.com "Trunk | Build, bundle & ship your Rust WASM application to ..."
[2]: https://yew.rs/docs/getting-started/introduction?utm_source=chatgpt.com "Getting Started"
[3]: https://trunk-rs.github.io/trunk/guide/assets/index.html?utm_source=chatgpt.com "Assets - The Trunk Guide"
[4]: https://docs.github.com/en/pages/getting-started-with-github-pages/using-custom-workflows-with-github-pages?utm_source=chatgpt.com "Using custom workflows with GitHub Pages"
