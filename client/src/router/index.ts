// src/router/index.ts
import { createRouter, createWebHistory } from 'vue-router'
import Home from '../views/Home.vue'
import Plans from '../views/Plans.vue'
import Test from '../views/Test.vue'
import Confirm from '../views/Confirm.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'home',
      component: Home
    },
    {
      path: '/plans',
      name: 'plans',
      component: Plans
    },
    {
      path: '/test',
      name: 'test',
      component: Test
    },
    {
      path: '/confirm',
      name: 'confirm',
      component: Confirm
    }
  ]
})

export default router