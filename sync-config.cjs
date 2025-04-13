const fs = require('fs');
const path = require('path');

const packageJsonPath = path.resolve(__dirname, 'package.json');
const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));

const tauriConfigPath = path.resolve(__dirname, 'src-tauri/tauri.conf.json');
const tauriConfig = JSON.parse(fs.readFileSync(tauriConfigPath, 'utf8'));

tauriConfig.productName = packageJson.name;
tauriConfig.version = packageJson.version;
tauriConfig.identifier = packageJson.identifier;

fs.writeFileSync(
    tauriConfigPath,
    JSON.stringify(tauriConfig, null, 2) + '\n'
);

console.log("Tauri config updated with package.json values.");