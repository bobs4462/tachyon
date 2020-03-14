const template = `
<el-container>
	<el-main>
		<el-card>
			<el-collapse accordion>
				<el-collapse-item title="Введение">
				<div>
					Любой шаблон является просто текстовым файлом, в котором переменные и выражения будут 
					заменены значениями, при отрисовке (рендере). Синтаксис основан на шаблонах Jinja2 и Django.
				</div>
				<div>
					Существует 3 вида разделителей, которые нельзя изменить:
				</div>
				<div>
					<ul>
						<li><code>{{</code> и <code>}}</code> для выражений</li>
						<li><code>{%</code> или <code>{%-</code> и <code>%}</code> или <code>-%}</code> для операторов</li>
						<li><code>{#</code> и <code>#}</code> для комментариев</li>
					</ul>
				</div>
				</el-collapse-item>

				<el-collapse-item title="Без отрисовки">
					<div>
					Весь текст внутри блока raw как строку и не будет пытаться отобразить
					то, что внутри. Полезно, если у вас есть текст, содержащий разделители.
				</div>
				<div>
					<p><code>{%</code><span style="color:#f92672;">raw </span><code>%}</code></p>
					<p><code>Здравствуйте {{</code><code>name}}</code></p>
					<p><code>{%</code><span style="color:#f92672;">endraw</span><code>%}</code></p>
				</div>
				<p>будет отображаться как <code>Здравствуйте {{</code><code>name}}</code>.</p>
				</el-collapse-item>
				<el-collapse-item title="Контроль пробельных символов">
				<div>
					Tera поставляется с простым в использовании управлением пробелами:
					используйте {%- если вы хотите удалить все пробелы перед оператором и
				-%} если вы хотите удалить все пробелы после.  
				</div>
				<div>Например, давайте посмотрим на следующий шаблон:</div>

				<div v-pre class="preformat">
				{% set my_var = 2%}<br/>
				{{my_var}}
				</div>
				<div>
				будет иметь следующий вывод:
				</div>
				<div v-pre class="preformat">
					2<br></br>
				</div pre>
 
				<div>
Если мы хотим избавиться от пустой строки, мы можем написать следующее:
				</div>

				<div v-pre class="preformat">

					{% set my_var = 2 -%}<br/>
		      {{my_var}}

				</div>
				</el-collapse-item>
				<el-collapse-item title="Оригинальная документация">
			<iframe src="/html/tera_docs.html" 
							frameBorder="0"
							width="100%" height="300px"></iframe>

				</el-collapse-item>
				</el-collapse>
		</el-card>
	</el-main>
</el-container>

`
var SyntaxDoc = Vue.component("syntax-doc", {
	template: template,
	data: function() {
		return {
			h: false
		}
	},
	mounted() {
		console.log('hello');
	},
	methods: {},
	computed: {},
});

export { SyntaxDoc }
