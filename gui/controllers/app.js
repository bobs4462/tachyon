import { router } from '../js/router.js'

const App = `
<el-container style="height: 100%" direction="horizontal">
	<el-aside style="height: 100%; background-color: #545c64; overflow: hidden" width="12%">
					<el-image fit="fit" src="images/default.png"></el-image>
		<el-menu background-color="#545c64"
						 text-color="#fff"
						 :router="true"
						 active-text-color="#ffd04b">
			<el-menu-item>
				<template slot="title">
					<i class="el-icon-picture"></i>
					<span slot="title">Отрисовка</span>
				</template></span></i></template>
			</el-menu-item>
			<el-menu-item>
				<i class="el-icon-document"></i>
				<span slot="title">Шаблоны</span>
				</template></span></i></template>
			</el-menu-item>
			<el-menu-item>
				<i class="el-icon-document-add"></i>
				<span slot="title">Новый шаблон</span>
				</template></span></i></template>
			</el-menu-item>
			<el-menu-item>
				<i class="el-icon-edit-outline"></i>
				<span slot="title">Синтаксис</span>
				</template></span></i></template>
			</el-menu-item>
			<el-menu-item index="/about">
				<i class="el-icon-setting"></i>
				<span slot="title">API</span>
				</template></span></i></template>
			</el-menu-item>
		</el-menu>
	</el-aside>
	<el-container>
		<el-header>
			<el-card shadow="always">
						<div>Добро пожаловать в Tachyon</div>
						<div><span style="font-size: 0.5em">(Быстрая система веб-шаблонов)</span></div>
				</div>

			</el-card>
		</el-header>
		<el-main>
			<router-view></router-view>
		</el-main>
	</el-container>
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

