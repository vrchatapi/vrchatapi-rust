#!/usr/bin/env node

import { createPatch } from 'rfc6902'
import { parse } from 'yaml'
import fs from 'node:fs';

const helpText = 'Usage: gen_patch.js <actual_file> <patched_file> <output_file>'

var args = process.argv.slice(2);

if (args.find(arg => arg === '--help')) {
    console.log(helpText)
    process.exit(0)
}

if (args.length < 3) {
    console.error('Missing arguments')
    console.log(helpText)
    process.exit(1)
}

const actualFile = args[0]
const patchedFile = args[1]
const outputFile = args[2]

try {
    const actual = fs.readFileSync(actualFile, 'utf8')
    const patched = fs.readFileSync(patchedFile, 'utf8')

    const actualJson = parse(actual)
    const patchedJson = parse(patched)

    const patch = createPatch(actualJson, patchedJson)

    fs.writeFileSync(outputFile, JSON.stringify(patch, null, 2))
} catch (err) {
    console.error(err)
}
