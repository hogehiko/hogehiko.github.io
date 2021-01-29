{
	const varTable= [];
	const errors = [];
}

script = declear_var* e:expression*
{
	return {values:e, varTable, errors};
}

NUMBER = s:([1-9][0-9]*){
	return parseInt(s);
}

IDENFITIER = id:([a-z]+){
	return id.join("");
}


declear_var = 
	'let' _ 
	id:(
		id:IDENFITIER
		{
			const l = location();
            return [id, l]
		}
	) 
	_  '=' _ num:expression 
	{
		
		varTable.push({name: id[0], location: id[1], value: num});
	}

variable = name:IDENFITIER _ {
	const l = location();
	let vars = varTable.filter(v=>v.name === name);
	if(vars.length === 1){
		return vars[0].value;
	}else{
		errors.push({
			message: 'No such variable',
			line: l.start.line,
			offset: l.start.offset,
			column: l.start.column
		});
		return 0;
	}
}

primary_exp = n:NUMBER _{return n;} / n:variable{return n;}

muldiv_exp = n:primary_exp rights:(  ('*' / '/')  _  primary_exp )*
{
	let v = n;

	for(let r of rights){
		if(r[0] === '*'){
			v = v * r[2]
		}else{
			v = v / r[2]
		}
	}
	return v;
}

expression = n:muldiv_exp rights:( ('+' / '-') _ muldiv_exp )*
{
	let v = n;

	for(let r of rights){
		if(r[0] === '+'){
			v = v + r[2]
		}else{
			v = v - r[2]
		}
	}
	return v;
}


_ = [ \n]*

