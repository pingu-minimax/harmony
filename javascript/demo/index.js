// CommonJS implementation
const { HarmonyEncoding } = require('@openai/harmony');

function initHarmony() {
    console.log('Harmony initialized with CommonJS');
    return new HarmonyEncoding();
}

module.exports = { initHarmony };