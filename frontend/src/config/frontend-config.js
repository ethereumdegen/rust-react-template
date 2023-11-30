import favicon from "@/assets/images/vector_waves_logo.png";
import homeImage from "@/assets/images/vector_waves_2.png";

const config = {
  title: "Tempo Virtual Assistant",
  tagline: "Performing traditional enterprise infrastructure tasks with AI chat",
  url: "https://tempova.com",
  baseURL: "/",
  favicon: favicon,
  homeImage: homeImage,

  navbar: {
    title: "",
    logo: {
      alt: "Pepe Logo",
      src: "assets/images/pepe_favicon.png",
    },
    items: [
      { to: "/", label: "Home" },
      { to: "/chat", label: "Chat" } 
    ],
  },

  accountMenu: {
    items: [
      {
        to: "/",
        label: "Home",
      } 
    ],
  },

  footer: {
    style: "light",
    columns: [
      {
        title: "Application",
        items: [
          {
            label: "Chat",
            to: "/chat/",
          },

          
        ],
      },
      {
        title: "Community",
        items: [],
      },
      {
        title: "More",
        items: [
          {
            label: "GitHub",
            href: "https://github.com/degentx",
          },
        ],
      },
    ],
    copyright: `Copyright © ${new Date().getFullYear()} `,

    socials: {
      twitter: "https://twitter.com/",
      github: "https://github.com/",
    },
  },
};

export default config;
//module.exports = config;
