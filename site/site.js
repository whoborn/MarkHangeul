import './sdk/markhangeul.js';
const english = document.documentElement.lang === 'en';
const demo = document.querySelector('#demo');
const status = document.querySelector('#demo-status');
if (demo) {
  demo.addEventListener('mh-render', () => { status.textContent = english ? 'Rendered live with MarkHangeul' : '실제 MarkHangeul 렌더러로 표시한 결과'; });
  demo.addEventListener('mh-error', () => { status.textContent = english ? 'Unable to load the preview. Please refresh.' : '미리보기를 불러오지 못했습니다. 새로고침해 주세요.'; });
  document.querySelectorAll('[data-source]').forEach(button => button.addEventListener('click', () => {
    document.querySelectorAll('[data-source]').forEach(b => b.setAttribute('aria-pressed', String(b === button)));
    document.querySelector('#demo-source').textContent = button.dataset.source;
    demo.source = button.dataset.source;
  }));
}
