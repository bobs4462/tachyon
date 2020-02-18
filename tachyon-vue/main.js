import Vue from 'https://cdn.jsdelivr.net/npm/vue@latest/dist/vue.esm.browser.min.js'

Vue.use(VueRouter)

const router = new VueRouter({
  routes: []
})

new Vue({
	el: '#app',
	data: function() {
		return { visible: false }
	},
	methods: {
		handleOpen(key, keyPath) {
			console.log(key, keyPath);
		},
		handleClose(key, keyPath) {
			console.log(key, keyPath);
		}
	},
	router
})
