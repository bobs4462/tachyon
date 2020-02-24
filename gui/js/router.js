import { Render } from "../controllers/render.js"
import { SyntaxDoc } from "../controllers/syntaxdoc.js"
import { APIWorkInProgress } from "../controllers/api_work_in_progress.js"
import { NewWorkInProgress } from "../controllers/new_work_in_progress.js"
import { TemplatesWorkInProgress } from "../controllers/templates_work_in_progress.js"

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
		component: NewWorkInProgress
	},
	{
		path: '/templates',
		component: TemplatesWorkInProgress
	},
]

const router = new VueRouter({
	mode: 'history',
	routes
})

export { router }
