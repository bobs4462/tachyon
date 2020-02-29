const template = `
<el-container>
	<el-main>
		<div style="flex">
			<el-card style="margin-right: 2%; width: 100%">
				<el-table :data="templates" ref="T" 
																		border
																		stripe @expand-change="templateGet" :max-height="table_height">
					<el-table-column label="Название" width="250px">
						<template slot-scope="scope">
							<div style="word-break: break-word">
								{{ scope.row.name }}
							</div>
						</template>
					</el-table-column>
					<el-table-column label="Файл" width="200px">
						<template slot-scope="scope">
							<div style="word-break: break-word">
								{{ scope.row.file }}
							</div>
						</template>
					</el-table-column>
					<el-table-column label="Комментарий">
						<template slot-scope="scope">
							<div style="word-break: break-word">
								{{ scope.row.comment }}
							</div>
						</template>
					</el-table-column>
					<el-table-column label="Размер" prop="size" width="70"></el-table-column>
					<el-table-column label="Шаблон" type="expand" width="90">
						<template slot-scope="scope">
							<div>
							<vue-prism-editor
							v-model="content"
							:code="code" :language="html" :lineNumbers="true"></vue-prism-editor>
							</div>
							<div style="margin-top: 2%; text-align: center">
							<el-button size="mini" @click="templateSave(scope.row.file)" 
							type="primary" icon="el-icon-check">Сохранить изменения</el-button>
							</div>
						</template>
					</el-table-column>
				</el-table>
			</el-card>
		</div>
	</el-main>
</el-container>
`

var Templates= Vue.component("templates", {
	template: template,
	data: function() {
		return {
			templates: [],
			content: null,
			code: 'const a = 10',
			html: 'html',
			// content: 'Hello WORLD',
			// current_file: null,
			// editorOption: {
			// 	placeholder: "Введите текст шаблона",
			// 	modules: {
			// 		toolbar: [
			// 			['bold', 'italic', 'underline', 'strike'],
			// 			[{ 'list': 'ordered'}, { 'list': 'bullet' }],
			// 			[{ 'header': [1, 2, 3, false] }],
			// 			[{ 'font': [] }],
			// 			['clean']
			// 		]
			// 	}
			// }
		}
	},
	mounted() {
			axios.get("/list").then(
				resp => {
					this.templates = resp.data
				}).catch((e) => { console.log(e) })
	},
	methods: {
		templateGet(row, expanded) {
			this.content = null
			for (let i = 0; i < expanded.length; i += 1) {
				if (row.file !== expanded[i].file) {
					this.$refs['T'].toggleRowExpansion(expanded[i])
				}
			}
			this.current_file = row.file
			axios.get(`/raw/${row.file}`).then(
				resp => {
					this.content = resp.data
				}).catch((e) => { console.log(e) })
		},

		onTemplateChange({ editor, html, text }) {
			console.log(html)
		},

		templateSave(file_name) {
			axios.post(`/add/${this.current_file}`, this.content).then(
				resp => {
					this.$message.success(`Шаблон ${this.current_file} успешно сохранён`)
				}).catch((e) => { console.log(e) })
		}
	},
	computed: {
		table_height: function() {
			return window.screen.height * 0.63
		}
	},
});

export { Templates }
