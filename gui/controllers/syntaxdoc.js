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
				<el-collapse-item title="Комментарии">
					<div>
						Для комментариев в шаблонах поместите текст между {# и #}.
					</div>

					<div v-pre class="preformat">
						{# Это комментарий #}
					</div>
				</el-collapse-item>
				<el-collapse-item title="Структуры данных">
					<el-collapse accordion>
						<el-collapse-item title="Литералы">
							<div>
								Тера имеет следующие литералы
							</div>
							<div>
								<ul>
									<li><b>Булевы:</b> true или false</li>
									<li><b>Целочисленные</b></li>
									<li><b>С плавающей точкой</b></li>
									<li><b>Строки:</b> текст находищийся между двойными или одинарными ковычками</li>
									<li><b>Массивы:</b> литералы находищийся <code>[</code> и <code>]</code> и разделенных запятыми</li>
								</ul>
							</div>
						</el-collapse-item>
						<el-collapse-item title="Переменные">
							<div>Для отображения переменных контекста необходимо обернуть нужную переменную между <code>{{</code> и <code>}}</code></div>
							<div>Если попытаться отобразить переменную которая не существует, то это приведёт к ошибке построения документа</div>
						</el-collapse-item>
						<el-collapse-item title="Доступ к членам структуры данных">
							<div>Для получения доступе к члену структуры данных существует 2 способа</div>
							<div><ol>
									<li><b>Точечная нотация:</b> например
										<div v-pre class="preformat">
											{{ product.name }}
										</div>
									</li>
									<li><b>Нотация квадратных скобок:</b> например
										<div v-pre class="preformat">
											{{ product['name'] }}
										</div>
									</li>
								</ol></div>
								<div>
									Для получения доступа к элементам массива также можно использовать обе 
									нотации, где после точки или внутри квадратных скобок должен быть индекс элемента
								</div>
						</el-collapse-item>
					</el-collapse>
				</el-collapse-item>
				<el-collapse-item title="Выражения">
					<div>
						Выражения можно использовать почти везде, существует несколько видов выражений:
					</div>
					<div>
						<ul>
							<li><b>Математические:</b>сложение, вычитание, умножение, деление, деление по модулю
								<div>Операндами этих опереций могут быть только числа</div>
							</li>
							<li><b>Сравнения</b></li>
							<li><b>Логические:</b> and, or, not</li>
							<li><b>Соеденение строк:</b> используя оператор конкатенации <b>~</b>, например
								<div v-pre class="preformat">
									{{ product.name ~ " is the best " in city product.city }}
								</div>
							</li>
							<li><b>Вхождение во множество:</b> используя оператор вхождения <b>in</b>, например
								<div v-pre class="preformat">
									{{ product.name in ["cola", "sprite", "fanta"] }}
								</div>
							</li>
						</ul>
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
