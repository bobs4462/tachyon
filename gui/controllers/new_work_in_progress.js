const template = `
<el-container>
	<el-main style="margin: 10%">
		<el-card>
		<h2>Раздел находится в работе</h2>
		</el-card>
	</el-main>
</el-container>
`

var NewWorkInProgress = Vue.component("new-work-in-progress", {
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

export { NewWorkInProgress }
