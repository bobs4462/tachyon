const template = `
<el-container>
	<el-main>
	</el-main>
</el-container>
`

var Templates= Vue.component("templates", {
	template: template,
	data: function() {
		return {
			templates: [],
			image_wide: [{src: 'images/html-tag.png', name: 'html.png'}],
			all_templates: [],
		}
	},
	mounted() {
			axios.get("/list").then(
				resp => {
					this.all_templates = resp.data
					this.templates = this.all_templates.slice(0, 9)
				}).catch((e) => { console.log(e) })
	},
	methods: {
			handlePictureCardPreview(file) {}
	},
	computed: {},
});

export { Templates }
