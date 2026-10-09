/* MIT · Whoborn Inc. */
import initWasm, * as wasm from './markhangeul_sdk.js';
let ready;
export function init() {
  return ready ??= initWasm().catch(error => { ready = undefined; throw error; });
}
// Call await init() once before these synchronous APIs.
export function renderMarkdown(source) { return wasm.renderMarkdown(String(source)); }
export function parse(source) { return JSON.parse(wasm.parse(String(source))); }
export function plainMarkdown(source) { return wasm.plainMarkdown(String(source)); }
export function exportHtml(source) { return wasm.exportHtml(String(source)); }

// Optional isolated embedding. Does not modify an editor's DOM or existing Markdown parser.
export class MarkHangeulPreview extends HTMLElement {
  static observedAttributes = ['source'];
  #source = ''; #revision = 0;
  constructor() { super(); this.attachShadow({mode: 'open'}); }
  get source() { return this.#source; }
  set source(value) { this.#source = String(value); if (this.isConnected) this.update(); }
  connectedCallback() {
    this.#source = this.getAttribute('source') ?? (this.#source || this.textContent);
    this.update();
  }
  attributeChangedCallback(_name, _old, value) { this.source = value ?? ''; }
  async update() {
    const revision = ++this.#revision;
    try {
      await init();
      if (revision !== this.#revision || !this.isConnected) return;
      const link = document.createElement('link');
      link.rel = 'stylesheet'; link.href = new URL('./markhangeul.css', import.meta.url).href;
      const style = document.createElement('style');
      style.textContent = ':host{display:block;line-height:1.85}article{overflow-wrap:anywhere}table{border-collapse:collapse}td,th{padding:.5em;border:1px solid #ddd}.mh-mark{cursor:inherit}';
      const article = document.createElement('article'); article.className = 'markdown-body';
      article.innerHTML = renderMarkdown(this.#source);
      this.shadowRoot.replaceChildren(link, style, article);
      this.dispatchEvent(new CustomEvent('mh-render', {detail:parse(this.#source)}));
    } catch (error) {
      if (revision !== this.#revision) return;
      this.shadowRoot.textContent = '발음 표시를 불러오지 못했습니다. 연결을 확인해 주세요.';
      this.dispatchEvent(new CustomEvent('mh-error', {detail:error}));
    }
  }
}
if (!customElements.get('mark-hangeul')) customElements.define('mark-hangeul', MarkHangeulPreview);
