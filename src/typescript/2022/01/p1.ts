export function p1(input: string) {
    const contents = input.split('\n');
    let count = 0;
    let max = 0;
    for (let i = 0; i < contents.length; i++) {
        count += +contents[i];
        if (count > max) max = count;
        if (!contents[i]) count = 0;
    }
    return max;
}


