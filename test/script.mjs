import { Worker, isMainThread, workerData } from 'node:worker_threads';
import * as url from 'node:url';

const __filename = url.fileURLToPath(import.meta.url);
const THREAD_ID = workerData || 0
const {
    THREADS = 1,
    COUNT = 100_000_000,
    LOOPS = 3,
} = process.env


if (isMainThread) {
    for (let i = 1; i < THREADS; i++) {
        new Worker(__filename, { workerData: i })
    }
}

const sleep = d => new Promise(res => setTimeout(res, d));

console.log(`[${THREAD_ID}] START`)

let buffer = []

for (let i = 0; i < LOOPS; i++) {
    console.log(`[${THREAD_ID}] ${i}`)
    for (let i = 0; i < COUNT; i++) {
        buffer.push(i)
    }
    await sleep(1000)
    buffer = []
}

console.log(`[${THREAD_ID}] DONE`)
