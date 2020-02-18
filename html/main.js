import Vue from 'https://cdn.jsdelivr.net/npm/vue@latest/dist/vue.esm.browser.min.js'

import {
	Navbar
} from './components/navbar.js'

import {
	MainTemplate
} from './templates/main-template.js'

import { About } from './components/render.js'

Vue.use(VueRouter)

const router = new VueRouter({
	routes: [
		{
			path: '/about',
			component: About,
			name: "About Us Page"
		},
		{
			path: '/render',
			component: RenderTemplate,
			name: 'render'
		}]
})

new Vue({
	el: '#app', // This should be the same as your <div id=""> from earlier.
	components: {
		'navbar': Navbar
	},
	router,
	template: MainTemplate
})
