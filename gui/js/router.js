const routes = [
	{
		path: '/about',
		component: ApiDoc
	}
]

const router = new VueRouter({
	mode: 'history',
	routes
})

export { router }
