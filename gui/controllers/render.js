const template = `
<el-container style="height: 100%">
	<el-main style="padding=5%">
		<div style="display: flex">
			<el-card style="width: 78%">
				<v-jsoneditor 
							 v-model="json" :options="options" :plus="false" :height="height" @error="onError">
				</v-jsoneditor>
			</el-card>
			<el-card style="width: 20%; margin-left: 2%">
			<el-container direction="vertical">
				<el-upload 
							 :before-upload="jsonSet"
							 action="#"
							 :limit="1"
							 ref="upload">		
					<el-button 
							 icon="el-icon-upload"
							 slot="trigger"
							 type="success"
							 size="mini">Загрузить JSON из файла</el-button>
					<div class="el-upload__tip" style="margin-left: 2%" slot="tip">формат JSON, размер до 1МБ</div>
				</el-upload>
				<el-select 
				@change="pick"
				style="margin-top: 7%" placeholder="Выбрать шаблон" v-model="template" size="mini">
					<el-option v-for="t in templates" :key="t.file" :value="t.file" :label="t.name"></el-option>
				</el-select>
				<el-button style="margin-top: 7%; margin-bottom: 7%"
									 icon="el-icon-view"
									 @click="render" size="mini" type="primary">Сгенерировать документ</el-button>

				<el-button style="margin-left: 0"
									 icon="el-icon-refresh"
									 @click="reload"
									 size="mini" type="danger">Перезагрузить шаблоны</el-button>
									 </el-container>
			</el-card>
		</div>
	</el-main>
	<el-drawer
		:title="drawer_title"
		:visible.sync="drawer"
		size="70%"
		style="height: 100%"
		direction="rtl">
		<el-card style="margin: 1%;">
			<div style="height: 570px; overflow: auto"
					 v-html="rendered"></div>
		</el-card>
	</el-drawer>
</el-container>
`

var Render = Vue.component('render', {
	template: template,
	data() {
		return {
			templates: [],
			template: null,
			drawer: false,
			rendered: 'hello world',
			json: {},
			options: {
				sortObjectKeys: true,
				history: true,
				mode: 'code',
				name: 'data',
				modes: ['tree', 'code'],
			},
		}
	},
	computed: {
		height: function() {
			console.log("height")
			return window.screen.height * 0.63 + "px"
		},
		drawer_title: function() {
			return "Шаблон " + this.template
		},
	},

	beforeMount() {
		axios.get("/list").then(
			resp => {
				this.templates = resp.data
				console.log(this.templates)
			}).catch((e) => { this.$message.error(e) })
	},
	methods: {
		onError(e) { },

		pick(file) {
			this.json = this.templates.find(i => { return i.file === file }).data
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
				this.json = JSON.parse(resp)
				return true
			}).catch((e) => { this.$message.error(e) })
		},

		reload() {
			axios.get("/reload").then(
				resp => {
					this.$message.success("Шаблоны успешно перезагружены")
				}).catch((e) => { this.$message.error(e) })
		},

		render() {
			if (!this.template) {
				this.$message.error("Пожалуйста выберите шаблон")
				return
			}
			axios.post("/render/" + this.template, this.json).then(
				resp => {
					console.log(resp.data)
					this.rendered = resp.data
					this.drawer = true
				}).catch((e) => { this.$message.error(e)  })
		},

		log() {
			console.log(this.$refs.upload)
		}
	}
})

export { Render }
