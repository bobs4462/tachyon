const template = `
<el-container>
<el-main style="padding=5%">
<el-card>
<v-jsoneditor 
v-model="json" :options="options" :plus="false" :height="height" @error="onError"></v-jsoneditor>
</el-card>
</el-main>
</el-container>

`

var Render = Vue.component('render', {
	template: template,
	data() {
		return {
			options: {
				sortObjectKeys: true,
				history: true,
				mode: 'code',
				name: 'data',
				modes: ['tree', 'form', 'code'],
			},
			json: {
				"hello": "world"
			}
		}
	},
	computed: {
		height: function() {
			console.log("height")
			return window.screen.height * 0.6 + "px"
		}
	},
	methods: {
		onError() {
			console.log('error')
		}
	}
})

export { Render }
