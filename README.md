# MarkHangeul · 마크한글

한글로 적은 발음의 **높낮이는 글자 자체의 세로 변화**, **장단은 좌우 폭**으로 표현하는 Markdown 확장 포맷과 웹 편집기입니다. 사람이 한글 독음과 발음 정보를 입력하는 표현 엔진이며, 모든 언어의 자동 전사기나 IPA 대체 표준은 아닙니다.

```md
마{T1} 마{T2} 마{T3} 마{T4}
마{lang=yue,tone=6}
마{toneContour=214,duration=long}
((안녕하세요)){pitch=rise}
**아{——!}**
```

- 중국어 4성·광둥어 6성·하노이 베트남어 6/6+2범주·태국어 5성 및 사용자 지정 contour 지원
- 언어·지역 선택기, 기식성·삐걱 발성·성문음화·입성 보조표시
- 8개 패턴 비교용 `toneSystem=generic-8` 제공: 실제 언어의 8성 번호 체계는 아님
- 한 음절 내부에서도 성조 변화 표시. 보조선 없이도 글자 변형 유지
- 장단 7단계와 실제 배치 폭 동기화
- Markdown 표·강조·목록·코드·수식 지원, HTML 입력 비실행
- 원문 Markdown / 일반 Markdown / CSS 포함 독립 HTML / JSON 파일 저장
- 서버 없이 브라우저에서 동작하는 Rust + Yew + WebAssembly 정적 앱

일반 Markdown 뷰어는 발음 변형을 표시하지 않습니다. 원문을 보존하려면 `.mh.md`, 일반 문서로 사용하려면 일반 MD, 글자 변형을 공유하려면 HTML 내보내기를 선택하세요. HTML의 수식은 TeX 원문으로 남습니다.

[문법과 지원 범위](docs/SYNTAX.md) · [언어·지역별 프로필](docs/LANGUAGE-PROFILES.md) · [전체 코드 점검 및 수정 보고서](docs/AUDIT-2026-10-01.md)

## 바로 실행

```sh
rustup target add wasm32-unknown-unknown
cargo install --locked trunk --version 0.21.14
env -u NO_COLOR trunk serve
```

`http://localhost:8080`에서 열고 **성조** 샘플의 번호·음높이·보조선 유무를 비교하세요. 처음 사용하기 안내는 화면 상단에 있습니다. 편집 내용은 자동 저장되지 않으므로 원문 파일을 저장하세요.

## 검증

```sh
cargo fmt --all -- --check
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo check -p markhangeul-web --target wasm32-unknown-unknown --locked
env -u NO_COLOR trunk build --release --locked --public-url /MarkHangeul/
```

## 웹 배포

정적 호스팅 루트에 올리려면:

```sh
env -u NO_COLOR trunk build --release --locked --public-url /
```

생성된 `dist/` 전체를 업로드합니다. 프로젝트 하위 경로에 배포하려면 `/MarkHangeul/`처럼 실제 경로를 지정하세요. `.wasm`은 `application/wasm` MIME으로 제공하는 HTTP(S) 서버가 필요합니다. WASM 앱의 `index.html`을 `file://`로 직접 열지 마세요. 내보낸 독립 HTML은 직접 열 수 있습니다.

GitHub Pages:

1. 저장소 Settings → Pages → Source를 GitHub Actions로 설정합니다.
2. `main` push 또는 Deploy workflow의 수동 실행으로 빌드·배포합니다.
3. workflow는 Pages 메타데이터의 `base_path`를 사용하므로 프로젝트 사이트·사용자 사이트·커스텀 도메인 경로를 하드코딩하지 않습니다.

PR에서는 별도 CI가 포맷·테스트·Clippy·WASM 체크·하위 경로 빌드를 실행합니다. 배포는 `deploy-pages.yml`이 수행합니다. 이 코드 수정 작업에서는 실제 원격 배포를 실행하지 않았습니다.

[GitHub Pages 공식 워크플로 안내](https://docs.github.com/en/pages/getting-started-with-github-pages/using-custom-workflows-with-github-pages)와 [Trunk 경로 설정](https://trunk-rs.github.io/trunk/guide/advanced/paths.html)을 참고하세요.

Playground 수식은 버전을 고정한 MathJax CDN을 사용합니다. CDN을 사용할 수 없어도 발음 표시와 편집은 동작하며 수식 조판만 생략됩니다.

## 구조와 기준 구현

```text
crates/markhangeul-core/   AST, 파서, 성조 해석, export
crates/markhangeul-web/    Yew UI, 안전한 Markdown/SVG 렌더러, HTML export
assets/styles/           렌더링과 반응형 UI
.github/workflows/       CI와 Pages 배포
apps/markhangeul-react/   보존용 초기 프로토타입 (운영 배포 대상 아님)
```

Rust/WASM 앱이 기준 구현입니다. 구 React 앱은 4성만 지원하고 최신 Markdown 보호·성조 문법이 없으므로 비교·역사 보존용으로만 유지합니다. 기획 v1.3/v1.4는 원래 설계 기록이며 현재 계약은 `docs/SYNTAX.md`를 따릅니다.

## 라이선스

MIT · 주식회사 후본 배영식 (Whoborn Inc. Bae Young Sik)

## 세계 언어 발음 예제

**세계 언어 발음**에서 유럽권·아랍권·남아시아·동아프리카 등 11개 언어(12개 지역/발음 프로필)를 선택하고, **원어 + 한글 / 한글 / 원어** 방식으로 Markdown 예제를 만들 수 있습니다. IPA·뜻·언어/지역·출처를 함께 표시합니다. 기존 4·6·8성 비교 기능도 유지합니다. 자세한 표현 범위와 한계는 [언어 프로필](docs/LANGUAGE-PROFILES.md#세계-언어-예제와-표기-언어-선택)을 참고하세요.
