import { themes as prismThemes } from "prism-react-renderer";
import type { Config } from "@docusaurus/types";
import type * as Preset from "@docusaurus/preset-classic";
import tailwindPlugin from "./plugins/tailwind-config.cjs";

// This runs in Node.js - Don't use client-side code here (browser APIs, JSX...)

require("dotenv").config();

const inkeepApiKey = process.env.INKEEP_API_KEY;
const inkeepIntegrationId = process.env.INKEEP_INTEGRATION_ID;
const inkeepOrgId = process.env.INKEEP_ORG_ID;

const config: Config = {
  title: "goose",
  tagline:
    "your local AI agent, automating engineering tasks seamlessly",
  favicon: "img/favicon.ico",

  // Set the production url of your site here
  url: "https://block.github.io/",
  // Set the /<baseUrl>/ pathname under which your site is served
  // For GitHub pages deployment, it is often '/<projectName>/'
  baseUrl: process.env.TARGET_PATH || "/goose/",

  // GitHub pages deployment config.
  // If you aren't using GitHub pages, you don't need these.
  organizationName: "block", // Usually your GitHub org/user name.
  projectName: "goose", // Usually your repo name.

  onBrokenLinks: "throw",
  onBrokenMarkdownLinks: "warn",

  // Even if you don't use internationalization, you can use this field to set
  // useful metadata like html lang. For example, if your site is Chinese, you
  // may want to replace "en" with "zh-Hans".
  i18n: {
    defaultLocale: "en",
    locales: ["en"],
  },

  presets: [
    [
      "classic",
      {
        docs: {
          sidebarPath: "./sidebars.ts",
        },
        blog: {
          showReadingTime: true,
          feedOptions: {
            type: ["rss", "atom"],
            xslt: true,
          },
          // Useful options to enforce blogging best practices
          onInlineTags: "warn",
          onInlineAuthors: "warn",
          onUntruncatedBlogPosts: "warn",
          blogSidebarCount: 'ALL'
        },
        theme: {
          customCss: [
            "./src/css/custom.css",
            "./src/css/extensions.css",
            "./src/css/tailwind.css",
          ],
        },
        gtag: process.env.NODE_ENV === 'production' ? {
          trackingID: 'G-ZS5D6SB4ZJ',
          anonymizeIP: true,
        } : undefined,
      } satisfies Preset.Options,
    ],
  ],
  plugins: [
    require.resolve("./plugins/custom-webpack.cjs"),
    [
      "@docusaurus/plugin-client-redirects",
      {
        redirects: [
          {
            from: '/docs/getting-started/using-goose-free',
            to: '/docs/getting-started/providers#using-goose-for-free'
          },
          {
            from: '/v1/docs/getting-started/providers',
            to: '/docs/getting-started/providers'
          },
          {
            from: '/v1/docs/getting-started/installation',
            to: '/docs/getting-started/installation'
          },
          {
            from: '/v1/docs/quickstart',
            to: '/docs/quickstart'
          },
          {
            from: '/v1/',
            to: '/'
          },
          {
            from: '/docs/guides/custom-extensions',
            to: '/docs/tutorials/custom-extensions'
          },
          {
            from: '/docs',
            to: '/docs/category/getting-started'
          },
          {
            from: '/v1/extensions',
            to: '/extensions'
          },
          {
            from: '/v1/extensions/detail/nondeveloper',
            to: '/docs/mcp/computer-controller-mcp'
          },
          {
            from: '/docs/guides/managing-goose-sessions',
            to: '/docs/guides/sessions/session-management'
          },
          {
            from: '/docs/guides/smart-context-management',
            to: '/docs/guides/sessions/smart-context-management'
          },
          {
            from: '/docs/guides/share-goose-sessions',
            to: '/docs/guides/recipes/session-recipes'
          },
          {
            from: '/docs/guides/session-recipes',
            to: '/docs/guides/recipes/session-recipes'
          },
          {
            from: '/docs/guides/recipe-reference',
            to: '/docs/guides/recipes/recipe-reference'
          },
          {
            from: '/docs/guides/recipes/sub-recipes',
            to: '/docs/guides/recipes/subrecipes'
          },
          {
            from: '/docs/tutorials/sub-recipes-in-parallel',
            to: '/docs/tutorials/subrecipes-in-parallel'
          },
          {
            from: '/docs/guides/tool-permissions',
            to: '/docs/guides/managing-tools/tool-permissions'
          },
          {
            from: '/docs/guides/adjust-tool-output',
            to: '/docs/guides/managing-tools/adjust-tool-output'
          },
          {
            from: '/docs/guides/benchmarking',
            to: '/docs/tutorials/benchmarking'
          },
          {
            from: '/docs/guides/goose-in-docker',
            to: '/docs/tutorials/goose-in-docker'
          },
          {
            from: '/docs/guides/creating-plans',
            to: '/docs/guides/multi-model/creating-plans'
          },
          // MCP tutorial redirects - moved from /docs/tutorials/ to /docs/mcp/
          {
            from: '/docs/tutorials/agentql-mcp',
            to: '/docs/mcp/agentql-mcp'
          },
          {
            from: '/docs/tutorials/asana-mcp',
            to: '/docs/mcp/asana-mcp'
          },
          {
            from: '/docs/tutorials/blender-mcp',
            to: '/docs/mcp/blender-mcp'
          },
          {
            from: '/docs/tutorials/brave-mcp',
            to: '/docs/mcp/brave-mcp'
          },
          {
            from: '/docs/tutorials/browserbase-mcp',
            to: '/docs/mcp/browserbase-mcp'
          },
          {
            from: '/docs/tutorials/computer-controller-mcp',
            to: '/docs/mcp/computer-controller-mcp'
          },
          {
            from: '/docs/tutorials/context7-mcp',
            to: '/docs/mcp/context7-mcp'
          },
          {
            from: '/docs/tutorials/developer-mcp',
            to: '/docs/mcp/developer-mcp'
          },
          {
            from: '/docs/tutorials/elevenlabs-mcp',
            to: '/docs/mcp/elevenlabs-mcp'
          },
          {
            from: '/docs/tutorials/fetch-mcp',
            to: '/docs/mcp/fetch-mcp'
          },
          {
            from: '/docs/tutorials/figma-mcp',
            to: '/docs/mcp/figma-mcp'
          },
          {
            from: '/docs/tutorials/filesystem-mcp',
            to: '/docs/mcp/filesystem-mcp'
          },
          {
            from: '/docs/tutorials/github-mcp',
            to: '/docs/mcp/github-mcp'
          },
          {
            from: '/docs/tutorials/google-drive-mcp',
            to: '/docs/mcp/google-drive-mcp'
          },
          {
            from: '/docs/tutorials/google-maps-mcp',
            to: '/docs/mcp/google-maps-mcp'
          },
          {
            from: '/docs/tutorials/jetbrains-mcp',
            to: '/docs/mcp/jetbrains-mcp'
          },
          {
            from: '/docs/tutorials/knowledge-graph-mcp',
            to: '/docs/mcp/knowledge-graph-mcp'
          },
          {
            from: '/docs/tutorials/mbot-mcp',
            to: '/docs/mcp/mbot-mcp'
          },
          {
            from: '/docs/tutorials/memory-mcp',
            to: '/docs/mcp/memory-mcp'
          },
          {
            from: '/docs/tutorials/nostrbook-mcp',
            to: '/docs/mcp/nostrbook-mcp'
          },
          {
            from: '/docs/tutorials/pdf-mcp',
            to: '/docs/mcp/pdf-mcp'
          },
          {
            from: '/docs/tutorials/pieces-mcp',
            to: '/docs/mcp/pieces-mcp'
          },
          {
            from: '/docs/tutorials/playwright-mcp',
            to: '/docs/mcp/playwright-mcp'
          },
          {
            from: '/docs/tutorials/postgres-mcp',
            to: '/docs/mcp/postgres-mcp'
          },
          {
            from: '/docs/tutorials/puppeteer-mcp',
            to: '/docs/mcp/puppeteer-mcp'
          },
          {
            from: '/docs/tutorials/reddit-mcp',
            to: '/docs/mcp/reddit-mcp'
          },
          {
            from: '/docs/tutorials/repomix-mcp',
            to: '/docs/mcp/repomix-mcp'
          },
          {
            from: '/docs/tutorials/selenium-mcp',
            to: '/docs/mcp/selenium-mcp'
          },
          {
            from: '/docs/tutorials/speech-mcp',
            to: '/docs/mcp/speech-mcp'
          },
          {
            from: '/docs/tutorials/square-mcp',
            to: '/docs/mcp/square-mcp'
          },
          {
            from: '/docs/tutorials/tavily-mcp',
            to: '/docs/mcp/tavily-mcp'
          },
          {
            from: '/docs/tutorials/tutorial-extension',
            to: '/docs/mcp/tutorial-mcp'
          },
          {
            from: '/docs/tutorials/vscode-mcp',
            to: '/docs/mcp/vs-code-mcp'
          },
          {
            from: '/docs/tutorials/youtube-transcript',
            to: '/docs/mcp/youtube-transcript-mcp'
          },
          {
            from: '/docs/guides/isolated-development-environments',
            to: '/docs/tutorials/isolated-development-environments'
          },
          {
            from: '/docs/experimental/subagents',
            to: '/docs/guides/subagents'
          }         
        ],
      },
    ],
    tailwindPlugin,
  ],
  themes: ["@inkeep/docusaurus/chatButton", "@inkeep/docusaurus/searchBar"],
  themeConfig: {
    // Replace with your project's social card
    image: "img/home-banner.png",
    colorMode: {
      respectPrefersColorScheme: true
    },
    navbar: {
      title: "",
      logo: {
        alt: "Block Logo",
        src: "img/logo_light.png",
        srcDark: "img/logo_dark.png",
      },
      items: [
        {
          to: "/docs/quickstart",
          label: "Quickstart",
          position: "left",
        },
        {
          to: "/docs/category/guides",
          position: "left",
          label: "Docs",
        },
        {
          to: "/docs/category/tutorials",
          position: "left",
          label: "Tutorials",
        },
        {
          to: "/docs/category/mcp-servers",
          position: "left",
          label: "MCPs",
        },
        { to: "/blog", label: "Blog", position: "left" },
        {
          type: 'dropdown',
          label: 'Resources',
          position: 'left',
          items: [
            {
              to: '/extensions',
              label: 'Extensions',
            },
            {
              to: '/recipe-generator',
              label: 'Recipe Generator',
            },
            {
              to: '/prompt-library',
              label: 'Prompt Library',
            },
            {
              to: '/recipes',
              label: 'Recipe Cookbook',
            },
            {
              to: 'deeplink-generator',
              label: 'Deeplink Generator',
            },
          ],
        },

        {
          href: "https://discord.gg/block-opensource",
          label: "Discord",
          position: "right",
        },
        {
          href: "https://github.com/block/goose",
          label: "GitHub",
          position: "right",
        },
      ],
    },
    footer: {
      links: [
        {
          title: "Quick Links",
          items: [
            {
              label: "Install goose",
              to: "docs/getting-started/installation",
            },
            {
              label: "Extensions",
              to: "/extensions",
            },
          ],
        },
        {
          title: "Community",
          items: [
            {
              label: "Spotlight",
              to: "community",
            },
            {
              label: "Discord",
              href: "https://discord.gg/block-opensource",
            },
            {
              label: "YouTube",
              href: "https://www.youtube.com/@goose-oss",
            },
            {
              label: "LinkedIn",
              href: "https://www.linkedin.com/company/goose-oss",
            },
            {
              label: "Twitter / X",
              href: "https://x.com/goose_oss",
            },
            {
              label: "BlueSky",
              href: "https://bsky.app/profile/opensource.block.xyz",
            },
            {
              label: "Nostr",
              href: "https://njump.me/opensource@block.xyz",
            },
          ],
        },
        {
          title: "More",
          items: [
            {
              label: "Blog",
              to: "/blog",
            },
            {
              label: "GitHub",
              href: "https://github.com/block/goose",
            },
          ],
        },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} Block, Inc.`,
    },
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.nightOwl,
    },
    inkeepConfig: {
      baseSettings: {
        apiKey: inkeepApiKey,
        integrationId: inkeepIntegrationId,
        organizationId: inkeepOrgId,
        primaryBrandColor: "#1E1E1E",
      },
      aiChatSettings: {
        chatSubjectName: "goose",
        botAvatarSrcUrl:
          "",
        getHelpCallToActions: [
          {
            name: "GitHub",
            url: "https://github.com/block/goose",
            icon: {
              builtIn: "FaGithub",
            },
          },
        ],
        quickQuestions: ["What is goose?"],
      },
    },
    announcementBar: {
      id: 'goose-grants',
      content:
        '✨ goose grant program now open: <a href="/goose/grants">apply now</a>! ✨',
      backgroundColor: '#20232a',
      textColor: '#fff',
      isCloseable: false,
    },
  } satisfies Preset.ThemeConfig,
};


export default config;
