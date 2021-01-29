const pegjs = require('pegjs');
const fs = require('fs');

export function makeParser(){
	const source = fs.readFileSync(__dirname + '/../src/grammer.pegjs', {
		encoding: 'utf8',
	});
	return pegjs.generate(source);
}

export function compile(src: string): {
	values: number[],
	varTable: {
		name: string,
		location: {
			line: number,
			offset:number,
			column: number
		},
		value: number
	}[],
	errors:{
		line:number, offset:number, column: number,
		message:string
	}[]
}

{
	const parser = makeParser();
	return parser.parse(src);
}