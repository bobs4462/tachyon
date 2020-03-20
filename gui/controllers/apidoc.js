const template = `
<el-container>
	<el-main>
		<el-card>
			<span style="font-size: 1.1em; margin-bottom: 10px">
				Сервис предоставляет несколько методов в своём API, воспользоваться
				которыми можно с исопользование HTTP запросов. Далее приведены все
				доступные методы, предоставляемых API.
			</span> 
			<el-collapse accordion>
				<el-collapse-item title="Получение шаблона">
					<div>
						<b>Метод:</b> /raw/{ название файла шаблона }
					</div>
					<div>
						<b>Тип запроса:</b> GET
					</div>
					<div>
						<b>Параметры:</b> отсуствуют
					</div>
					<div>
						<b>Тип ответа:</b> строковый
					</div>
					<div>
						<b>Содержимое ответа:</b> запрошенный шаблон
					</div>
					<div>
						<b>Возможные ошибки:</b>
						<div>
							<ul>
								<li><b>Код: 404</b>. Причина: такой шаблона не существует</li>
							</ul>
						</div>
					</div>
				</el-collapse-item>
				<el-collapse-item title="Построение документа">
				</el-collapse-item>
				<el-collapse-item title="Проверка шаблона">
				</el-collapse-item>
				<el-collapse-item title="Создание/Обновление шаблона">
				</el-collapse-item>
				<el-collapse-item title="Список существующих шаблонов">
				</el-collapse-item>
				<el-collapse-item title="Ручное обновление кеша">
				</el-collapse-item>
			</el-collapse>
		</el-card>
	</el-main>
</el-container>

`

var API = Vue.component("api-doc", {
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

export { API }
