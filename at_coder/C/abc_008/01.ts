function float2int (value) {
    return value | 0;
}

"use strict";
function main(input) {
    const args = input.split('\n');
    const n = parseInt(args[0], 10);
    let a = [];
    for (let i = 1; i <= n; i++) {
        a.push(args[i]);
    }
    let ans = 0.0;
    for (let i = 0; i < n; i++) {
        let cnt = 0;
        for (let j = 0; j < n; j++) {
            if (a[j] % a[i] === 0) cnt++;
        }
        ans += float2int((cnt + 1) / 2) / cnt;
    }
    console.log(ans);
}

main(require('fs').readFileSync('/dev/stdin', 'utf8'));
