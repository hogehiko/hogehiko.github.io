console.log('union');

let a: number | string = 1
let b: [number, string] 

b = a;

function f(): [number, string] {
    return [1,'a']
}

const x = f();