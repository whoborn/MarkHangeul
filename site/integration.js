import './sdk/markhangeul.js';
const english = document.documentElement.lang === 'en';
const input = document.querySelector('#source');
const output = document.querySelector('#output');
const status = document.querySelector('#status');
output.addEventListener('mh-render', event => { status.textContent = english ? `Rendered · ${event.detail.errors.length} annotation errors` : `표시 완료 · 주석 오류 ${event.detail.errors.length}개`; });
output.addEventListener('mh-error', () => { status.textContent = english ? 'Unable to load the SDK. Please check your connection.' : 'SDK를 불러오지 못했습니다. 연결 상태를 확인하세요.'; });
input.addEventListener('input', () => { output.source = input.value; });
output.source = input.value;
