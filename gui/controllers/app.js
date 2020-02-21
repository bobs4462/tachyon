import { router } from '../js/router.js'

const App = `
<el-container style="height: 100%">
	<el-header>
		<div>Добро пожаловать в Tachyon</div>
		<div><span style="font-size: 0.5em">(Быстрая система веб-шаблонов)</span></div>
	</el-header>
	<el-aside style="height: 100%">
		<el-menu background-color="#545c64"
						 text-color="#fff"
						 :router="true"
						 active-text-color="#ffd04b">
			<el-menu-item>Отрисовка</el-menu-item>
			<el-menu-item>Шаблоны</el-menu-item>
			<el-menu-item>Новый шаблон</el-menu-item>
			<el-menu-item>Синтаксис</el-menu-item>
			<el-menu-item index="/about">API</el-menu-item>
		</el-menu>
	</el-aside>
	<el-main>
			<router-view></router-view>
			</el-main>
</el-container>
	`

var app = new Vue({
	el: '#app',
	template: App,
	data: function() {
		return { 
			lol: 1
		}
	},
	methods: {
		save(account) {
			axios.post("/update", account).then(
				resp => {
					if (resp.data.code === 0) {
						this.$message(
							{
								message: "Аккаунт успешно обновлён",
								type: "success"
							}
						);	
					}
				}).catch((e) => { console.log(e) }).finally(() => { this.edit_account = false })
		}
	},
	router,
})

