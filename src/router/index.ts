
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
]

const router = createRouter({
	history: createWebHistory(),
	routes
 })
 
 export default router