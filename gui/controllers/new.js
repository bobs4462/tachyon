const template = `
<el-container style="height: 100%">
	<el-main style="padding=5%">
		<div style="display: flex">
			<el-card style="width: 61%">
				<div>
					<vue-prism-editor
						style="overflow: auto; max-height: 495px; height: 490px"
						:readonly="disabled"
						v-model="content"
						:language="html" code="<html></html>":lineNumbers="true"></vue-prism-editor>
				</div>
				<div style="margin-top: 2%; text-align: center; display: flex">
					<el-tooltip content="Текст, размер до 1МБ">
						<el-upload :before-upload="htmlSet"
											action="#"
											:limit="1"
											style="margin-right: 1%"
											:show-file-list="false"
											ref="upload">		
							<el-button icon="el-icon-upload"
												 slot="trigger"
												 type="success"
												 size="mini">Шаблон из файла</el-button>
						</el-upload>
					</el-tooltip>
					<el-button style="margin-left: 2%"
										 icon="el-icon-check"
										 @click="save" size="mini" type="primary">Cохранить шаблон</el-button>

					<el-button style="margin-left: 2%"
										 icon="el-icon-refresh"
										 @click="reload"
										 size="mini" type="danger">Перезагрузить шаблоны</el-button>
					<el-switch style="margin-left: 2%; margin-top: 1%" active-text="Блокировать редактор"
											v-model="disabled"></el-switch>
				</div>

			</el-card>
			<el-container direction="vertical"  style="width: 37%; margin-left: 2%">
				<el-card>
					<el-container direction="vertical">				
						<div style="display: flex">
							<el-input size="mini"
												style="margin-right: 1%; margin-bottom: 1%"
												placeholder="Название шаблона" v-model="template.name"></el-input>
							<el-input size="mini" placeholder="Название файла (с расширением)" v-model="template.file"></el-input>
						</div>
						<el-input size="mini" type="textarea" autosize placeholder="Комментарий" v-model="template.comment"></el-input>
					</el-container>

				</el-card>
				<el-card style="margin-top: 1%">
					<v-jsoneditor 
								 v-model="template.data"
								 :options="options" :plus="true" :height="height" @error="onError">
					</v-jsoneditor>
					<el-tooltip content="JSON, размер до 1МБ">
						<el-upload :before-upload="jsonSet"
											style="margin-top: 2%"
											action="#"
											:show-file-list="false"
											:limit="1"
											ref="upload">		
							<el-button icon="el-icon-upload"
												 slot="trigger"
												 type="success"
												 size="mini">JSON из файла</el-button>
						</el-upload>
					</el-tooltip>
				</el-card>
			</el-container>
		</div>
	</el-main>
</el-container>
`

var New = Vue.component("new", {
	template: template,
	data: function() {
		return {
			content: null,
			disabled: false,
			html: 'html',
			template: {
				name: null,
				file: null,
				size: 0,
				comment: null,
				data: {},
			},
			options: {
				sortObjectKeys: true,
				history: true,
				mode: 'code',
				name: 'data',
				modes: ['tree', 'code'],
			},
		}
	},
	mounted() {
	},
	methods: {
		onError(e) { },

		htmlSet(file) {
			let type = new RegExp(/^text\/*/)
			if (!type.test(file.type)) {
				this.$message.error("Тип файла не текстовый")
				return false
			}
			if (file.size > 1048576) {
				this.$message.error("Размер файла больше 1МБ")
				return false
			}
			file.text().then(resp => {
				this.content = resp
				this.template.file = file.name
			}).catch((e) => { this.$message.error(e) 
			})
		},

		jsonSet(file) {
			if (file.type !== 'application/json') {
				this.$message.error("Тип файла не json")
				return false
			}
			if (file.size > 1048576) {
				this.$message.error("Размер файла больше 1МБ")
				return false
			}
			file.text().then(resp => {
				this.template.data = JSON.parse(resp)
				return true
			}).catch((e) => { 
				this.$message.error(e) 
			})
		},

		reload() {
			axios.get("/reload").then(
				resp => {
					this.$message.success("Шаблоны успешно перезагружены")
				}).catch((e) => { this.$message.error(e) })
		},

		save() {
			if (!this.template.name) {
				this.$message.warning("Введите название шаблона")
			}
			if (!this.template.file) {
				this.$message.warning("Введите название файла")
			}
			axios.post(`/add/${this.template.file}`, this.template).then(
				resp1 => {
					if (resp1.status === 200) {
						axios.put(`/add/${this.template.file}`, this.content).then(
							resp2	=> {
								if (resp1.status === 200) {
									this.$message.success("Шаблон успешно сохранён")
									setTimeout(() => { this.reload() }, 1000)
								}
							}).catch((e) => { console.log(e) })
					}
				}).catch((e) => { console.log(e) })
		},
	},

	computed: {
		height: function() {
			return window.screen.height * 0.42 + "px"
		},
	},
});

export { New }
