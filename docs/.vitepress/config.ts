import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "Franz BC Dev Helper",
  description: "Documentation for Franz BC Dev Helper",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: 'Home', link: '/' },
      // { text: 'Examples', link: '/markdown-examples' }
    ],

    sidebar: [
      {
        //text: 'Examples',
        //items: [
        //  { text: 'Markdown Examples', link: '/markdown-examples' },
        //  { text: 'Runtime API Examples', link: '/api-examples' }
        //]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/Franz-Mueller/FranzBCDevHelper' }
    ]
  }
})
