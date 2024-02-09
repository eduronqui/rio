"use strict";(self.webpackChunkrio_docs=self.webpackChunkrio_docs||[]).push([[5769],{4921:(e,i,t)=>{t.r(i),t.d(i,{assets:()=>l,contentTitle:()=>o,default:()=>h,frontMatter:()=>r,metadata:()=>a,toc:()=>d});var s=t(5893),n=t(1151);const r={layout:"post",title:"Release 0.0.5",date:"2023-05-31 10:34:14 +0200",categories:"macos linux release",description:"Arrival to Linux, themes support and many bug fixes."},o="Arrival to Linux, Themes support and many bug fixes.",a={permalink:"/rio/pt-br/blog/2023/05/31/release-0.0.5",editUrl:"https://github.com/raphamorim/rio/tree/main/docs/blog/2023-05-31-release-0.0.5.md",source:"@site/blog/2023-05-31-release-0.0.5.md",title:"Release 0.0.5",description:"Arrival to Linux, themes support and many bug fixes.",date:"2023-05-31T10:34:14.000Z",formattedDate:"31 de maio de 2023",tags:[],readingTime:1.235,hasTruncateMarker:!1,authors:[],frontMatter:{layout:"post",title:"Release 0.0.5",date:"2023-05-31 10:34:14 +0200",categories:"macos linux release",description:"Arrival to Linux, themes support and many bug fixes."},unlisted:!1,prevItem:{title:"Release 0.0.6",permalink:"/rio/pt-br/blog/2023/06/07/release-0.0.6"}},l={authorsImageUrls:[]},d=[{value:"New configuration path",id:"new-configuration-path",level:2},{value:"Linux Support",id:"linux-support",level:2},{value:"Themes Support",id:"themes-support",level:2},{value:"Bold and italic style",id:"bold-and-italic-style",level:2},{value:"Changelog",id:"changelog",level:2}];function c(e){const i={a:"a",code:"code",h2:"h2",img:"img",li:"li",p:"p",ul:"ul",...(0,n.a)(),...e.components};return(0,s.jsxs)(s.Fragment,{children:[(0,s.jsx)(i.p,{children:"Rio release 0.0.5 is finally here, there's a lot of updates to cover so let's get started. I also would like to remind you that Rio is still unstable. This release adds a lot of bug fixes and feature additions in order to make Rio terminal stable."}),"\n",(0,s.jsx)(i.h2,{id:"new-configuration-path",children:"New configuration path"}),"\n",(0,s.jsxs)(i.p,{children:["Configuration path has changed from ",(0,s.jsx)(i.code,{children:"{$HOME}/.rio/"})," to ",(0,s.jsx)(i.code,{children:"{$HOME}/.config/rio"}),"."]}),"\n",(0,s.jsx)(i.p,{children:"Changes were applied for macOS and Linux."}),"\n",(0,s.jsx)(i.h2,{id:"linux-support",children:"Linux Support"}),"\n",(0,s.jsxs)(i.p,{children:["Rio is now available to Linux, build information was added in the ",(0,s.jsx)(i.a,{href:"/docs/install",children:"install"})," page."]}),"\n",(0,s.jsx)(i.p,{children:(0,s.jsx)(i.img,{alt:"Linux support",src:t(5714).Z+"",width:"2560",height:"1666"})}),"\n",(0,s.jsx)(i.p,{children:"Linux builds are considered less stable than macOs, due to macOs features availability comparison."}),"\n",(0,s.jsx)(i.h2,{id:"themes-support",children:"Themes Support"}),"\n",(0,s.jsx)(i.p,{children:'A new property was added to Rio configuration file called "theme". You can set the theme that you want to use and Rio will look in the folder "themes" in the configuration path.'}),"\n",(0,s.jsx)(i.p,{children:(0,s.jsx)(i.img,{alt:"Themes support",src:t(9204).Z+"",width:"2584",height:"1604"})}),"\n",(0,s.jsx)(i.p,{children:"Dracula theme example:"}),"\n",(0,s.jsx)(i.p,{children:(0,s.jsx)(i.img,{alt:"Example dracula",src:t(2576).Z+"",width:"1824",height:"1424"})}),"\n",(0,s.jsx)(i.h2,{id:"bold-and-italic-style",children:"Bold and italic style"}),"\n",(0,s.jsx)(i.p,{children:'Support to text styling as such "bold" and "italic".'}),"\n",(0,s.jsx)(i.p,{children:(0,s.jsx)(i.img,{alt:"Themes support",src:t(1073).Z+"",width:"1572",height:"658"})}),"\n",(0,s.jsx)(i.h2,{id:"changelog",children:"Changelog"}),"\n",(0,s.jsxs)(i.ul,{children:["\n",(0,s.jsxs)(i.li,{children:["Fix to render specific 24bit colors ",(0,s.jsx)(i.a,{href:"https://github.com/raphamorim/rio/issues/#66",children:"#66"})," by ",(0,s.jsx)(i.a,{href:"https://github.com/niuez",children:"@niuez"}),"."]}),"\n",(0,s.jsx)(i.li,{children:"Cross build for arm64 and x86"}),"\n",(0,s.jsxs)(i.li,{children:["Bold and Italic support ",(0,s.jsx)(i.a,{href:"https://github.com/raphamorim/rio/issues/#33",children:"#33"}),"."]}),"\n",(0,s.jsx)(i.li,{children:"Add RioEvent::ColorRequest events to write color updates on pty."}),"\n",(0,s.jsxs)(i.li,{children:["Theme support ",(0,s.jsx)(i.a,{href:"https://github.com/raphamorim/rio/issues/42",children:"#42"}),"."]}),"\n",(0,s.jsx)(i.li,{children:"Fix font-size dependency for serialization"}),"\n",(0,s.jsxs)(i.li,{children:["Fix cursor visibility on VI mode and scroll ",(0,s.jsx)(i.a,{href:"https://github.com/raphamorim/#51",children:"#51"})]}),"\n",(0,s.jsx)(i.li,{children:"Performance fixes for rendering from teletypewriter updates."}),"\n",(0,s.jsxs)(i.li,{children:["Fix scale issues for 1.0 scale factor or using monitor with different scale factor. ",(0,s.jsx)(i.a,{href:"https://github.com/raphamorim/rio/issues/#50",children:"#50"})]}),"\n",(0,s.jsxs)(i.li,{children:["Improved release process to only contain Rio.app file. ",(0,s.jsx)(i.a,{href:"https://github.com/raphamorim/rio/issues/#54",children:"#54"})]}),"\n"]})]})}function h(e={}){const{wrapper:i}={...(0,n.a)(),...e.components};return i?(0,s.jsx)(i,{...e,children:(0,s.jsx)(c,{...e})}):c(e)}},2576:(e,i,t)=>{t.d(i,{Z:()=>s});const s=t.p+"assets/images/dracula-nvim-a1d1457e2aa3780989b7e3378e473316.png"},1073:(e,i,t)=>{t.d(i,{Z:()=>s});const s=t.p+"assets/images/font-macos-c4784c228d88df845c1799fd07542625.png"},5714:(e,i,t)=>{t.d(i,{Z:()=>s});const s=t.p+"assets/images/linux-edbdfd8e494118000e7984f4edd140fd.jpeg"},9204:(e,i,t)=>{t.d(i,{Z:()=>s});const s=t.p+"assets/images/themes-3d4190a60babb8dd15249ebd3ff09433.png"},1151:(e,i,t)=>{t.d(i,{Z:()=>a,a:()=>o});var s=t(7294);const n={},r=s.createContext(n);function o(e){const i=s.useContext(r);return s.useMemo((function(){return"function"==typeof e?e(i):{...i,...e}}),[i,e])}function a(e){let i;return i=e.disableParentContext?"function"==typeof e.components?e.components(n):e.components||n:o(e.components),s.createElement(r.Provider,{value:i},e.children)}}}]);