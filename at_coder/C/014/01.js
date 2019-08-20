function main(input) {
    var args = input.split('\n');
    var n = parseInt(args[0], 10);

    var s = [];
    var i;
    for (i = 0; i <= 1000002; i++) {
        s.push(0);
    }

    var arg;
    var a;
    var b;
    for (i = 1; i <= n; i++) {
        arg = args[i].split(' ');
        a = parseInt(arg[0], 10);
        b = parseInt(arg[1], 10);
        s[a]++;
        s[b+1]--;
    }

    for (i = 1; i < 1000001; i++) {
        s[i] += s[i-1];
    }

    var max = 0;
    for (i = 0; i <= 1000001; i++) {
        max = Math.max(max, s[i]);
    }
    console.log(max);

}

main(require('fs').readFileSync('/dev/stdin', 'utf8'));
