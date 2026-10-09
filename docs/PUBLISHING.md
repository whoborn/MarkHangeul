# Whoborn 공개 및 배포

## 계정과 소유권

2026-10-09 Whoborn 계정에 `whoborn/MarkHangeul` 공개 저장소를 생성했습니다. 원본 `baesic/MarkHangeul` 비공개 저장소는 유지하며 로컬 작업 브랜치는 `customiing`입니다. 공개 배포 브랜치는 `main`입니다. `whoborn`은 조직이 아닌 별도 개인 계정입니다. baesic의 커밋 이력은 Whoborn 소유로 공개해도 그대로 유지할 수 있습니다. Git 작성자와 저장소 소유자는 서로 다릅니다.

공개 방법은 두 가지입니다.

1. Whoborn 계정으로 새 공개 저장소 `whoborn/MarkHangeul`을 만들고 검증한 브랜치를 `main`으로 push. 기존 비공개 저장소를 유지할 수 있습니다.
2. 원본 저장소를 Whoborn 계정으로 이전한 뒤 공개 전환. 개인 계정 간 이전은 Whoborn 측 이메일 수락이 필요하고 GitHub 안내상 1일 내 수락해야 합니다.

이번 공개는 새 저장소 방식을 사용합니다. 아래 생성 절차는 새로운 저장소를 준비할 때의 참고이며, 이미 생성된 `whoborn/MarkHangeul`에 반복 실행하지 마세요. 토큰을 소스코드에 저장하지 마세요.

## 새 저장소 방식 (기존 저장소 유지)

```sh
gh auth login --hostname github.com --web
# 로그인 계정이 whoborn인지 반드시 확인
gh api user --jq .login
gh repo create whoborn/MarkHangeul --public --description '한글로 그리는 세계의 소리 · Whoborn Hangeul Day 2026'
git remote add whoborn https://github.com/whoborn/MarkHangeul.git
git push whoborn HEAD:main
```

이미 저장소나 remote가 있으면 덮어쓰지 않고 기존 설정을 확인합니다. 전체 Git 이력을 push하면 현재 삭제된 과거 파일도 이력에서 볼 수 있습니다. 이력을 삭제하거나 작성자를 바꿀 필요는 없습니다.

## Pages

1. Settings → Pages → Source를 **GitHub Actions**로 선택합니다.
2. Actions → Deploy MarkHangeul Site and SDK → Run workflow (`main`).
3. workflow의 Pages 환경 URL에서 소개·편집기·연동 예제를 확인합니다.
4. `https://whoborn.github.io/MarkHangeul/`, `preview.html`, `integration.html`, `sdk/markhangeul.js`를 확인합니다.
5. 외부 출처에서도 모듈과 WASM을 호출해 확인합니다.

CI는 Rust 테스트·fmt·Clippy·WASM 빌드를 검사합니다. 배포는 같은 파서/렌더러로 편집기와 SDK를 생성하고 사이트 ZIP을 만듭니다. WASM 바인딩 버전은 Cargo.lock의 wasm-bindgen 버전과 CLI 버전을 함께 갱신합니다.

근거: [GitHub 저장소 이전](https://docs.github.com/en/repositories/creating-and-managing-repositories/transferring-a-repository), [Pages 사용자 워크플로](https://docs.github.com/en/pages/getting-started-with-github-pages/using-custom-workflows-with-github-pages).
