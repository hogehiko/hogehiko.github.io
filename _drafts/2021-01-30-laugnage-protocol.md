---
layout: post
title:  自作言語のLanguage Serverを作る
date:   2021-01-30 00:30:00 +0900
categories: programming lsp
---

プログラミング言語は強力なエディタ・IDEのサポートがあってはじめて実力を発揮します。
自作のプログラミング言語にエディタサポートを付けるため、Language Protocol の Client/Server を VSCode プラグインとして実装してみます。

# Language Server Protocol (LSP)

Language Serverはクライアント（エディタ等）からの問い合わせに対し、自動保管やコードジャンプの情報を返すサーバです。
対応する機能の一部を列挙しましょう。

* completion
* definition
* rename
* formatting
* 他多数

普段使う機能は一通り揃っているようです。

通信はJSON-RPCで実現されます。プロトコルの詳細については本稿では扱いません。


# LSP サンプルを実行

microsoftのvscode-extension-samples リポジトリにある lsp-sample は、clone してvscodeを開くだけで Language Server を含む VSCode言語サポートの開発を始めることができます。

サンプルで実装されている機能を見てみましょう。

## 自動補完

server/src/server.ts
onCompletionメソッドを使って自動補完時のレスポンスを定義しています。

```typescript
connection.onCompletion(
	(_textDocumentPosition: TextDocumentPositionParams): CompletionItem[] => {
		// The pass parameter contains the position of the text document in
		// which code complete got requested. For the example we ignore this
		// info and always provide the same completion items.
		return [
			{
				label: 'TypeScript',
				kind: CompletionItemKind.Text,
				data: 1
			},
			{
				label: 'JavaScript',
				kind: CompletionItemKind.Text,
				data: 2
			}
		];
	}
);


```
ワードを指定するだけで良いのですね。_textDocumentPosition でキャレットの位置が取れるので、コンテキストに沿って保管候補を絞り込むことも可能です。
補完対象を選択した際に表示する情報は、onCompletionResolved で定義します。
```typescript
// This handler resolves additional information for the item selected in
// the completion list.
connection.onCompletionResolve(
	(item: CompletionItem): CompletionItem => {
		if (item.data === 1) {
			item.detail = 'TypeScript details';
			item.documentation = 'TypeScript documentation';
		} else if (item.data === 2) {
			item.detail = 'JavaScript details';
			item.documentation = 'JavaScript documentation';
		}
		return item;
	}
);
```
completionItemに合わせて必要なものを返します。

## エラー表示

次にエラー表示の実装を見てみましょう。
sendDiagnostics メソッドでエラー情報を設定できます。onDidChangeContent でソースコードの変更があるたびにエラー位置を変更しています。

```typescript
documents.onDidChangeContent(change => {
	validateTextDocument(change.document);
});

async function validateTextDocument(textDocument: TextDocument): Promise<void> {
	// In this simple example we get the settings for every validate run.
	let settings = await getDocumentSettings(textDocument.uri);

	// The validator creates diagnostics for all uppercase words length 2 and more
	let text = textDocument.getText();
	let pattern = /\b[A-Z]{2,}\b/g;
	let m: RegExpExecArray | null;

	let problems = 0;
	let diagnostics: Diagnostic[] = [];
	while ((m = pattern.exec(text)) && problems < settings.maxNumberOfProblems) {
		problems++;
		let diagnostic: Diagnostic = {
			severity: DiagnosticSeverity.Warning,
			range: {
				start: textDocument.positionAt(m.index),
				end: textDocument.positionAt(m.index + m[0].length)
			},
			message: `${m[0]} is all uppercase.`,
			source: 'ex'
		};
		if (hasDiagnosticRelatedInformationCapability) {
			diagnostic.relatedInformation = [
				{
					location: {
						uri: textDocument.uri,
						range: Object.assign({}, diagnostic.range)
					},
					message: 'Spelling matters'
				},
				{
					location: {
						uri: textDocument.uri,
						range: Object.assign({}, diagnostic.range)
					},
					message: 'Particularly for names'
				}
			];
		}
		diagnostics.push(diagnostic);
	}

	// Send the computed diagnostics to VSCode.
	connection.sendDiagnostics({ uri: textDocument.uri, diagnostics });
}


```

# 自作言語の組み込み


今回は自作の電卓の Language Serverを実装してみることにします。
言語は PEG.jsで実装します。

```pegjs
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
			location: l
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
```

```
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
```


## オートコンプリートを実装

## エラー表示を実装

a