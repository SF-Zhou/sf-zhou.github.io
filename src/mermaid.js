import puppeteer from 'puppeteer';
import { renderMermaid } from '@mermaid-js/mermaid-cli';

let browser_promise = null;

function get_browser() {
  if (!browser_promise) {
    browser_promise = puppeteer.launch({
      headless: 'shell',
      args: ['--no-sandbox', '--disable-setuid-sandbox'],
    });
  }
  return browser_promise;
}

export async function render_mermaid(definition, svg_id, theme) {
  const browser = await get_browser();
  const { data } = await renderMermaid(browser, definition, 'svg', {
    svgId: svg_id,
    backgroundColor: 'transparent',
    mermaidConfig: { theme },
  });
  return Buffer.from(data).toString();
}

export async function close_mermaid() {
  if (browser_promise) {
    const browser = await browser_promise;
    browser_promise = null;
    await browser.close();
  }
}
