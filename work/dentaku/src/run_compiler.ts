import * as readline from 'readline';
import * as compiler from './compiler';

process.stdin.resume();
process.stdin.setEncoding('utf8');
let line: string = '';

let reader = readline.createInterface({
	input: process.stdin,
	output: process.stdout
});



reader.on('line', (line)=>{
	console.log(JSON.stringify(compiler.compile(line), undefined, 2));
});


