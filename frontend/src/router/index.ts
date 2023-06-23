import {createRouter, createWebHistory} from 'vue-router'
import type {NavigationGuardNext, RouteLocationNormalized} from 'vue-router'
import {useStore} from "@/stores/reservation";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      redirect: '/reservation/calendar'
    },
    {
      path: '/login',
      name: 'login',
      component: () => import('@/views/Login.vue')
    },
    {
      path: '/reservation/calendar',
      name: 'reservation_calendar',
      // route level code-splitting
      // this generates a separate chunk (About.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: () => import('@/views/ReservationCalendar.vue')
    },
    {
      path: '/reservation/checklist',
      name: 'preflight_checklist',
      // route level code-splitting
      // this generates a separate chunk (About.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: () => import('@/views/PreflightChecklist.vue')
    },
    {
      path: '/settings',
      name: 'settings',
      // route level code-splitting
      // this generates a separate chunk (About.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: () => import('@/views/Settings.vue')
    }
  ]
})
router.beforeEach((to: RouteLocationNormalized, from: RouteLocationNormalized, next: NavigationGuardNext) => {
  const store = useStore();

  // Check if user is authenticated (You might want to replace this with actual token validation logic)
  if (!store.token) {
    // If user is not authenticated and trying to access other pages, redirect to Login
    if (to.path !== '/login') {
      next({ path: '/login' });
    } else {
      next();
    }
  } else {
    // If user is authenticated, just proceed
    next();
  }
});
export default router
