import { Render } from "../controllers/render.js"
import { ApiDoc } from "../controllers/apidoc.js"

const routes = [
	{
		path: '/about',
		component: ApiDoc
	},
	{
		path: '/engine',
		component: Render
	},
]

const router = new VueRouter({
	mode: 'history',
	routes
})

export { router }
