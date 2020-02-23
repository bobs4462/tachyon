const template = `
<el-container>
	<el-main>
		<el-card>
			<h2>Шаблоны</h2>
			<el-card>
				<h3>Введение</h3>

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
			</el-card>
			<el-card>
				<h3>Без отрисовки</h3>
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
			</el-card>
		</el-card>
		<el-card>
			<iframe src="https://tera.netlify.com/docs/" 
							frameBorder="0"
							width="100%" height="700px"></iframe>
		</el-card>
	</el-main>
</el-container>

`
var ApiDoc = Vue.component("api-doc", {
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

export { ApiDoc }
