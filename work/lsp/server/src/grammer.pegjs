{
	const varTable = [];
}

script = declear_var* e:expression*
{
	return [e, varTable];
}

NUMBER = s:([1-9][0-9]*){
	return parseInt(s);
}

IDENFITIER = id:([a-z]+){
	return id.join("");
}


declear_var = 'let' _ name:IDENFITIER _  '=' _ num:expression {
	varTable.push([name, num]);
}

variable = name:IDENFITIER _ {
	let vars = varTable.filter(v=>v[0] === name);
	if(vars.length === 1){
		return vars[0][1];
	}else{
		throw new Error("No such variable: "+JSON.stringify(varTable) );
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

