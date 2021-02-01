const pegjs = require('pegjs');
const fs = require('fs');

export function makeParser(){
	console.log("generated")
	const source = fs.readFileSync(__dirname + '/../src/grammer.pegjs', {
		encoding: 'utf8',
	});
	return pegjs.generate(source);
}

const parser = makeParser();

export type VariableDef = {
	values: number[],
	varTable: {
		name: string,
		location: {
			start:{
				line: number,
				offset:number,
				column: number	
			},
			end:{
				line: number,
				offset:number,
				column: number
			}
		},
		value: number
	}
};

export type ErrorDef = {
	location: {
		start:{
			line: number,
			offset:number,
			column: number	
		},
		end:{
			line: number,
			offset:number,
			column: number
		}
	},
	message:string
}

export function compile(src: string): {
	values: number[],
	varTable: VariableDef[],
	errors:ErrorDef[]
}
{
	try{
		return parser.parse(src);
	}catch(e){
		return {
			values: [],
			varTable: [],
			errors:[
				{
					location:e.location,
					message: e.message
				}
			]
		}
	}
}