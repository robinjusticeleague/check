<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/checkjs/resources/main/svg/slogan-dark-transparent.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/checkjs/resources/main/svg/slogan-light-transparent.svg">
    <img alt="Shows the banner of Check, with its logo and the phrase 'Check - Toolchain of the web'." src="https://raw.githubusercontent.com/checkjs/resources/main/svg/slogan-light-transparent.svg" width="700">
  </picture>

  <br>
  <br>

  [![CI on main][ci-badge]][ci-url]
  [![Discord chat][discord-badge]][discord-url]
  [![npm version][npm-badge]][npm-url]
  [![VSCode version][vscode-badge]][vscode-url]
  [![Open VSX version][open-vsx-badge]][open-vsx-url]

  [ci-badge]: https://github.com/checkjs/check/actions/workflows/main.yml/badge.svg
  [ci-url]: https://github.com/checkjs/check/actions/workflows/main.yml
  [discord-badge]: https://badgen.net/discord/online-members/BypW39g6Yc?icon=discord&label=discord&color=60a5fa
  [discord-url]: https://checkjs.dev/chat
  [npm-badge]: https://badgen.net/npm/v/@checkjs/check?icon=npm&color=60a5fa&label=%40checkjs%2Fcheck
  [npm-url]: https://www.npmjs.com/package/@checkjs/check/v/latest
  [vscode-badge]: https://img.shields.io/visual-studio-marketplace/v/checkjs.check?label=Visual%20Studio%20Marketplace&labelColor=374151&color=60a5fa
  [vscode-url]: https://marketplace.visualstudio.com/items?itemName=checkjs.check
  [open-vsx-badge]: https://img.shields.io/visual-studio-marketplace/v/checkjs.check?label=Open%20VSX%20Registry&logo=data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0idXRmLTgiPz4KPHN2ZyB2aWV3Qm94PSI0LjYgNSA5Ni4yIDEyMi43IiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPgogIDxwYXRoIGQ9Ik0zMCA0NC4yTDUyLjYgNUg3LjN6TTQuNiA4OC41aDQ1LjNMMjcuMiA0OS40em01MSAwbDIyLjYgMzkuMiAyMi42LTM5LjJ6IiBmaWxsPSIjYzE2MGVmIi8+CiAgPHBhdGggZD0iTTUyLjYgNUwzMCA0NC4yaDQ1LjJ6TTI3LjIgNDkuNGwyMi43IDM5LjEgMjIuNi0zOS4xem01MSAwTDU1LjYgODguNWg0NS4yeiIgZmlsbD0iI2E2MGVlNSIvPgo8L3N2Zz4=&labelColor=374151&color=60a5fa
  [open-vsx-url]: https://open-vsx.org/extension/checkjs/check

<!-- Insert new entries lexicographically by language code.
     For example given below is the same order as these files appear on page:
     https://github.com/checkjs/check/tree/main/packages/@checkjs/check -->

  [हिन्दी](https://github.com/checkjs/check/blob/main/packages/%40checkjs/check/README.hi.md) | English | [Español](https://github.com/checkjs/check/blob/main/packages/%40checkjs/check/README.es.md) | [Français](https://github.com/checkjs/check/blob/main/packages/%40checkjs/check/README.fr.md) | [繁體中文](https://github.com/checkjs/check/blob/main/packages/%40checkjs/check/README.zh-TW.md) | [简体中文](https://github.com/checkjs/check/blob/main/packages/%40checkjs/check/README.zh-CN.md) | [日本語](https://github.com/checkjs/check/blob/main/packages/%40checkjs/check/README.ja.md) | [Português do Brasil](https://github.com/checkjs/check/blob/main/packages/%40checkjs/check/README.pt-BR.md) | [한국어](https://github.com/checkjs/check/blob/main/packages/%40checkjs/check/README.kr.md) | [Русский](https://github.com/checkjs/check/blob/main/packages/%40checkjs/check/README.ru.md) | [Українська](https://github.com/checkjs/check/blob/main/packages/%40checkjs/check/README.uk.md)
</div>

<br>

**Check** is a performant toolchain for web projects, it aims to provide developer tools to maintain the health of said projects.

**Check is a [fast formatter](./benchmark#formatting)** for _JavaScript_, _TypeScript_, _JSX_, _JSON_, _CSS_ and _GraphQL_ that scores **[97% compatibility with _Prettier_](https://console.algora.io/challenges/prettier)**.

**Check is a [performant linter](https://github.com/checkjs/check/tree/main/benchmark#linting)** for _JavaScript_, _TypeScript_, _JSX_, _JSON_, _CSS_, and _GraphQL_ that features **[more than 340 rules](https://checkjs.dev/linter/javascript/rules/)** from ESLint, typescript-eslint, and [other sources](https://github.com/checkjs/check/discussions/3).
It **outputs detailed and contextualized diagnostics** that help you to improve your code and become a better programmer!

**Check** is designed from the start to be used [interactively within an editor](https://checkjs.dev/guides/editors/first-party-extensions/).
It can format and lint malformed code as you are writing it.

### Installation

```shell
npm install --save-dev --save-exact @checkjs/check
```

### Usage

```shell
# format files
npx @checkjs/check format --write

# lint files and apply the safe fixes
npx @checkjs/check lint --write

# run format, lint, etc. and apply the safe fixes
npx @checkjs/check check --write

# check all files against format, lint, etc. in CI environments
npx @checkjs/check ci
```

If you want to give Check a run without installing it, use the [online playground](https://checkjs.dev/playground/), compiled to WebAssembly.

## Documentation

Check out our [homepage][checkjs] to learn more about Check,
or directly head to the [Getting Started guide][getting-started] to start using Check.

## More about Check

**Check** has sane defaults and it doesn't require configuration.

**Check** aims to support [all main languages][language-support] of modern web development.

**Check** [doesn't require Node.js](https://checkjs.dev/guides/manual-installation/) to function.

**Check** has first-class LSP support, with a sophisticated parser that represents the source text in full fidelity and top-notch error recovery.

**Check** wants to offer a high-quality *Developer Experience*, with descriptive diagnostics and great performance.

**Check** unifies functionalities that have previously been separate tools. Building upon a shared base allows us to provide a cohesive experience for processing code, displaying errors, parallelize work, caching, and configuration.

Read more about our [project philosophy][check-philosophy].

**Check** is [MIT licensed](https://github.com/checkjs/check/tree/main/LICENSE-MIT) or [Apache 2.0 licensed](https://github.com/checkjs/check/tree/main/LICENSE-APACHE) and moderated under the [Contributor Covenant Code of Conduct](https://github.com/checkjs/check/tree/main/CODE_OF_CONDUCT.md).

## Funding

You can fund the project in different ways

### Project sponsorship and funding

You can sponsor or fund the project via [Open collective](https://opencollective.com/check) or [GitHub sponsors](https://github.com/sponsors/checkjs)

Check offers a simple sponsorship program that allows companies to get visibility and recognition among various developers.

Check offers [enterprise support](https://checkjs.dev/enterprise), where Core Contributors can be employed to work on company-focused projects.

## Sponsors

### Gold Sponsors

<table>
  <tbody>
    <tr>
      <td align="center" valign="middle">
        <a href="https://depot.dev/?utm_source=check&utm_medium=readme" target="_blank">
          <picture>
            <source media="(prefers-color-scheme: light)" srcset="https://depot.dev/assets/brand/1693758816/depot-logo-horizontal-on-light@3x.png" />
            <source media="(prefers-color-scheme: dark)" srcset="https://depot.dev/assets/brand/1693758816/depot-logo-horizontal-on-dark@3x.png" />
            <img src="https://depot.dev/assets/brand/1693758816/depot-logo-horizontal-on-light@3x.png" width="400" alt="Depot logo" />
          </picture>
        </a>
      </td>
    </tr>
    <tr>
      <td align="center" valign="middle">
        <a href="https://vercel.com/?utm_source=check&utm_medium=readme" target="_blank">
          <picture>
            <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/checkjs/resources/refs/heads/main/sponsors/vercel-dark.png" />
            <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/checkjs/resources/refs/heads/main/sponsors/vercel-light.png" />
            <img src="https://raw.githubusercontent.com/checkjs/resources/refs/heads/main/sponsors/vercel-dark.png" width="400" alt="Vercel" />
          </picture>
        </a>
      </td>
    </tr>
  </tbody>
</table>

### Silver Sponsors

<table>
  <tbody>
    <tr>
      <td align="center" valign="middle">
        <a href="https://l2beat.com/?utm_source=check&utm_medium=readme" target="_blank"><img src="https://images.opencollective.com/l2beat/c2b2a27/logo/256.png" height="100" alt="L2BEAT logo"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://www.phoenixlabs.dev/?utm_source=check&utm_medium=readme" target="_blank"><img src="https://images.opencollective.com/phoenix-labs/2824ed4/logo/100.png?height=100" height="100" alt="Phoenix Labs logo"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://lokalise.com/?utm_source=check&utm_medium=readme" target="_blank"><img src="https://avatars.githubusercontent.com/u/14294501?s=200&v=4" height="100" alt="Lokalise logo"></a>
      </td>
    </tr>
  </tbody>
</table>

### Bronze Sponsors

<table>
  <tbody>
    <tr>
      <td align="center" valign="middle">
        <a href="https://nanabit.dev/?utm_source=check&utm_medium=readme" target="_blank"><img src="https://images.opencollective.com/nanabit/d15fd98/logo/256.png?height=80" width="80" alt="Nanabit logo"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://vital.io/?utm_source=check&utm_medium=readme" target="_blank"><img src="https://avatars.githubusercontent.com/u/25357309?s=200" width="80" alt="Vital logo"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://coderabbit.ai/?utm_source=check&utm_medium=readme" target="_blank"><img src="https://avatars.githubusercontent.com/u/132028505?s=200&v=4" width="80" alt="CodeRabbit logo"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://forge42.dev/?utm_source=check&utm_medium=readme" target="_blank"><img src="https://avatars.githubusercontent.com/u/161314831?s=200&v=4" width="80" alt="Forge42 logo"></a>
      </td>
      <td align="center" valign="middle">
        <a href="http://rstudio.org/?utm_source=check&utm_medium=readme" target="_blank"><img src="https://avatars.githubusercontent.com/u/513560?s=200&v=4" width="80" alt="RStudio logo"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://pennylane.com/?utm_source=check&utm_medium=readme" target="_blank"><img src="https://avatars.githubusercontent.com/u/57875210?s=200&v=4" width="80" alt="Pennylane logo"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://jetbrains.com/?utm_source=check&utm_medium=readme" target="_blank"><img src="https://resources.jetbrains.com/storage/products/company/brand/logos/jetbrains.png" width="100" alt="JetBrains logo"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://www.egstock.co.jp/?utm_source=check&utm_medium=readme" target="_blank"><img src="https://images.opencollective.com/egstock/b18c836/logo/256.png?height=256" width="80" alt="EGSTOCK, Inc. logo"></a>
      </td>
    </tr>
  </tbody>
</table>

[checkjs]: https://checkjs.dev/
[check-philosophy]: https://checkjs.dev/internals/philosophy/
[language-support]: https://checkjs.dev/internals/language-support/
[getting-started]: https://checkjs.dev/guides/getting-started/
