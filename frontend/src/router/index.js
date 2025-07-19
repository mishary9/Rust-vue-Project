
import { createRouter, createWebHistory } from 'vue-router'
import RegistrationComponent from '@/components/RegistrationComponent.vue'
import HomePage from '@/components/HomePage.vue'

const routes = [
  {
    path: '/',
    name: 'Registration',
    component: RegistrationComponent
  },
  {
    path: '/home',
    name: 'Home',
    component: HomePage
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router
