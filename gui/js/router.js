import { Render } from "../controllers/render.js"
import { SyntaxDoc } from "../controllers/syntaxdoc.js"
import { APIWorkInProgress } from "../controllers/api_work_in_progress.js"
import { New } from "../controllers/new.js"
import { Templates } from "../controllers/templates.js"

const routes = [
	{
		path: '/syntax',
		component: SyntaxDoc
	},
	{
		path: '/engine',
		component: Render
	},
	{
		path: '/api-docs',
		component: APIWorkInProgress
	},
	{
		path: '/new',
		component: New
	},
	{
		path: '/templates',
		component: Templates
	},
]

const router = new VueRouter({
	mode: 'history',
	routes
})

export { router }
