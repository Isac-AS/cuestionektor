
import { createRouter, createWebHistory } from 'vue-router';
import Home from '../views/Home.vue'

const routes = [
	{
		path: '/',
		component: Home
	},
	{
		path: '/create',
		component: () => import('../views/Create.vue')
	},
	{
		path: '/create-from-file',
		component: () => import('../views/UploadFile.vue')
	},
	{
		path: '/manage',
		component: () => import('../views/Manage.vue')
	},
	{
		path: '/questionnaire',
		component: () => import('../views/Questionnaire.vue')
	},
]

const router = createRouter({
	history: createWebHistory(),
	routes
 })
 
 export default router