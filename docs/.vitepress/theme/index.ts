import DefaultTheme from 'vitepress/theme'
import { onMounted } from 'vue'
import { useRouter } from 'vitepress'

export default {
  extends: DefaultTheme,
  setup() {
    const router = useRouter()

    onMounted(() => {
      // 只在首页检测语言
      if (window.location.pathname === '/' || window.location.pathname === '/index.html') {
        // 检查是否已经重定向过（避免循环）
        const redirected = sessionStorage.getItem('lang-redirected')
        if (redirected) return

        // 检测浏览器语言
        const browserLang = navigator.language || (navigator as any).userLanguage
        const isZh = browserLang.toLowerCase().startsWith('zh')

        if (isZh) {
          sessionStorage.setItem('lang-redirected', 'true')
          router.go('/zh/')
        }
      }
    })
  },
}
