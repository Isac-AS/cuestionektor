
import { createRouter, createWebHistory } from 'vue-router';
import Home from '../views/Home.vue'

const routes = [
	{
		path: '/',
		name: 'Inicio',
		component: Home
	},
	{
		path: '/create',
		name: 'Crear cuestionario',
		component: () => import('../views/Create.vue')
	},
	{
		path: '/create-from-file',
		name: 'Subir fichero',
		component: () => import('../views/UploadFile.vue')
	},
	{
		path: '/manage',
		name: 'Gestionar cuestionarios',
		component: () => import('../views/Manage.vue')
	},
	{
		path: '/questionnaire',
		name: 'Cuestionario',
		component: () => import('../views/Questionnaire.vue'),
	},
]

const router = createRouter({
	history: createWebHistory(),
	routes
 })
 
 export default router