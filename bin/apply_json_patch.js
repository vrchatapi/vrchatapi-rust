#!/usr/bin/env node

import { applyPatch } from 'rfc6902'
import { parse, stringify } from 'yaml'

import fs, { cp } from 'node:fs';

const helpText = 'Usage: apply_json_patch.js <patch_file> <source_file> [output_file]'

var args = process.argv.slice(2);

if (args.find(arg => arg === '--help')) {
    console.log(helpText)
    process.exit(0)
}

if (args.length < 2) {
    console.error('Missing arguments')
    console.log(helpText)
    process.exit(1)
}

const patchFile = args[0]
const sourceFile = args[1]
const outputFile = args[2] || sourceFile

try {
    const patch = JSON.parse(fs.readFileSync(patchFile, 'utf8'))
    const source = fs.readFileSync(sourceFile, 'utf8')
    
    const sourceJson = parse(source)
    const results = applyPatch(sourceJson, patch)

    results.forEach((result, i) => {
        if (result !== null) {
            throw new Error('Patch failed at operation ' + i)
        }
    })
    
    fs.writeFileSync(outputFile, stringify(sourceJson, { singleQuote: false, lineWidth: 0, }))
} catch (err) {
    console.error(err)
}
