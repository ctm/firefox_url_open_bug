import init from './pkg/yew_firefox_bug_demo.js';

window.addEventListener('load', async () => {
    await init('./pkg/yew_firefox_bug_demo_bg.wasm');
});
