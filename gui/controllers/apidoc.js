var ApiDoc = Vue.component("api-doc", {
	template: `
<el-container style="height: 100%">
<el-main>
<div>Hello world</div>
</el-main>
</el-container>
	`,
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

