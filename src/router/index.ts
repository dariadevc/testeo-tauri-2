import { createRouter, createWebHistory } from 'vue-router'
import Home from '../views/home.vue'
import CalcularAnioNacimiento from '../views/calcular-anio-nacimiento.vue'
import CalcularEdad from '../views/calcular-edad.vue'

const routes = [
  { path: '/', name: 'Inicio', component: Home },
  { path: '/anio-nacimiento', name: 'anio-nacimiento', component: CalcularAnioNacimiento },
  { path: '/edad', name: 'edad', component: CalcularEdad }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router