<script lang="ts">
  let { children } = $props();
</script>

<header>
  <h1>Cereal Co</h1>
</header>
{@render children()}

<style>
  h1 {
    text-align: center;
  }

  header {
    width: 100vw;
    box-sizing: border-box;
    display: flex;
    justify-content: center;
    align-items: center;

    background-color: var(--secondary-color);

    box-shadow: 0px 1px 0px 0px hsl(0, 0%, 20%);
  }

  :root {
    --primary-color: hsl(0, 0%, 15%);
    --secondary-color: hsl(0, 0%, 20%);
    --tertiary-color: hsl(0, 0%, 55%);
    --quaternary-color: hsl(231, 48%, 36%);

    background-color: var(--primary-color);
    color: var(--tertiary-color);

    font-family: "Roboto Flex Variable", sans-serif;
  }

  /* roboto-flex-latin-wght-normal */
  @font-face {
    font-family: "Roboto Flex Variable";
    font-style: oblique 0deg 10deg;
    font-display: swap;
    font-weight: 100 1000;
    font-stretch: 25% 151%;
    src: url(@fontsource-variable/roboto-flex/files/roboto-flex-latin-full-normal.woff2)
      format("woff2-variations");
    unicode-range: U+0000-00FF, U+0131, U+0152-0153, U+02BB-02BC, U+02C6, U+02DA,
      U+02DC, U+0304, U+0308, U+0329, U+2000-206F, U+2074, U+20AC, U+2122,
      U+2191, U+2193, U+2212, U+2215, U+FEFF, U+FFFD;
  }

  /* roboto-flex-latin-ext-wght-normal */
  @font-face {
    font-family: "Roboto Flex Variable";
    font-style: oblique 0deg 10deg;
    font-display: swap;
    font-weight: 100 1000;
    font-stretch: 25% 151%;
    src: url(@fontsource-variable/roboto-flex/files/roboto-flex-latin-ext-full-normal.woff2)
      format("woff2-variations");
    unicode-range: U+0100-02AF, U+0304, U+0308, U+0329, U+1E00-1E9F, U+1EF2-1EFF,
      U+2020, U+20A0-20AB, U+20AD-20C0, U+2113, U+2C60-2C7F, U+A720-A7FF;
  }
</style>
