let wasmModule = null;

// Compile and run BDL code
async function compileBDL() {
    if (!wasmModule) {
        console.error('WASM not initialized');
        return;
    }

    const bdlCode = document.getElementById('bdl-input').value;
    const stdin = document.getElementById('stdin').value;

    try {
        // Clear previous outputs
        document.getElementById('cpp-output').value = '';
        document.getElementById('program-output').value = '';

        // Call the compile function directly with strings
        const result = wasmModule.compile_and_run(bdlCode, stdin);
        
        // Display the result
        document.getElementById('cpp-output').value = result;
    } catch (error) {
        console.error('Compilation failed:', error);
        document.getElementById('program-output').value = 'Error: ' + error.message;
    }
}

// Make compileBDL available globally
window.compileBDL = compileBDL;

// Initialize the WASM module
async function init() {
    console.log("Initializing WASM module...");
    try {
        // Import the wasm-bindgen generated JavaScript
        const wasm = await import('./web/wasm/bdl_web.js');
        // Initialize the module
        await wasm.default();
        wasmModule = wasm;
        console.log("WASM module initialized successfully");
    } catch (error) {
        console.error('Failed to load WASM:', error);
    }
}



// Initialize when the page loads
init();
