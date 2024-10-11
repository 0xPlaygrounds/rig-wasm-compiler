import init, { WasmAgent, run_rag_example } from '../pkg/rig_wasm.js';

async function main() {
    await init();

    const agent = new WasmAgent('{"api_key": "your-api-key-here", "model": "gpt-3.5-turbo"}');
    const response = await agent.process("Hello, who are you?");
    console.log("Agent response:", response);

    await run_rag_example();
}

main().catch(console.error);