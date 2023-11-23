const sleep = d => new Promise(res => setTimeout(res, d));

console.log('Starting')

for (let i = 0; i < 5; i++) {
    console.log(i)
    for (let i = 0; i < 1_000_000_000; i++) {
    }
    await sleep(1000)
}

console.log('Done')
