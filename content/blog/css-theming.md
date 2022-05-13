+++
title = "How to add a dark theme to your website using basic CSS and Javascript"
date = 2022-05-09
+++

If you are making a simple website such as this one and you think it could use a
dark theme, that can be done very easily with some basic HTML, CSS, and
Javascript.

## First, the CSS

The first step is to define your themes in CSS. The way I will demonstrate only
uses one stylesheet.

The first step is to make a `:root` section in your stylesheet and then define
all the colours you are going to use in it.

This is what mine looks like:

```css
:root {
  --background-color: #0e1019;
  --text-color: #cccccc;
  --heading-color: #ffffff;
  --header-color: #ff000f;
  --hover-color: #f5515b;
}
```

I want my default theme to be dark so I have used my dark theme colours. You can
name these variables whatever you feel makes most sense.

Now you need to add another CSS section for your alternate theme, in my case
this is the light theme:

```css
[data-theme="light"] {
  --background-color: #ffffff;
  --text-color: #000000;
  --heading-color: #000000;
  --header-color: #2c3e50;
  --hover-color: #4595a8;
}
```

If this were your dark theme, it would make more sense to write
data-theme="dark" but it doesn't actually matter because you are going to write
Javascript code that reads the names later.

*That's the CSS done, time for some Javascript!*

## The Javascript

The Javascript for this will be pretty simple. I am going to use browser
localstorage to save the state. This way, when the user refreshes the page the
theme won't revert back to the default.

The entire script is this:

```javascript
function toggleTheme() {
  if (localStorage.getItem("theme") === "light") {
    localStorage.setItem("theme", "dark");
  } else {
    localStorage.setItem("theme", "light");
  }

  initTheme();
}

function initTheme() {
  if (localStorage.getItem("theme") === "light") {
    document.documentElement.setAttribute("data-theme", "light");
  } else {
    document.documentElement.setAttribute("data-theme", "dark");
  }
}

initTheme();
```

I chose to put this as the first thing in the body of my HTML file. It's so
small that I didn't see the need for a standalone Javascript file.

### Let's analyze these functions

It makes most sense to look at `initTheme` first. Here it is again:

```javascript
function initTheme() {
  if (localStorage.getItem("theme") === "light") {
    document.documentElement.setAttribute("data-theme", "light");
  } else {
    document.documentElement.setAttribute("data-theme", "dark");
  }
}
```

This is used to read the theme from localstorage and then set it. An important
thing to note here is that if "theme" is not yet set in localstorage, it will
default to dark theme. You can change this behaviour if you want light theme to
be the default.

It is also important that the value you set "data-theme" to must match the name
you used in your stylesheet. You may have noticed that I never actually titled
anything "dark" in the CSS, and you would be right. I could make that say
anything and it would still work because dark is the default theme.
