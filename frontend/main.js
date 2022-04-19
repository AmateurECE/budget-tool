import init, { run_app } from './pkg/budget_frontend.js';
async function main() {
   await init('/pkg/budget_frontend_bg.wasm');
   run_app();
}
main()
